#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_variables,
    unused_must_use
)]

#[cfg(feature = "image")]
extern crate image;

#[cfg(feature = "image")]
use image::ImageBuffer;

pub extern crate ul_sys as ul;
pub mod config;
pub mod helpers;

use helpers::{create_js_function, evaluate_script, set_js_object_property};

use std::{fmt::Debug, os::raw::c_void};

mod helpers_internal;
use crate::helpers_internal::unpack_window_resize_cb;
use helpers_internal::{log_forward_cb, unpack_closure_view_cb};

pub type App = ul::ULApp;
pub type Config = config::UltralightConfig;
pub type Monitor = ul::ULMonitor;
pub type Overlay = ul::ULOverlay;
pub type Renderer = ul::ULRenderer;
pub type View = ul::ULView;
pub type Window = ul::ULWindow;

/*
    Current flow

    initialize using
        - None as renderer
            -> initialize empty config, create new renderer
        - Some renderer
            -> initialize view using renderer

    Desired flow

    - Create clear API for instanciating an app from AppCore
      using a new UltralightApp (?) impl that initializes
      all requirements of an AppCore instance:

        - initialize empty config
        - initialize app
        - obtain monitor from app
            - uses macOS internal APIs to resolve "main" monitor,
              which always becomes the currently focused oned
        - create window monitor using
            width,
            height,
            fullscreen-flag,
            configuration bitmap:
                  borderless = 1 << 0,
                  titled = 1 << 1,
                  resizable = 1 << 2,
                  maximizable = 1 << 3,

                  0b0000
                    ^^^^
        - Configure app using created window on monitor
          via ulAppSetWindow
        - Obtain renderer from app via ulAppGetRenderer
*/

/*

    let config = ul::ulCreateConfig(); -> config

    let app = ul::ulCreateApp(config); -> app
    let monitor = ul::ulAppGetMainMonitor(app); -> monitor

    let (width, height): (u32, u32) = (1280, 768);

    let window = ul::ulCreateWindow(
        monitor, width, height, false, 0b0110
    ); -> window

    ul::ulAppSetWindow(app, window); -> void

    let renderer = ul::ulAppGetRenderer(app); -> renderer
*/

pub struct UltralightApp {
    config: Config,
    app: App,
    // check if we really want to store
    // the monitor instance here as
    // in the future we might be able
    // to obtain a non-main monitor
    monitor: Monitor,
    overlay: Option<Overlay>,
    window: Option<Window>,
}

impl UltralightApp {
    pub fn new(config: Option<Config>) -> UltralightApp {
        let ulconfig = match config {
            Some(config) => config,
            None => Config::new(),
        };

        unsafe {
            let app = ul::ulCreateApp(ulconfig.to_ulconfig());
            let monitor = ul::ulAppGetMainMonitor(app);

            UltralightApp {
                config: ulconfig,
                app,
                monitor,
                window: None,
                overlay: None,
            }
        }
    }

    pub fn window(
        &mut self,
        height: u32,
        width: u32,
        fullscreen: bool,
        borderless: bool,
        titled: bool,
        resizable: bool,
        maximizable: bool,
    ) {
        let mut window_flags = 0u32;

        if borderless {
            window_flags ^= 0b0001;
        }

        if titled {
            window_flags ^= 0b0010;
        }

        if resizable {
            window_flags ^= 0b0100;
        }

        if maximizable {
            window_flags ^= 0b1000;
        }

        let window =
            unsafe { ul::ulCreateWindow(self.monitor, height, width, fullscreen, window_flags) };

        unsafe {
            ul::ulAppSetWindow(self.app, window);
        }

        let overlay = unsafe { ul::ulCreateOverlay(window, width, height, 0, 0) };

        self.window = Some(window);
        self.overlay = Some(overlay);
    }

    pub fn get_renderer(&mut self) -> Renderer {
        unsafe { ul::ulAppGetRenderer(self.app) }
    }

    pub fn get_view(&mut self) -> Result<View, ()> {
        if let Some(overlay) = self.overlay {
            unsafe { Ok(ul::ulOverlayGetView(overlay)) }
        } else {
            Err(())
        }
    }

    pub fn set_window_title(&mut self, title: &str) -> Result<(), ()> {
        if let Some(window) = self.window {
            unsafe {
                ul::ulWindowSetTitle(window, std::ffi::CString::new(title).unwrap().as_ptr());
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn resize_overlay(&self, width: u32, height: u32) -> Result<(), ()> {
        if let Some(overlay) = self.overlay {
            unsafe {
                ul::ulOverlayResize(overlay, width, height);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_window_resize_callback<T>(&self, mut cb: T) -> Result<(), ()>
    where
        T: FnMut(u32, u32),
    {
        if let Some(window) = self.window {
            unsafe {
                let (cb_closure, cb_function) = unpack_window_resize_cb(&mut cb);

                ul::ulWindowSetResizeCallback(window, Some(cb_function), cb_closure);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn run(&mut self) {
        unsafe {
            ul::ulAppRun(self.app);
        }
    }
}

pub struct Ultralight {
    config: Config,
    renderer: Renderer,
    view: Option<View>,
}

impl Ultralight {
    pub fn new(config: Option<Config>, renderer: Option<Renderer>) -> Ultralight {
        let ulconfig = match config {
            Some(config) => config,
            None => Config::new(),
        };

        let used_renderer = match renderer {
            Some(renderer) => renderer,
            None => unsafe { ul::ulCreateRenderer(ulconfig.to_ulconfig()) },
        };

        Ultralight {
            config: ulconfig,
            renderer: used_renderer,
            view: None,
        }
    }

    pub fn set_view(&mut self, view: View) {
        unsafe {
            self.view = Some(view);
        }
    }

    pub fn view(&mut self, width: u32, height: u32, transparent: bool) {
        unsafe {
            self.view = Some(ul::ulCreateView(self.renderer, width, height, transparent));
        }
    }

    pub fn load_url(&mut self, url: &'static str) -> Result<(), ()> {
        if let Some(view) = self.view {
            unsafe {
                let url_ulstr = helpers_internal::ul_string(url);

                ul::ulViewLoadURL(view, url_ulstr);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn load_html(&mut self, code: &'static str) -> Result<(), ()> {
        if let Some(view) = self.view {
            unsafe {
                let code_ulstr = helpers_internal::ul_string(code);

                ul::ulViewLoadHTML(view, code_ulstr);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn update(&mut self) {
        unsafe {
            ul::ulUpdate(self.renderer);
        }
    }

    pub fn update_until_loaded(&mut self) -> Result<(), ()> {
        if let Some(view) = self.view {
            unsafe {
                while ul::ulViewIsLoading(view) {
                    ul::ulUpdate(self.renderer);
                }
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn render(&mut self) {
        unsafe {
            ul::ulRender(self.renderer);
        }
    }

    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) -> Result<(), ()> {
        if let Some(view) = self.view {
            unsafe {
                let scrollEvent = ul::ulCreateScrollEvent(
                    ul::ULScrollEventType_kScrollEventType_ScrollByPixel,
                    delta_x,
                    delta_y,
                );

                ul::ulViewFireScrollEvent(view, scrollEvent);

                ul::ulDestroyScrollEvent(scrollEvent);

                Ok(())
            }
        } else {
            Err(())
        }
    }

    pub fn get_scroll_height(&mut self) -> Result<f64, ()> {
        if let Some(view) = self.view {
            unsafe {
                let (jsgctx, _) = helpers::getJSContextFromView(view);

                Ok(ul::JSValueToNumber(
                    jsgctx,
                    self.evaluate_script("document.body.scrollHeight").unwrap(),
                    std::ptr::null_mut(),
                ))
            }
        } else {
            Err(())
        }
    }

    pub fn set_finish_loading_callback<T>(&mut self, mut cb: T) -> Result<(), ()>
    where
        T: FnMut(View),
    {
        if let Some(view) = self.view {
            unsafe {
                let (cb_closure, cb_function) = unpack_closure_view_cb(&mut cb);

                ul::ulViewSetFinishLoadingCallback(view, Some(cb_function), cb_closure);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_dom_ready_callback<T>(&mut self, mut cb: T) -> Result<(), ()>
    where
        T: FnMut(View),
    {
        if let Some(view) = self.view {
            unsafe {
                let (cb_closure, cb_function) = unpack_closure_view_cb(&mut cb);

                ul::ulViewSetDOMReadyCallback(view, Some(cb_function), cb_closure);
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn create_function<T>(
        &mut self,
        name: &'static str,
        hook: &mut T,
    ) -> Result<ul::JSObjectRef, ()>
    where
        T: FnMut(
            ul::JSContextRef,
            ul::JSObjectRef,
            ul::JSObjectRef,
            usize,
            *const ul::JSValueRef,
            *mut ul::JSValueRef,
        ) -> ul::JSValueRef,
    {
        if let Some(view) = self.view {
            Ok(create_js_function(view, name, hook))
        } else {
            Err(())
        }
    }

    pub fn set_js_object_property(
        &mut self,
        name: &'static str,
        object: ul::JSObjectRef,
    ) -> Result<(), ()> {
        if let Some(view) = self.view {
            set_js_object_property(view, name, object);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn evaluate_script(&mut self, script: &'static str) -> Result<ul::JSValueRef, ()> {
        if let Some(view) = self.view {
            Ok(evaluate_script(view, script))
        } else {
            Err(())
        }
    }

    pub fn get_raw_pixels(&mut self) -> Result<Vec<u8>, ()> {
        if let Some(view) = self.view {
            unsafe {
                let bitmap_obj = ul::ulViewGetBitmap(view);

                let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
                let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

                let bitmap_raw = std::slice::from_raw_parts_mut(bitmap as *mut u8, bitmap_size);

                ul::ulBitmapUnlockPixels(bitmap_obj);

                Ok(bitmap_raw.to_vec())
            }
        } else {
            Err(())
        }
    }

    pub fn write_png_to_file(&mut self, file_name: &'static str) -> Result<bool, ()> {
        if let Some(view) = self.view {
            unsafe {
                let bitmap_obj = ul::ulViewGetBitmap(view);

                let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
                let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

                let bitmap_raw = std::slice::from_raw_parts_mut(bitmap as *mut u8, bitmap_size);

                let fn_c_str = std::ffi::CString::new(file_name).unwrap();

                Ok(ul::ulBitmapWritePNG(bitmap_obj, fn_c_str.as_ptr()))
            }
        } else {
            Err(())
        }
    }

    pub fn is_loading(&self) -> bool {
        match self.view {
            Some(view) => unsafe { ul::ulViewIsLoading(view) },
            None => false,
        }
    }

    pub fn log_to_stdout(&mut self) -> Result<(), ()> {
        if let Some(view) = self.view {
            unsafe {
                ul::ulViewSetAddConsoleMessageCallback(
                    view,
                    Some(log_forward_cb),
                    std::ptr::null_mut() as *mut c_void,
                );
            }

            Ok(())
        } else {
            Err(())
        }
    }
}
