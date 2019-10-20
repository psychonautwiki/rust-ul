// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSClass, JSContext, JSContextGroup};
use std::ptr;

impl JSContextGroup {
    /// Creates a JavaScript context group.
    pub fn new() -> Self {
        JSContextGroup::default()
    }

    /// Creates a global JavaScript execution context in this context
    /// group.
    ///
    /// This allocates a global object and populates it with all the
    /// built-in JavaScript objects, such as `Object`, `Function`,
    /// `String`, and `Array`.
    ///
    /// The created global context retains this group.
    pub fn new_context(&self) -> JSContext {
        JSContext {
            raw: unsafe { ul_sys::JSGlobalContextCreateInGroup(self.raw, ptr::null_mut()) },
        }
    }

    /// Creates a global JavaScript execution context in this context
    /// group.
    ///
    /// This allocates a global object and populates it with all the
    /// built-in JavaScript objects, such as `Object`, `Function`,
    /// `String`, and `Array`.
    ///
    /// The created global context retains this group.
    ///
    /// * `global_object_class`: The class to use when creating the global
    ///   object.
    pub fn new_context_with_class(&self, global_object_class: &JSClass) -> JSContext {
        JSContext {
            raw: unsafe { ul_sys::JSGlobalContextCreateInGroup(self.raw, global_object_class.raw) },
        }
    }
}

impl Default for JSContextGroup {
    /// Creates a JavaScript context group.
    fn default() -> Self {
        JSContextGroup {
            raw: unsafe { ul_sys::JSContextGroupCreate() },
        }
    }
}

impl Drop for JSContextGroup {
    fn drop(&mut self) {
        unsafe { ul_sys::JSContextGroupRelease(self.raw) }
    }
}
