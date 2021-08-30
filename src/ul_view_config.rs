#[derive(Default)]
pub struct UltralightViewConfig {

    enableImages: Option<bool>,
    enableJavaScript: Option<bool>,

    initialDeviceScale: Option<f64>,
    initialFocus: Option<bool>,

    fontFamilyFixed: Option<String>,
    fontFamilySansSerif: Option<String>,
    fontFamilySerif: Option<String>,
    fontFamilyStandard: Option<String>,

    isAccelerated: Option<bool>,
    isTransparent: Option<bool>,

    userAgent: Option<String>,
}

impl UltralightViewConfig {
    pub fn new() -> UltralightViewConfig {
        UltralightViewConfig {
            enableImages: None,
            enableJavaScript: None,

            fontFamilyFixed: None,
            fontFamilySansSerif: None,
            fontFamilySerif: None,
            fontFamilyStandard: None,

            initialDeviceScale: None,
            initialFocus: None,

            isAccelerated: None,
            isTransparent: None,

            userAgent: None,
        }
    }

    pub fn to_ul_viewconfig(&self) -> ul_sys::ULViewConfig {
        let view_config = unsafe {
            ul_sys::ulCreateViewConfig()
        };

        set_config!(view_config, self, enableImages, ulViewConfigSetEnableImages);
        set_config!(view_config, self, enableJavaScript, ulViewConfigSetEnableJavaScript    );

        set_config!(view_config, self, initialDeviceScale, ulViewConfigSetInitialDeviceScale);
        set_config!(view_config, self, initialFocus, ulViewConfigSetInitialFocus);

        set_config_str!(view_config, self, fontFamilyFixed, ulViewConfigSetFontFamilyFixed);
        set_config_str!(view_config, self, fontFamilySansSerif, ulViewConfigSetFontFamilySansSerif);
        set_config_str!(view_config, self, fontFamilySerif, ulViewConfigSetFontFamilySerif);
        set_config_str!(view_config, self, fontFamilyStandard, ulViewConfigSetFontFamilyStandard);

        set_config!(view_config, self, isAccelerated, ulViewConfigSetIsAccelerated);
        set_config!(view_config, self, isTransparent, ulViewConfigSetIsAccelerated);

        set_config_str!(view_config, self, userAgent, ulViewConfigSetUserAgent);

        view_config
    }

    config_item!( enableImages, bool, "Set whether images should be enabled (default = true)." );
    config_item!( enableJavaScript, bool, "Set whether JavaScript should be eanbled (default = true)." );

    config_item!( initialDeviceScale, f64, "Set the amount that the application DPI has been scaled, used for scaling device coordinates to pixels and oversampling raster shapes (default = 1.0)." );
    config_item!( initialFocus, bool, "Set whether view should be focused after it is created." );

    config_item!( fontFamilyFixed, String, "Set default font-family to use for fixed fonts, eg <pre> and <code> (Default = Courier New)." );
    config_item!( fontFamilySansSerif, String, "Set default font-family to use for serif fonts (Default = Times New Roman)." );
    config_item!( fontFamilySerif, String, "Set default font-family to use (Default = Times New Roman)." );
    config_item!( fontFamilyStandard, String, "Set default font-family to use (Default = Times New Roman)" );

    config_item!( userAgent, String, "Set user agent string (See <Ultralight/platform/Config.h> for the default)." );
}
