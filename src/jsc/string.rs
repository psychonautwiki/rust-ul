// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::JSString;
use std::ffi::CString;
use std::fmt;

impl JSString {
    /// Convert this `JSString` to a `String`.
    pub fn to_string(&self) -> String {
        unsafe {
            let max_size = ul_sys::JSStringGetMaximumUTF8CStringSize(self.raw);
            let mut buffer: Vec<u8> = Vec::with_capacity(max_size as usize);
            let actual_size = ul_sys::JSStringGetUTF8CString(
                self.raw,
                buffer.as_mut_ptr() as *mut ::std::os::raw::c_char,
                max_size,
            );
            buffer.set_len(actual_size as usize - 1);
            String::from_utf8(buffer).unwrap()
        }
    }
}

impl fmt::Debug for JSString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "JSString {{ \"{}\" }}", self.to_string())
    }
}

impl Drop for JSString {
    fn drop(&mut self) {
        unsafe { ul_sys::JSStringRelease(self.raw) }
    }
}

impl PartialEq for JSString {
    fn eq(&self, other: &JSString) -> bool {
        unsafe { ul_sys::JSStringIsEqual(self.raw, other.raw) }
    }
}

impl<'s> PartialEq<&'s str> for JSString {
    fn eq(&self, other: &&'s str) -> bool {
        let utf8 = CString::new(other.as_bytes()).unwrap();
        unsafe { ul_sys::JSStringIsEqualToUTF8CString(self.raw, utf8.as_ptr()) }
    }
}

impl PartialEq<String> for JSString {
    fn eq(&self, other: &String) -> bool {
        let utf8 = CString::new(other.as_bytes()).unwrap();
        unsafe { ul_sys::JSStringIsEqualToUTF8CString(self.raw, utf8.as_ptr()) }
    }
}

impl<'s> PartialEq<JSString> for &'s str {
    fn eq(&self, other: &JSString) -> bool {
        let utf8 = CString::new(self.as_bytes()).unwrap();
        unsafe { ul_sys::JSStringIsEqualToUTF8CString(other.raw, utf8.as_ptr()) }
    }
}

impl PartialEq<JSString> for String {
    fn eq(&self, other: &JSString) -> bool {
        let utf8 = CString::new(self.as_bytes()).unwrap();
        unsafe { ul_sys::JSStringIsEqualToUTF8CString(other.raw, utf8.as_ptr()) }
    }
}

impl<'s> From<&'s str> for JSString {
    fn from(s: &'s str) -> Self {
        let c = CString::new(s.as_bytes()).unwrap();
        JSString {
            raw: unsafe { ul_sys::JSStringCreateWithUTF8CString(c.as_ptr()) },
        }
    }
}

impl From<String> for JSString {
    fn from(s: String) -> Self {
        let c = CString::new(s.as_bytes()).unwrap();
        JSString {
            raw: unsafe { ul_sys::JSStringCreateWithUTF8CString(c.as_ptr()) },
        }
    }
}

impl<'s> From<&'s JSString> for String {
    fn from(s: &'s JSString) -> Self {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::JSString;

    #[test]
    fn from_conversion() {
        let a: JSString = "abc".into();
        let b: JSString = "abc".to_owned().into();
        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_eq!(b, b);

        let c: JSString = "def".into();
        assert_ne!(a, c);

        let d: JSString = "abcdef".into();
        assert_ne!(a, d);

        let e: String = (&d).into();
        assert_eq!(e, "abcdef");
    }

    #[test]
    fn equality() {
        let a: JSString = "abc".into();
        let s: String = "abc".to_owned();

        assert_eq!(a, "abc");
        assert_eq!(a, s);

        assert_eq!("abc", a);
        assert_eq!(s, a);
    }
}
