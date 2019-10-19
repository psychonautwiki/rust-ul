fn main() {
    let config = ul::Config::new();
    let settings = ul::Settings::new();

    let mut ul_app = ul::UltralightApp::new(
        Some(settings),
        Some(config),
    );

    ul_app.window(
        500u32, 500u32, false, false, true, true, false,
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

    let mut finished = |view| println!("loaded!");
    let mut dom_ready = |view| println!("dom ready!");

    ul.set_finish_loading_callback(&mut finished);
    ul.set_dom_ready_callback(&mut dom_ready);

    ul_app.set_window_resize_callback(&mut |width: u32, height: u32| {
        ul_app.resize_overlay(width, height);
    });

    ul_app.run();
}
