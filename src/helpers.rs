use crate::ul;
use crate::helpers_internal::unpack_closure_hook_cb;

pub fn create_js_function<T> (
    view: crate::View,
    name: &'static str,
    mut hook: &mut T
) -> ul::JSObjectRef
    where T: FnMut(
        ul::JSContextRef,
        ul::JSObjectRef,
        ul::JSObjectRef,
        usize,
        *const ul::JSValueRef,
        *mut ul::JSValueRef,
    ) -> ul::JSValueRef
{
    unsafe {
        let (
            hook_closure,
            hook_function
        ) = unpack_closure_hook_cb(&mut hook);

        let classname_str = std::ffi::CString::new(name).unwrap();

        let jsclassdef = ul::JSClassDefinition {
            version: 0,
            attributes: 0,
            className: classname_str.as_ptr(),
            parentClass: std::ptr::null_mut() as ul::JSClassRef,
            staticValues: std::ptr::null() as *const ul::JSStaticValue,
            staticFunctions: std::ptr::null() as *const ul::JSStaticFunction,
            initialize: None,
            hasProperty: None,
            getProperty: None,
            setProperty: None,
            deleteProperty: None,
            getPropertyNames: None,
            callAsConstructor: None,
            hasInstance: None,
            convertToType: None,
            finalize: None,
            callAsFunction: Some(hook_function),
            // need to implement drop!
            //finalize: Some(|| std::mem::drop(jsclass)),
        };

        let jsclass = ul::JSClassCreate(
            &jsclassdef
        );

        let (jsgctx, ..) = getJSContextFromView(view);

        ul::JSObjectMake(
            jsgctx,
            jsclass,
            hook_closure
        )
    }
}

pub fn getJSContextFromView(
    view: crate::View
) -> (ul::JSContextRef, ul::JSObjectRef) {
    unsafe {
        let jsgctx = ul::ulViewGetJSContext(view);
        let jsgctx_object = ul::JSContextGetGlobalObject(jsgctx);

        (jsgctx, jsgctx_object)
    }
}

pub fn set_js_object_property(
    view: crate::View,
    name: &'static str,
    object: ul::JSObjectRef
) {
    unsafe {
        let (jsgctx, jsgctx_object) = getJSContextFromView(view);

        let c_name = std::ffi::CString::new(
            name
        ).unwrap();

        let propertyName = ul::JSStringCreateWithUTF8CString(
            c_name.as_ptr()
        );

        ul::JSObjectSetProperty(
            jsgctx,
            jsgctx_object,
            propertyName,
            object,
            0,
            std::ptr::null_mut() as *mut *const ul::OpaqueJSValue
        );
    }
}

// "window.styla={callbacks:[{render:global_spotfire_hook}]};"

pub fn evaluate_script(
    view: crate::View,
    script: &'static str
) -> ul::JSValueRef {
    unsafe {
        let (jsgctx, jsgctx_object) = getJSContextFromView(view);

        let script_c_str = std::ffi::CString::new(
            script
        ).unwrap();

        ul::JSEvaluateScript(
            jsgctx,
            ul::JSStringCreateWithUTF8CString(
                script_c_str.as_ptr()
            ),
            jsgctx_object,
            std::ptr::null_mut() as *mut ul::OpaqueJSString,
            ul::kJSPropertyAttributeNone as i32,
            std::ptr::null_mut() as *mut *const ul::OpaqueJSValue
        )
    }
}
