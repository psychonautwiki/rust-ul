# javascriptcore

[![](http://meritbadge.herokuapp.com/javascriptcore)](https://crates.io/crates/javascriptcore)

This crate provides a safe binding to the public API for
[JavaScriptCore](https://trac.webkit.org/wiki/JavaScriptCore),
the JavaScript engine used by the WebKit browser.

Dual licensed under the MIT and Apache 2 licenses.

## Documentation

The API is fully documented with examples:
[https://endoli.github.io/javascriptcore.rs/](https://endoli.github.io/javascriptcore.rs/)

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/javascriptcore).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
javascriptcore = "0.0.4"
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Support and Maintenance

I am developing this library largely on my own so far. I am able
to offer support and maintenance, but would very much appreciate
donations via [Patreon](https://patreon.com/endoli). I can also
provide commercial support, so feel free to
[contact me](mailto:bruce.mitchener@gmail.com).

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
