<h1 align="center">ul-sys</h1>

<h5 align="center">Low level bindings to Ultralight SDK. Ultralight is a light and fast option to integrate GPU-accelerated HTML UI in your app.</h5>

<div align="center">
  <a href="https://crates.io/crates/ul-sys">
    crates.io
  </a>
  —
  <a href="https://docs.rs/ul-sys/latest/ul_sys/">
    documentation
  </a>
  —
  <a href="https://ultralig.ht">
    Ultralight SDK
  </a>
</div>

<br />

```rust
let (width, height): (u32, u32) = (1280, 768);

let config = ul::ulCreateConfig();

let app = ul::ulCreateApp(config);
let monitor = ul::ulAppGetMainMonitor(app);
let window = ul::ulCreateWindow(monitor, width, height, false, 0);

ul::ulAppSetWindow(app, window);

let renderer = ul::ulAppGetRenderer(app);
let view = ul::ulCreateView(renderer, width, height, false);
let overlay = ul::ulCreateOverlay(window, width as i32, height as i32, 0, 0);
let view = ul::ulOverlayGetView(overlay);

ul::ulViewLoadURL(view, ulstr("https://apple.com"));

ul::ulAppRun(app);
```
