// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JavaScriptCore Bindings
//!
//! Evaluate JavaScript programs from within an app, and support
//! JavaScript scripting of your app.

#![deny(trivial_numeric_casts, unstable_features, unused_import_braces, unused_qualifications)]

mod base;
mod class;
mod context;
mod contextgroup;
mod exception;
mod object;
mod string;
mod value;

pub use base::{check_script_syntax, evaluate_script, garbage_collect};
pub use ul_sys::{JSType, JSTypedArrayType};

/// A JavaScript class.
///
/// Used with `JSObjectMake` to construct objects with custom
/// behavior.
///
/// TODO: Fix `JSObjectMake` reference once it has been wrapped.
pub struct JSClass {
    pub raw: ul_sys::JSClassRef,
}

/// A JavaScript execution context.
///
/// Holds the global object and other execution state.
pub struct JSContext {
    pub raw: ul_sys::JSGlobalContextRef,
}

/// A group that associates JavaScript contexts with one another.
///
/// Contexts in the same group may share and exchange JavaScript
/// objects. Sharing and/or exchanging JavaScript objects between
/// contexts in different groups will produce undefined behavior.
/// When objects from the same context group are used in multiple
/// threads, explicit synchronization is required.
pub struct JSContextGroup {
    pub raw: ul_sys::JSContextGroupRef,
}

/// A wrapper for a [`JSValue`] that contains an exception.
///
/// [`JSValue`]: struct.JSValue.html
#[derive(Debug)]
pub struct JSException {
    pub value: JSValue,
}

/// A JavaScript object.
///
/// An `JSObject` is a [`JSValue`]. This is implemented by having
/// `JSObject` implement the `Deref` trait so that anything that
/// expects a `JSValue` can receive a `JSObject` as well.
///
/// [`JSValue`]: struct.JSValue.html
pub struct JSObject {
    pub raw: ul_sys::JSObjectRef,
    pub value: JSValue,
}

/// A UTF16 character buffer.
///
/// The fundamental string representation in JavaScript. Since
/// this is using a UTF16 encoding and Rust strings are using
/// UTF8 encoding, converting between string representations
/// is not cheap.
///
/// In this crate, implementations of the conversion traits
/// `Into` and `From` are provided for `JSString`. This allows
/// conversion from `&str` and `String` into `JSString`:
///
/// ```
/// # use javascriptcore::JSString;
/// let j: JSString = "abc".into();
/// ```
///
/// Similarly, a `JSString` can be converted to a `String`
/// via a conversion trait or directly:
///
/// ```
/// # use javascriptcore::JSString;
/// let j: JSString = "abc".into();
/// let s: String = (&j).into(); // Requires a reference.
/// let s: String = j.to_string();
/// ```
///
/// In this crate, functions that need a `JSString` use
/// generics so that they can take anything that can be
/// converted to a `JSString` instead. This allows the
/// caller to pass an `&str` or `String`, or to cache a
/// previously converted `JSString` and pass that directly.
///
/// A `JSString` is not a [`JSValue`] and so it can not be
/// passed where a `JSValue` is expected. Instead, it must
/// be boxed using [`JSValue::new_string`].
///
/// [`JSValue`]: struct.JSValue.html
/// [`JSValue::new_string`]: struct.JSValue.html#method.new_string
#[derive(Eq)]
pub struct JSString {
    pub raw: ul_sys::JSStringRef,
}

/// A JavaScript value.
///
/// The base type for all JavaScript values, and polymorphic functions
/// on them.
///
/// All values passed between Rust and JavaScriptCore will be boxed with
/// a `JSValue`.
///
/// # Creating JS values
///
/// * [`JSValue::new_undefined`]
/// * [`JSValue::new_null`]
/// * [`JSValue::new_boolean`]
/// * [`JSValue::new_number`]
/// * [`JSValue::new_string`]
/// * [`JSValue::new_from_json`]
///
/// # JSON
///
/// * [`JSValue::new_from_json`]
/// * [`JSValue::to_json_string`]
///
/// # Retrieving Rust values
///
/// * [`JSValue::as_boolean`]
/// * [`JSValue::as_number`]
/// * [`JSValue::as_object`]
/// * [`JSValue::as_string`]
///
/// [`JSValue::new_undefined`]: #method.new_undefined
/// [`JSValue::new_null`]: #method.new_null
/// [`JSValue::new_boolean`]: #method.new_boolean
/// [`JSValue::new_number`]: #method.new_number
/// [`JSValue::new_string`]: #method.new_string
/// [`JSValue::new_from_json`]: #method.new_from_json
/// [`JSValue::to_json_string`]: #method.to_json_string
/// [`JSValue::as_boolean`]: #method.as_boolean
/// [`JSValue::as_number`]: #method.as_number
/// [`JSValue::as_object`]: #method.as_object
/// [`JSValue::as_string`]: #method.as_string
#[derive(Debug)]
pub struct JSValue {
    pub raw: ul_sys::JSValueRef,
    pub ctx: ul_sys::JSContextRef,
}
