// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSObject, JSString, JSValue};
use std::ops::Deref;
use std::ptr;

impl JSObject {
    /// Gets an iterator over the names of an object's enumerable properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// let names: Vec<String> = o.property_names()
    ///                           .map(|s| s.to_string())
    ///                           .collect();
    /// assert_eq!(names, vec!["id"]);
    /// ```
    pub fn property_names(&self) -> JSObjectPropertyNameIter {
        JSObjectPropertyNameIter {
            raw: unsafe { ul_sys::JSObjectCopyPropertyNames(self.value.ctx, self.raw) },
            idx: 0,
        }
    }

    /// Tests whether an object has a given property.
    ///
    /// * `name`: A value that can be converted to a [`JSString`] containing
    ///   the property's name.
    ///
    /// Returns `true` if the object has a property whose name matches
    /// `name`, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// assert!(o.has_property("id"));
    /// ```
    ///
    /// [`JSString`]: struct.JSString.html
    pub fn has_property<S>(&self, name: S) -> bool
        where
            S: Into<JSString>,
    {
        unsafe { ul_sys::JSObjectHasProperty(self.value.ctx, self.raw, name.into().raw) }
    }

    /// Gets a property from an object.
    ///
    /// * `name`: A value that can be converted to a [`JSString`] containing
    ///   the property's name.
    ///
    /// Returns the property's value if object has the property, otherwise
    /// the undefined value.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// let n = o.get_property("id");
    /// assert!(n.is_number());
    /// // Remember that this will be an f64 now!
    /// assert_eq!(n.as_number().expect("number"), 123.0);
    /// ```
    ///
    /// [`JSString`]: struct.JSString.html
    pub fn get_property<S>(&self, name: S) -> JSValue
        where
            S: Into<JSString>,
    {
        let mut e: ul_sys::JSValueRef = ptr::null_mut();
        let v =
            unsafe { ul_sys::JSObjectGetProperty(self.value.ctx, self.raw, name.into().raw, &mut e) };
        JSValue {
            raw: v,
            ctx: self.value.ctx,
        }
    }

    /// Gets a property from an object by numeric index.
    ///
    /// * `index`: An integer value that is the property's name.
    ///
    /// Returns the property's value if object has the property,
    /// otherwise the undefined value.
    ///
    /// Calling `get_property_at_index` is equivalent to calling
    /// `get_property` with a string containing `index`,
    /// but `get_property_at_index` provides optimized access to
    /// numeric properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "[3, true, \"abc\"]").expect("valid array");
    /// let o = v.as_object().expect("object");
    ///
    /// let n = o.get_property_at_index(0).as_number().expect("number");
    /// let b = o.get_property_at_index(1).as_boolean();
    /// let s = o.get_property_at_index(2).as_string().expect("string");
    ///
    /// assert_eq!(n, 3.0);
    /// assert_eq!(b, true);
    /// assert_eq!(s, "abc");
    /// ```
    ///
    /// This also works with objects when the keys are strings of numeric indexes:
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"a\": 3, \"1\": true, \"2\": \"abc\"}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// // There is no property "0", so this will be `undefined`:
    /// assert!(o.get_property_at_index(0).is_undefined());
    /// assert_eq!(o.get_property_at_index(1).as_boolean(), true);
    /// assert_eq!(o.get_property_at_index(2).as_string().expect("string"), "abc");
    /// ```
    pub fn get_property_at_index(&self, index: u32) -> JSValue {
        let mut e: ul_sys::JSValueRef = ptr::null_mut();
        let v = unsafe { ul_sys::JSObjectGetPropertyAtIndex(self.value.ctx, self.raw, index, &mut e) };
        JSValue {
            raw: v,
            ctx: self.value.ctx,
        }
    }
}

/// A `JSObject` can be dereferenced to return the underlying `JSValue`.
///
/// This lets a `JSObject` instance be used where a `JSValue` instance is
/// expected.
impl Deref for JSObject {
    type Target = JSValue;

    fn deref(&self) -> &JSValue {
        &self.value
    }
}

pub struct JSObjectPropertyNameIter {
    raw: ul_sys::JSPropertyNameArrayRef,
    idx: usize,
}

impl Iterator for JSObjectPropertyNameIter {
    type Item = JSString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < unsafe { ul_sys::JSPropertyNameArrayGetCount(self.raw) } {
            let name = unsafe { ul_sys::JSPropertyNameArrayGetNameAtIndex(self.raw, self.idx) };
            self.idx += 1;
            Some(JSString { raw: name })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { ul_sys::JSPropertyNameArrayGetCount(self.raw) };
        (sz - self.idx, Some(sz))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{JSContext, JSValue};

    #[test]
    fn can_has_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.has_property("id"));
        assert!(o.has_property("no-such-value") == false);
    }

    #[test]
    fn can_get_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property("id").is_number());
        assert!(o.get_property("no-such-value").is_undefined());
    }

    #[test]
    fn can_get_property_at_index() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "[3, true, \"abc\"]").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property_at_index(0).is_number());
        assert!(o.get_property_at_index(1).is_boolean());
        assert!(o.get_property_at_index(2).is_string());
        assert!(o.get_property_at_index(5).is_undefined());
    }

    #[test]
    fn can_get_property_names() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        let names = o.property_names().collect::<Vec<_>>();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "id");
    }

    #[test]
    fn can_use_as_jsvalue_via_deref() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(v.is_object());
        assert!(o.is_object());
    }
}
