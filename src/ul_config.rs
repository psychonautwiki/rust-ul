#[derive(Default)]
pub struct UltralightConfig {
    forceRepaint: Option<bool>,

    animationTimerDelay: Option<f64>,
    recycleDelay: Option<f64>,
    scrollTimerDelay: Option<f64>,

    fontGamma: Option<f64>,
    fontHinting: Option<u32>,

    memoryCacheSize: Option<u32>,
    pageCacheSize: Option<u32>,
    minLargeHeapSize: Option<u32>,
    minSmallHeapSize: Option<u32>,
    overrideRAMSize: Option<u32>,

    faceWinding: Option<u32>,

    cachePath: Option<String>,

    userStylesheet: Option<String>,
}

impl UltralightConfig {
    pub fn new() -> UltralightConfig {
        UltralightConfig {
            forceRepaint: None,

            animationTimerDelay: None,
            recycleDelay: None,
            scrollTimerDelay: None,

            fontGamma: None,
            fontHinting: None,

            memoryCacheSize: None,
            pageCacheSize: None,
            minLargeHeapSize: None,
            minSmallHeapSize: None,
            overrideRAMSize: None,

            faceWinding: None,

            cachePath: None,

            userStylesheet: None,
        }
    }

    pub fn to_ulconfig(&self) -> ul_sys::ULConfig {
        let config = unsafe {
            ul_sys::ulCreateConfig()
        };

        set_config!(config, self, forceRepaint, ulConfigSetForceRepaint);

        set_config!(config, self, animationTimerDelay, ulConfigSetAnimationTimerDelay);
        set_config!(config, self, recycleDelay, ulConfigSetRecycleDelay);
        set_config!(config, self, scrollTimerDelay, ulConfigSetScrollTimerDelay);

        set_config!(config, self, memoryCacheSize, ulConfigSetMemoryCacheSize);
        set_config!(config, self, pageCacheSize, ulConfigSetPageCacheSize);
        set_config!(config, self, minLargeHeapSize, ulConfigSetMinLargeHeapSize);
        set_config!(config, self, minSmallHeapSize, ulConfigSetMinSmallHeapSize);
        set_config!(config, self, overrideRAMSize, ulConfigSetOverrideRAMSize);

        set_config!(config, self, fontGamma, ulConfigSetFontGamma);
        set_config!(config, self, fontHinting, ulConfigSetFontHinting);

        set_config_str!(config, self, userStylesheet, ulConfigSetUserStylesheet);

        set_config_str!(config, self, cachePath, ulConfigSetCachePath);

        config
    }

    config_item!( forceRepaint, bool, "Set whether or not we should continuously repaint any Views or compositor layers, regardless if they are dirty or not. This is mainly used to diagnose painting/shader issues. (Default = False)" );

    config_item!( animationTimerDelay, f64, "Set the amount of time to wait before triggering another repaint when a CSS animation is active. (Default = 1.0 / 60.0)" );
    config_item!( recycleDelay, f64, "The amount of time (in seconds) to wait before running the recycler (will attempt to return excess memory back to the system). (Default = 4.0)" );
    config_item!( scrollTimerDelay, f64, "When a smooth scroll animation is active, the amount of time (in seconds) to wait before triggering another repaint. Default is 60 Hz." );

    config_item!( memoryCacheSize, u32, "Set the size of WebCore’s memory cache for decoded images, scripts, and other assets in bytes. (Default = 64 * 1024 * 1024)" );
    config_item!( pageCacheSize, u32, "Set the number of pages to keep in the cache. (Default = 0)" );
    config_item!( minLargeHeapSize, u32, "The minimum size of large VM heaps in JavaScriptCore. Set this to a lower value to make these heaps start with a smaller initial value." );
    config_item!( minSmallHeapSize, u32, "The minimum size of small VM heaps in JavaScriptCore. Set this to a lower value to make these heaps start with a smaller initial value." );
    config_item!( overrideRAMSize, u32, "JavaScriptCore tries to detect the system’s physical RAM size to set reasonable allocation limits. Set this to anything other than 0 to override the detected value. Size is in bytes." );

    config_item!( faceWinding, u32, "The winding order for front-facing triangles. @see FaceWinding" );
    config_item!( fontGamma, f64, "The gamma to use when compositing font glyphs, change this value to adjust contrast (Adobe and Apple prefer 1.8, others may prefer 2.2). (Default = 1.8)" );
    config_item!( fontHinting, u32, "The hinting algorithm to use when rendering fonts. (Default = kFontHinting_Normal) @see ULFontHinting" );

    config_item!( userStylesheet, String, "Set user stylesheet (CSS). (Default = Empty)" );
}
