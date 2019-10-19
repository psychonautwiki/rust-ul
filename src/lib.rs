#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_variables,
    unused_must_use
)]

pub extern crate ul_sys as ul;

#[macro_use]
mod config_macros;

pub mod config;
pub mod settings;

pub mod helpers;

use helpers::{create_js_function, evaluate_script, set_js_object_property};

use std::os::raw::c_void;

mod helpers_internal;
use crate::helpers_internal::{unpack_window_resize_cb, unpack_window_close_cb};
use helpers_internal::{log_forward_cb, unpack_closure_view_cb};

mod cursor;

use std::ops::DerefMut;
use std::marker::PhantomData;

pub type App = ul::ULApp;
pub type Config = config::UltralightConfig;
pub type Settings = settings::UltralightSettings;
pub type Monitor = ul::ULMonitor;
pub type Overlay = ul::ULOverlay;
pub type Renderer = ul::ULRenderer;
pub type View = ul::ULView;
pub type Window = ul::ULWindow;

pub type Cursor = cursor::Cursor;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NoneError {}

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

pub struct UltralightApp<'a> {
    config: Config,
    settings: Settings,
    app: App,
    // check if we really want to store
    // the monitor instance here as
    // in the future we might be able
    // to obtain a non-main monitor
    monitor: Monitor,
    overlay: Option<Overlay>,
    window: Option<Window>,

    phantom: PhantomData<&'a ()>,
}

impl<'a> UltralightApp<'a> {
    pub fn new(settings: Option<Settings>, config: Option<Config>) -> UltralightApp<'a> {
        let ulconfig = match config {
            Some(config) => config,
            None => Config::new(),
        };

        let ulsettings = match settings {
            Some(settings) => settings,
            None => Settings::new(),
        };

        unsafe {
            let app = ul::ulCreateApp(
                ulsettings.to_ulsettings(),
                ulconfig.to_ulconfig(),
            );

            let monitor = ul::ulAppGetMainMonitor(app);

            UltralightApp {
                config: ulconfig,
                settings: ulsettings,
                app,
                monitor,
                window: None,
                overlay: None,
                phantom: PhantomData,
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

    pub fn run(&mut self) {
        unsafe {
            ul::ulAppRun(self.app);
        }
    }
}

pub trait UltralightAppOverlay {
    fn overlay_get_view(&mut self) -> Result<View, NoneError>;

    fn overlay_get_height(&mut self) -> Result<u32, NoneError>;
    fn overlay_get_width(&mut self) -> Result<u32, NoneError>;

    fn overlay_get_x(&mut self) -> Result<i32, NoneError>;
    fn overlay_get_y(&mut self) -> Result<i32, NoneError>;

    fn overlay_focus(&mut self) -> Result<(), NoneError>;
    fn overlay_unfocus(&mut self) -> Result<(), NoneError>;
    fn overlay_has_focus(&mut self) -> Result<bool, NoneError>;

    fn overlay_hide(&mut self) -> Result<(), NoneError>;
    fn overlay_is_hidden(&mut self) -> Result<bool, NoneError>;

    fn overlay_move_to(&self, x: i32, y: i32) -> Result<(), NoneError>;
    fn overlay_resize(&self, width: u32, height: u32) -> Result<(), NoneError>;
}

impl<'a> UltralightAppOverlay for UltralightApp<'a> {
    fn overlay_get_view(&mut self) -> Result<View, NoneError> {
        unsafe { Ok(ul::ulOverlayGetView(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_get_height(&mut self) -> Result<u32, NoneError> {
        unsafe { Ok(ul::ulOverlayGetHeight(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_get_width(&mut self) -> Result<u32, NoneError> {
        unsafe { Ok(ul::ulOverlayGetWidth(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_get_x(&mut self) -> Result<i32, NoneError> {
        unsafe { Ok(ul::ulOverlayGetX(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_get_y(&mut self) -> Result<i32, NoneError> {
        unsafe { Ok(ul::ulOverlayGetY(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_focus(&mut self) -> Result<(), NoneError> {
        unsafe { Ok(ul::ulOverlayFocus(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_unfocus(&mut self) -> Result<(), NoneError> {
        unsafe { Ok(ul::ulOverlayUnfocus(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_has_focus(&mut self) -> Result<bool, NoneError> {
        unsafe { Ok(ul::ulOverlayHasFocus(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_hide(&mut self) -> Result<(), NoneError> {
        unsafe { Ok(ul::ulOverlayHide(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_is_hidden(&mut self) -> Result<bool, NoneError> {
        unsafe { Ok(ul::ulOverlayIsHidden(self.overlay.ok_or(NoneError {})?)) }
    }

    fn overlay_move_to(&self, x: i32, y: i32) -> Result<(), NoneError> {
        unsafe {
            ul::ulOverlayMoveTo(self.overlay.ok_or(NoneError {})?, x, y);
        }

        Ok(())
    }

    fn overlay_resize(&self, width: u32, height: u32) -> Result<(), NoneError> {
        unsafe {
            ul::ulOverlayResize(self.overlay.ok_or(NoneError {})?, width, height);
        }

        Ok(())
    }
}

pub trait UltralightAppWindow {
    fn window_close(&self) -> Result<(), NoneError>;

    fn window_device_to_pixel(&self, val: i32) -> Result<i32, NoneError>;
    fn window_pixels_to_device(&self, val: i32) -> Result<i32, NoneError>;

    fn window_get_height(&self) -> Result<u32, NoneError>;
    fn window_get_width(&self) -> Result<u32, NoneError>;
    fn window_get_scale(&self) -> Result<f64, NoneError>;

    fn window_set_title(&mut self, title: &str) -> Result<(), NoneError>;
    fn window_set_cursor(&mut self, cursor: Cursor) -> Result<(), NoneError>;
}

impl<'a> UltralightAppWindow for UltralightApp<'a> {
    fn window_close(&self) -> Result<(), NoneError> {
        unsafe { Ok(ul::ulWindowClose(self.window.ok_or(NoneError {})?)) }
    }

    fn window_device_to_pixel(&self, val: i32) -> Result<i32, NoneError> {
        unsafe { Ok(ul::ulWindowDeviceToPixel(self.window.ok_or(NoneError {})?, val)) }
    }

    fn window_pixels_to_device(&self, val: i32) -> Result<i32, NoneError> {
        unsafe { Ok(ul::ulWindowPixelsToDevice(self.window.ok_or(NoneError {})?, val)) }
    }

    fn window_get_height(&self) -> Result<u32, NoneError> {
        unsafe { Ok(ul::ulWindowGetHeight(self.window.ok_or(NoneError {})?)) }
    }

    fn window_get_width(&self) -> Result<u32, NoneError> {
        unsafe { Ok(ul::ulWindowGetWidth(self.window.ok_or(NoneError {})?)) }
    }

    fn window_get_scale(&self) -> Result<f64, NoneError> {
        unsafe { Ok(ul::ulWindowGetScale(self.window.ok_or(NoneError {})?)) }
    }

    fn window_set_title(&mut self, title: &str) -> Result<(), NoneError> {
        unsafe {
            ul::ulWindowSetTitle(
                self.window.ok_or(NoneError {})?,
                std::ffi::CString::new(title).unwrap().as_ptr(),
            );
        }

        Ok(())
    }

    fn window_set_cursor(&mut self, cursor: Cursor) -> Result<(), NoneError> {
        unsafe {
            Ok(ul::ulWindowSetCursor(
                self.window.ok_or(NoneError {})?,
                cursor as u32
            ))
        }
    }
}

pub trait UltralightAppWindowCallbacks<'a> {
    fn window_set_close_callback<T>(&self, cb: &'a mut T) -> Result<(), NoneError>
        where T: FnMut();

    fn window_set_resize_callback<T>(&self, cb: &'a mut T) -> Result<(), NoneError>
        where T: FnMut(u32, u32);
}

impl<'a> UltralightAppWindowCallbacks<'a> for UltralightApp<'a> {
    fn window_set_close_callback<T>(&self, cb: &'a mut T) -> Result<(), NoneError>
        where T: FnMut(),
    {
        unsafe {
            let (cb_closure, cb_function) = unpack_window_close_cb(cb);

            Ok(ul::ulWindowSetCloseCallback(
                self.window.ok_or(NoneError {})?,
                Some(cb_function),
                cb_closure,
            ))
        }
    }

    fn window_set_resize_callback<T>(&self, cb: &'a mut T) -> Result<(), NoneError>
        where T: FnMut(u32, u32),
    {
        unsafe {
            let (cb_closure, cb_function) = unpack_window_resize_cb(cb);

            Ok(ul::ulWindowSetResizeCallback(
                self.window.ok_or(NoneError {})?,
                Some(cb_function),
                cb_closure,
            ))
        }
    }
}

pub struct Ultralight<'a> {
    config: Config,
    renderer: Renderer,
    view: Option<View>,

    phantom: PhantomData<&'a ()>,
}

impl<'a> Ultralight<'a> {
    pub fn new(config: Option<Config>, renderer: Option<Renderer>) -> Ultralight<'a> {
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

            phantom: PhantomData,
        }
    }

    pub fn app(&mut self, app: &mut UltralightApp) -> Result<(), NoneError> {
        self.view = Some(app.overlay_get_view()?);

        Ok(())
    }

    pub fn set_view(&mut self, view: View) {
        self.view = Some(view);
    }

    pub fn view(&mut self, width: u32, height: u32, transparent: bool) {
        unsafe {
            self.view = Some(ul::ulCreateView(self.renderer, width, height, transparent));
        }
    }

    pub fn load_url(&mut self, url: &'static str) -> Result<(), NoneError> {
        unsafe {
            let url_ulstr = helpers_internal::ul_string(url);

            ul::ulViewLoadURL(self.view.ok_or(NoneError {})?, url_ulstr);
        }

        Ok(())
    }

    pub fn load_html(&mut self, code: &'static str) -> Result<(), NoneError> {
        unsafe {
            let code_ulstr = helpers_internal::ul_string(code);

            ul::ulViewLoadHTML(self.view.ok_or(NoneError {})?, code_ulstr);
        }

        Ok(())
    }

    pub fn update(&mut self) {
        unsafe {
            ul::ulUpdate(self.renderer);
        }
    }

    pub fn update_until_loaded(&mut self) -> Result<(), NoneError> {
        unsafe {
            while ul::ulViewIsLoading(self.view.ok_or(NoneError {})?) {
                ul::ulUpdate(self.renderer);
            }
        }

        Ok(())
    }

    pub fn render(&mut self) {
        unsafe {
            ul::ulRender(self.renderer);
        }
    }

    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) -> Result<(), NoneError> {
        unsafe {
            let scrollEvent = ul::ulCreateScrollEvent(
                ul::ULScrollEventType_kScrollEventType_ScrollByPixel,
                delta_x,
                delta_y,
            );

            ul::ulViewFireScrollEvent(self.view.ok_or(NoneError {})?, scrollEvent);

            ul::ulDestroyScrollEvent(scrollEvent);

            Ok(())
        }
    }

    pub fn get_scroll_height(&mut self) -> Result<f64, NoneError> {
        unsafe {
            let (jsgctx, _) = helpers::getJSContextFromView(self.view.ok_or(NoneError {})?);

            Ok(ul::JSValueToNumber(
                jsgctx,
                self.evaluate_script("document.body.scrollHeight").unwrap(),
                std::ptr::null_mut(),
            ))
        }
    }

    pub fn set_finish_loading_callback<T>(&mut self, cb: &'a mut T) -> Result<(), NoneError>
    where
        T: FnMut(View),
    {
        let view = self.view.ok_or(NoneError {})?;

        unsafe {
            let (cb_closure, cb_function) = unpack_closure_view_cb(cb);

            ul::ulViewSetFinishLoadingCallback(view, Some(cb_function), cb_closure);
        }

        Ok(())
    }

    pub fn set_dom_ready_callback<T>(&mut self, cb: &'a mut T) -> Result<(), NoneError>
    where
        T: FnMut(View),
    {
        let view = self.view.ok_or(NoneError {})?;

        unsafe {
            let (cb_closure, cb_function) = unpack_closure_view_cb(cb);

            ul::ulViewSetDOMReadyCallback(view, Some(cb_function), cb_closure);
        }

        Ok(())
    }

    pub fn create_function<T>(
        &mut self,
        name: &'static str,
        hook: &'a mut T,
    ) -> Result<ul::JSObjectRef, NoneError>
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
        Ok(create_js_function(
            self.view.ok_or(NoneError {})?,
            name,
            hook,
        ))
    }

    pub fn set_js_object_property(
        &mut self,
        name: &'static str,
        object: ul::JSObjectRef,
    ) -> Result<(), NoneError> {
        set_js_object_property(self.view.ok_or(NoneError {})?, name, object);

        Ok(())
    }

    pub fn evaluate_script(&mut self, script: &'static str) -> Result<ul::JSValueRef, NoneError> {
        Ok(evaluate_script(self.view.ok_or(NoneError {})?, script))
    }

    pub fn get_raw_pixels(&mut self) -> Result<Vec<u8>, NoneError> {
        unsafe {
            let bitmap_obj = ul::ulViewGetBitmap(self.view.ok_or(NoneError {})?);

            let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
            let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

            let bitmap_raw = std::slice::from_raw_parts_mut(bitmap as *mut u8, bitmap_size);

            ul::ulBitmapUnlockPixels(bitmap_obj);

            Ok(bitmap_raw.to_vec())
        }
    }

    pub fn write_png_to_file(&mut self, file_name: &'static str) -> Result<bool, NoneError> {
        unsafe {
            let bitmap_obj = ul::ulViewGetBitmap(self.view.ok_or(NoneError {})?);

            let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
            let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

            let bitmap_raw = std::slice::from_raw_parts_mut(bitmap as *mut u8, bitmap_size);

            let fn_c_str = std::ffi::CString::new(file_name).unwrap();

            Ok(ul::ulBitmapWritePNG(bitmap_obj, fn_c_str.as_ptr()))
        }
    }

    pub fn is_loading(&self) -> bool {
        match self.view {
            Some(view) => unsafe { ul::ulViewIsLoading(view) },
            None => false,
        }
    }

    pub fn log_to_stdout(&mut self) -> Result<(), NoneError> {
        unsafe {
            ul::ulViewSetAddConsoleMessageCallback(
                self.view.ok_or(NoneError {})?,
                Some(log_forward_cb),
                std::ptr::null_mut() as *mut c_void,
            );
        }

        Ok(())
    }
}
