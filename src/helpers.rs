use crate::helpers_internal::unpack_closure_hook_cb;

pub fn create_js_function<T> (
    view: crate::View,
    name: &'static str,
    mut hook: &mut T
) -> ul_sys::JSObjectRef
    where T: FnMut(
        ul_sys::JSContextRef,
        ul_sys::JSObjectRef,
        ul_sys::JSObjectRef,
        usize,
        *const ul_sys::JSValueRef,
        *mut ul_sys::JSValueRef,
    ) -> ul_sys::JSValueRef
{
    unsafe {
        let (
            hook_closure,
            hook_function
        ) = unpack_closure_hook_cb(&mut hook);

        let classname_str = std::ffi::CString::new(name).unwrap();

        let jsclassdef = ul_sys::JSClassDefinition {
            version: 0,
            attributes: 0,
            className: classname_str.as_ptr(),
            parentClass: std::ptr::null_mut() as ul_sys::JSClassRef,
            callAsFunction: Some(hook_function),
            // need to implement drop!
            //finalize: Some(|| std::mem::drop(jsclass)),
        };

        let jsclass = ul_sys::JSClassCreate(
            &jsclassdef
        );

        let (jsgctx, ..) = getJSContextFromView(view);

        ul_sys::JSObjectMake(
            jsgctx,
            jsclass,
            hook_closure
        )
    }
}

pub fn getJSContextFromView(
    view: crate::View
) -> (ul_sys::JSContextRef, ul_sys::JSObjectRef) {
    unsafe {
        let jsgctx = ul_sys::ulViewLockJSContext(view);
        let jsgctx_object = ul_sys::JSContextGetGlobalObject(jsgctx);

        (jsgctx, jsgctx_object)
    }
}

pub fn set_js_object_property(
    view: crate::View,
    name: &'static str,
    object: ul_sys::JSObjectRef
) {
    unsafe {
        let (jsgctx, jsgctx_object) = getJSContextFromView(view);

        let c_name = std::ffi::CString::new(
            name
        ).unwrap();

        let propertyName = ul_sys::JSStringCreateWithUTF8CString(
            c_name.as_ptr()
        );

        ul_sys::JSObjectSetProperty(
            jsgctx,
            jsgctx_object,
            propertyName,
            object,
            0,
            std::ptr::null_mut() as *mut *const ul_sys::OpaqueJSValue
        );
    }
}

// "window.styla={callbacks:[{render:global_spotfire_hook}]};"

pub fn evaluate_script(
    view: crate::View,
    script: &'static str
) -> ul_sys::JSValueRef {
    unsafe {
        let (jsgctx, jsgctx_object) = getJSContextFromView(view);

        let script_c_str = std::ffi::CString::new(
            script
        ).unwrap();

        ul_sys::JSEvaluateScript(
            jsgctx,
            ul_sys::JSStringCreateWithUTF8CString(
                script_c_str.as_ptr()
            ),
            jsgctx_object,
            std::ptr::null_mut() as *mut ul_sys::OpaqueJSString,
            ul_sys::kJSPropertyAttributeNone as i32,
            std::ptr::null_mut() as *mut *const ul_sys::OpaqueJSValue
        )
    }
}
