use crate::{
    ul,
    View
};

use std::{
    os::raw::{
        c_void
    },
};

pub unsafe fn ul_string(stref: &str) -> ul::ULString {
    let cstr = std::ffi::CString::new(
        stref
    ).unwrap();

    ul::ulCreateString(
        cstr.as_ptr()
    )
}

pub unsafe fn unpack_window_resize_cb<F>(closure: &mut F) -> (*mut c_void, unsafe extern "C" fn(*mut c_void, width: u32, height: u32))
    where
        F: FnMut(u32, u32),
{
    extern "C" fn trampoline<F>(data: *mut c_void, width: u32, height: u32)
        where
            F: FnMut(u32, u32),
    {
        let closure: &mut F = unsafe { &mut *(data as *mut F) };
        (*closure)(width, height);
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

// All callbacks that accept take a (view: ULView) argument

pub unsafe fn unpack_closure_view_cb<F>(closure: &mut F) -> (*mut c_void, unsafe extern "C" fn(*mut c_void, View))
    where
        F: FnMut(View),
{
    extern "C" fn trampoline<F>(data: *mut c_void, n: View)
        where
            F: FnMut(View),
    {
        let closure: &mut F = unsafe { &mut *(data as *mut F) };
        (*closure)(n);
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

// JSContextHooks
type ClosureHookCallbackSig = unsafe extern "C" fn(
    ul::JSContextRef,
    ul::JSObjectRef,
    ul::JSObjectRef,
    usize,
    *const ul::JSValueRef,
    *mut ul::JSValueRef
) -> ul::JSValueRef;

pub unsafe fn unpack_closure_hook_cb<F>(closure: &mut F) -> (*mut c_void, ClosureHookCallbackSig)
    where
        F: FnMut(
            ul::JSContextRef,
            ul::JSObjectRef,
            ul::JSObjectRef,
            usize,
            *const ul::JSValueRef,
            *mut ul::JSValueRef,
        ) -> ul::JSValueRef,
{
    unsafe extern "C" fn trampoline<F>(
        ctx: ul::JSContextRef,
        function: ul::JSObjectRef,
        thisObject: ul::JSObjectRef,
        argumentCount: usize,
        arguments: *const ul::JSValueRef,
        exception: *mut ul::JSValueRef,
    ) -> ul::JSValueRef
        where
            F: FnMut(
                ul::JSContextRef,
                ul::JSObjectRef,
                ul::JSObjectRef,
                usize,
                *const ul::JSValueRef,
                *mut ul::JSValueRef,
            ) -> ul::JSValueRef,
    {
        let closure: &mut F = &mut *(ul::JSObjectGetPrivate(function) as *mut F);

        (*closure)(
            ctx,
            function,
            thisObject,
            argumentCount,
            arguments,
            exception
        )
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

static msg_parsing_failed: &'static str = "!parsing failed!";

pub unsafe extern "C" fn log_forward_cb(
    user_data: *mut ::std::os::raw::c_void,
    caller: View,
    source: ul::ULMessageSource,           /* u32 */
    level: ul::ULMessageLevel,             /* u32 */
    message: ul::ULString,                 /* *mut C_String aka *mut u8 */
    line_number: ::std::os::raw::c_uint,    /* u32 */
    column_number: ::std::os::raw::c_uint,  /* u32 */
    source_id: ul::ULString,               /* *mut C_String aka *mut u8 */
) {
    let level = match level {
        ul::ULMessageLevel_kMessageLevel_Log => "log",
        ul::ULMessageLevel_kMessageLevel_Warning => "warning",
        ul::ULMessageLevel_kMessageLevel_Error => "error",
        ul::ULMessageLevel_kMessageLevel_Debug => "debug",
        ul::ULMessageLevel_kMessageLevel_Info => "info",
        _ => "unknown",
    };

    let source = match source {
        ul::ULMessageSource_kMessageSource_XML => "xml",
        ul::ULMessageSource_kMessageSource_JS => "js",
        ul::ULMessageSource_kMessageSource_Network => "network",
        ul::ULMessageSource_kMessageSource_ConsoleAPI => "consoleapi",
        ul::ULMessageSource_kMessageSource_Storage => "storage",
        ul::ULMessageSource_kMessageSource_AppCache => "appcache",
        ul::ULMessageSource_kMessageSource_Rendering => "rendering",
        ul::ULMessageSource_kMessageSource_CSS => "css",
        ul::ULMessageSource_kMessageSource_Security => "security",
        ul::ULMessageSource_kMessageSource_ContentBlocker => "contentblocker",
        ul::ULMessageSource_kMessageSource_Other => "other",
        _ => "unknown",
    };

    let message = match String::from_utf16(std::slice::from_raw_parts_mut(
        ul::ulStringGetData(message),
        ul::ulStringGetLength(message),
    )) {
        Ok(msg) => msg,
        Err(_) => msg_parsing_failed.to_string(),
    };

    let source_id = match String::from_utf16(std::slice::from_raw_parts_mut(
        ul::ulStringGetData(source_id),
        ul::ulStringGetLength(source_id),
    )) {
        Ok(src) => src,
        Err(_) => msg_parsing_failed.to_string(),
    };

    println!(
        "[{}] [{}] {} ({}:{}:{})",
        level, source, message, source_id, line_number, column_number
    );
}
