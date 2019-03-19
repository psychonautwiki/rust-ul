#![feature(try_trait,unboxed_closures)]
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
pub mod helpers;
pub mod config;

use helpers::{
    evaluate_script,
    set_js_object_property,
    create_js_function,
};

use std::{
    option::NoneError,
    os::raw::c_void,
};

mod helpers_internal;
use helpers_internal::{
    log_forward_cb,
    unpack_closure_view_cb,
};

pub type Renderer = ul::ULRenderer;
pub type View = ul::ULView;
pub type Config = config::UltralightConfig;

pub struct Ultralight {
    config: Config,
    renderer: Renderer,
    view: Option<View>,
}

impl Ultralight {
    pub fn new(config: Option<Config>, renderer: Option<Renderer>) -> Ultralight {
        let ulconfig = match config {
            Some(config) => config,
            None => Config::new()
        };

        let used_renderer = match renderer {
            Some(renderer) => renderer,
            None => {
                unsafe {
                    ul::ulCreateRenderer(ulconfig.to_ulconfig())
                }
            }
        };

        Ultralight {
            config: ulconfig,
            renderer: used_renderer,
            view: None
        }
    }

    pub fn view(&mut self, width: u32, height: u32, transparent: bool) {
        unsafe {
            self.view = Some(ul::ulCreateView(self.renderer, width, height, transparent));
        }
    }

    pub fn load_url(&mut self, url: &'static str) -> Result<(), NoneError> {
        unsafe {
            let url_str = std::ffi::CString::new(
                url
            ).unwrap();

            let url = ul::ulCreateString(
                url_str.as_ptr()
            );

            ul::ulViewLoadURL(self.view?, url);
        }

        Ok(())
    }

    pub fn load_html(&mut self, code: &'static str) -> Result<(), NoneError> {
        unsafe {
            let code_str = std::ffi::CString::new(
                code
            ).unwrap();

            let code = ul::ulCreateString(
                code_str.as_ptr()
            );

            ul::ulViewLoadHTML(self.view?, code);
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
            while ul::ulViewIsLoading(self.view?) {
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
                delta_y
            );

            ul::ulViewFireScrollEvent(self.view?, scrollEvent);

            ul::ulDestroyScrollEvent(scrollEvent);

            Ok(())
        }
    }

    pub fn get_scroll_height(&mut self) -> Result<f64, NoneError> {
        unsafe {
            let (jsgctx, _) = helpers::getJSContextFromView(self.view?);

            Ok(ul::JSValueToNumber(
                jsgctx,
                self.evaluate_script("document.body.scrollHeight").unwrap(),
                std::ptr::null_mut()
            ))
        }
    }

    pub fn set_finish_loading_callback<T>(&mut self, mut cb: T) -> Result<(), NoneError>
        where T: FnMut(View)
    {
        let view = self.view?;

        unsafe {
            let (
                cb_closure,
                cb_function
            ) = unpack_closure_view_cb(&mut cb);

            ul::ulViewSetFinishLoadingCallback(
                view,
                Some(cb_function),
                cb_closure
            );
        }

        Ok(())
    }

    pub fn set_dom_ready_callback<T>(&mut self, mut cb: T) -> Result<(), NoneError>
        where T: FnMut(View)
    {
        let view = self.view?;

        unsafe {
            let (
                cb_closure,
                cb_function
            ) = unpack_closure_view_cb(&mut cb);

            ul::ulViewSetDOMReadyCallback(
                view,
                Some(cb_function),
                cb_closure
            );
        }

        Ok(())
    }

    pub fn create_function<T>(
        &mut self,
        name: &'static str,
        hook: &mut T
    ) -> Result<ul::JSObjectRef, NoneError>
        where T: FnMut(
            ul::JSContextRef,
            ul::JSObjectRef,
            ul::JSObjectRef,
            usize,
            *const ul::JSValueRef,
            *mut ul::JSValueRef,
        ) -> ul::JSValueRef
    {
        Ok(
            create_js_function(
                self.view?,
                name,
                hook
            )
        )
    }

    pub fn set_js_object_property(
        &mut self,
        name: &'static str,
        object: ul::JSObjectRef
    ) -> Result<(), NoneError> {
        set_js_object_property(
            self.view?,
            name,
            object
        );

        Ok(())
    }

    pub fn evaluate_script(
        &mut self,
        script: &'static str,
    ) -> Result<ul::JSValueRef, NoneError> {
        Ok(evaluate_script(self.view?, script))
    }

    pub fn get_raw_pixels(&mut self) -> Result<Vec<u8>, NoneError> {
        unsafe {
            let bitmap_obj = ul::ulViewGetBitmap( self.view? );

            let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
            let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

            let bitmap_raw = std::slice::from_raw_parts_mut(
                bitmap as *mut u8,
                bitmap_size,
            );

            ul::ulBitmapUnlockPixels(bitmap_obj);

            Ok(bitmap_raw.to_vec())
        }
    }

    pub fn write_png_to_file(
        &mut self,
        file_name: &'static str,
    ) -> Result<bool, NoneError> {
        unsafe {
            let bitmap_obj = ul::ulViewGetBitmap( self.view? );

            let bitmap = ul::ulBitmapLockPixels(bitmap_obj);
            let bitmap_size = ul::ulBitmapGetSize(bitmap_obj);

            let bitmap_raw = std::slice::from_raw_parts_mut(
                bitmap as *mut u8,
                bitmap_size,
            );

            let fn_c_str = std::ffi::CString::new(file_name).unwrap();

            Ok(
                ul::ulBitmapWritePNG(
                    bitmap_obj,
                    fn_c_str.as_ptr()
                )
            )
        }
    }

    pub fn is_loading(&self) -> bool {
        match self.view {
            Some(view) => unsafe {
                ul::ulViewIsLoading(view)
            },
            None => false
        }
    }

    pub fn log_to_stdout(&mut self) -> Result<(), NoneError> {
        unsafe {
            ul::ulViewSetAddConsoleMessageCallback(
                self.view?,
                Some(log_forward_cb),
                std::ptr::null_mut() as *mut c_void
            );
        }

        Ok(())
    }
}
