// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSClass, JSContext, JSContextGroup, JSString};
use std::ptr;

impl JSContext {
    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    pub fn new() -> Self {
        JSContext::default()
    }

    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    ///
    /// * `global_object_class`: The class to use when creating the global
    ///   object.
    pub fn new_with_class(global_object_class: &JSClass) -> Self {
        JSContext {
            raw: unsafe { ul_sys::JSGlobalContextCreate(global_object_class.raw) },
        }
    }

    /// Gets the context group to which a JavaScript execution context belongs.
    pub fn group(&self) -> JSContextGroup {
        let g = unsafe { ul_sys::JSContextGetGroup(self.raw) };
        unsafe {
            ul_sys::JSContextGroupRetain(g);
        };
        JSContextGroup { raw: g }
    }

    /// Gets a copy of the name of a context.
    ///
    /// A `JSContext`'s name is exposed for remote debugging
    /// to make it easier to identify the context you would like to
    /// attach to.
    ///
    /// Returns the name for this context, if there is one.
    ///
    /// ```
    /// # use javascriptcore::JSContext;
    /// let ctx = JSContext::new();
    ///
    /// // By default, a context has no name.
    /// assert!(ctx.name().is_none());
    /// ```
    pub fn name(&self) -> Option<JSString> {
        let r = unsafe { ul_sys::JSGlobalContextCopyName(self.raw) };
        if r.is_null() {
            None
        } else {
            Some(JSString { raw: r })
        }
    }

    /// Sets the remote debugging name for a context.
    ///
    /// * `name`: The remote debugging name to set.
    ///
    /// ```
    /// # use javascriptcore::JSContext;
    /// let ctx = JSContext::new();
    ///
    /// ctx.set_name("test thread");
    /// assert_eq!(ctx.name().unwrap(), "test thread");
    /// ```
    pub fn set_name<S: Into<JSString>>(&self, name: S) {
        unsafe { ul_sys::JSGlobalContextSetName(self.raw, name.into().raw) }
    }
}

impl Default for JSContext {
    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    fn default() -> Self {
        JSContext {
            raw: unsafe { ul_sys::JSGlobalContextCreate(ptr::null_mut()) },
        }
    }
}

impl Drop for JSContext {
    fn drop(&mut self) {
        unsafe { ul_sys::JSGlobalContextRelease(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::JSContext;

    #[test]
    fn context_group() {
        let ctx = JSContext::new();
        let _g = ctx.group();
        // Nothing to do with g now...
    }

    #[test]
    fn context_names() {
        let ctx = JSContext::new();
        assert!(ctx.name().is_none());

        ctx.set_name("test thread");
        assert_eq!(ctx.name().unwrap(), "test thread");
    }
}
