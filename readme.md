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
// in development

fn main() {
    let config = ul::Config::new();

    let mut ul_app = ul::UltralightApp::new(Some(config));

    ul_app.window(
        1024u32, 1024u32, false, false, true, true, false,
    );

    let mut ul = ul::Ultralight::new(None, Some(ul_app.get_renderer()));

    ul.set_view(ul_app.get_view().unwrap());
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

    ul_app.resize_overlay(1024u32, 1024u32);

    ul.set_finish_loading_callback(|_view| println!("loaded!"));

    ul.set_dom_ready_callback(|_view| println!("dom ready!"));

    ul_app.set_window_resize_callback(|width: u32, height: u32| {
        ul_app.resize_overlay(width, height);
    });

    ul_app.run();
}
```

