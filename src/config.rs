#[derive(Default)]
pub struct UltralightConfig {
    enableImages: Option<bool>,
    enableJavaScript: Option<bool>,
    forceRepaint: Option<bool>,
    useBGRAForOffscreenRendering: Option<bool>,

    animationTimerDelay: Option<f64>,
    deviceScaleHint: Option<f64>,

    memoryCacheSize: Option<u32>,
    pageCacheSize: Option<u32>,

    fontFamilyStandard: Option<String>,
    fontFamilyFixed: Option<String>,
    fontFamilySerif: Option<String>,
    fontFamilySansSerif: Option<String>,
    userAgent: Option<String>,
    userStylesheet: Option<String>,
}

#[rustfmt::skip]
impl UltralightConfig {
    pub fn new() -> UltralightConfig {
        UltralightConfig {
            enableImages: None,
            enableJavaScript: None,
            forceRepaint: None,
            useBGRAForOffscreenRendering: None,

            animationTimerDelay: None,
            deviceScaleHint: None,

            memoryCacheSize: None,
            pageCacheSize: None,

            fontFamilyStandard: None,
            fontFamilyFixed: None,
            fontFamilySerif: None,
            fontFamilySansSerif: None,
            userAgent: None,
            userStylesheet: None,
        }
    }

    pub fn to_ulconfig(&self) -> ul_sys::ULConfig {
        let config = unsafe {
            ul_sys::ulCreateConfig()
        };

        set_config!(config, self, enableImages, ulConfigSetEnableImages);
        set_config!(config, self, enableJavaScript, ulConfigSetEnableJavaScript);
        set_config!(config, self, forceRepaint, ulConfigSetForceRepaint);
        set_config!(config, self, useBGRAForOffscreenRendering, ulConfigSetUseBGRAForOffscreenRendering);

        set_config!(config, self, animationTimerDelay, ulConfigSetAnimationTimerDelay);
        set_config!(config, self, deviceScaleHint, ulConfigSetDeviceScaleHint);

        set_config!(config, self, memoryCacheSize, ulConfigSetMemoryCacheSize);
        set_config!(config, self, pageCacheSize, ulConfigSetPageCacheSize);

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

    config_item!( animationTimerDelay, f64, "Set the amount of time to wait before triggering another repaint when a CSS animation is active. (Default = 1.0 / 60.0)" );
    config_item!( deviceScaleHint, f64, "Set the amount that the application DPI has been scaled, used for oversampling raster shapes. (Default = 1f64)" );

    config_item!( memoryCacheSize, u32, "Set the size of WebCore's memory cache for decoded images, scripts, and other assets in bytes. (Default = 64 * 1024 * 1024)" );
    config_item!( pageCacheSize, u32, "Set the number of pages to keep in the cache. (Default = 0)" );

    config_item!( fontFamilyStandard, String, "Set default font-family to use (Default = Times New Roman)" );
    config_item!( fontFamilyFixed, String, "Set default font-family to use for fixed fonts, eg pre and code tags. (Default = Courier New)" );
    config_item!( fontFamilySerif, String, "Set default font-family to use for serif fonts. (Default = Times New Roman)" );
    config_item!( fontFamilySansSerif, String, "Set default font-family to use for sans-serif fonts. (Default = Arial)" );
    config_item!( userAgent, String, "Set user agent string. (See <Ultralight/platform/Config.h> for the default)" );
    config_item!( userStylesheet, String, "Set user stylesheet (CSS). (Default = Empty)" );
}
