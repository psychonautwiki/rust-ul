use crate::ul;

macro_rules! config_item (
    ($name:ident, $type:ty, $comment:expr) => (
        #[doc = $comment]
        pub fn $name(&mut self, flag: $type) {
           self.$name = Some(flag);
        }
    )
);

macro_rules! set_config (
    ($config: expr, $self: expr, $name:ident, $ffiName:ident) => (
        if $self.$name.is_some() {
            unsafe {
                ul::$ffiName($config, $self.$name.unwrap());
            }
        }
    )
);

macro_rules! set_config_str (
    ($config: expr, $self: expr, $name:ident, $ffiName:ident) => (
        if $self.$name.is_some() {
            unsafe {
                let str = ul::ulCreateString(
                    std::ffi::CString::new(
                        $self.$name.clone().unwrap()
                    ).unwrap().as_ptr()
                );

                ul::$ffiName($config, str);
            }
        }
    )
);

#[derive(Default)]
pub struct UltralightConfig {
    enableImages: Option<bool>,
    enableJavaScript: Option<bool>,
    useBGRAForOffscreenRendering: Option<bool>,
    deviceScaleHint: Option<f64>,
    fontFamilyStandard: Option<String>,
    fontFamilyFixed: Option<String>,
    fontFamilySerif: Option<String>,
    fontFamilySansSerif: Option<String>,
    userAgent: Option<String>,
    userStylesheet: Option<String>,
}

impl UltralightConfig {
    pub fn new() -> UltralightConfig {
        UltralightConfig {
            enableImages: None,
            enableJavaScript: None,
            useBGRAForOffscreenRendering: None,
            deviceScaleHint: None,
            fontFamilyStandard: None,
            fontFamilyFixed: None,
            fontFamilySerif: None,
            fontFamilySansSerif: None,
            userAgent: None,
            userStylesheet: None,
        }
    }

    pub fn to_ulconfig(&self) -> ul::ULConfig {
        let config = unsafe {
            ul::ulCreateConfig()
        };

        set_config!(config, self, enableImages, ulConfigSetEnableImages);
        set_config!(config, self, enableJavaScript, ulConfigSetEnableJavaScript);
        set_config!(config, self, useBGRAForOffscreenRendering, ulConfigSetUseBGRAForOffscreenRendering);
        set_config!(config, self, deviceScaleHint, ulConfigSetDeviceScaleHint);
        set_config_str!(config, self, fontFamilyStandard, ulConfigSetFontFamilyStandard);
        set_config_str!(config, self, fontFamilyFixed, ulConfigSetFontFamilyFixed);
        set_config_str!(config, self, fontFamilySerif, ulConfigSetFontFamilySerif);
        set_config_str!(config, self, fontFamilySansSerif, ulConfigSetFontFamilySansSerif);
        set_config_str!(config, self, userAgent, ulConfigSetUserAgent);
        set_config_str!(config, self, userStylesheet, ulConfigSetUserStylesheet);

        config
    }

    config_item!( enableImages, bool, "Set whether images should be enabled (Default = true)" );
    config_item!( enableJavaScript, bool, "Set whether JavaScript should be eanbled (Default = true)" );
    config_item!( useBGRAForOffscreenRendering, bool, "Set whether we should use BGRA byte order (instead of RGBA) for View bitmaps. (Default = false)" );
    config_item!( deviceScaleHint, f64, "Set the amount that the application DPI has been scaled, used for oversampling raster shapes. (Default = 1f64)" );
    config_item!( fontFamilyStandard, String, "Set default font-family to use (Default = Times New Roman)" );
    config_item!( fontFamilyFixed, String, "Set default font-family to use for fixed fonts, eg pre and code tags. (Default = Courier New)" );
    config_item!( fontFamilySerif, String, "Set default font-family to use for serif fonts. (Default = Times New Roman)" );
    config_item!( fontFamilySansSerif, String, "Set default font-family to use for sans-serif fonts. (Default = Arial)" );
    config_item!( userAgent, String, "Set user agent string. (See <Ultralight/platform/Config.h> for the default)" );
    config_item!( userStylesheet, String, "Set user stylesheet (CSS). (Default = Empty)" );
}
