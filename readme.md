<h1 align="center">ul</h1>

<h5 align="center">High level bindings to Ultralight SDK. Ultralight is a light and fast option to integrate GPU-accelerated HTML UI in your app.</h5>

<div align="center">
  <a href="https://crates.io/crates/ul">
    crates.io
  </a>
  —
  <a href="https://docs.rs/ul/latest/ul/">
    documentation
  </a>
  —
  <a href="https://ultralig.ht">
    Ultralight SDK
  </a>
</div>

<br />

```rust
#![allow(dead_code, unused_must_use)]

use ::ul::*;

fn main() {
    let mut config = ul::Config::new();
    let settings = ul::Settings::new();

    let mut ul_app = ul::UltralightApp::new(
        Some(settings),
        Some(config),
    );

    ul_app.window(
        853u32, 480u32,
        false, false, true, true, false,
    );

    let mut ul = ul::Ultralight::new(
        None,
        Some(ul_app.get_renderer()),
    );

    ul.app(&mut ul_app);

    ul.log_to_stdout();

    ul.load_html(r#"
        <html>
            <head>
                <style>
                    body {
                        background-color: black;
                        color: white;
                        font-size: 100px;
                    }
                </style>
            </head>
            <body>Hello</body>
        </html>"#);

    // or ..
    //ul.load_url("https://sly.mn");

    ul_app.overlay_resize(853u32, 480u32);

    let mut finished = |_| println!("loaded!");
    let mut dom_ready = |_| println!("dom ready!");

    ul.set_finish_loading_callback(&mut finished);
    ul.set_dom_ready_callback(&mut dom_ready);

    ul_app.window_set_resize_callback(&mut |width: u32, height: u32| {
        ul_app.overlay_resize(width, height);
    });

    ul_app.run();
}
```

