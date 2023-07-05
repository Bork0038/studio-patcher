use std::collections::HashMap;

// fuck you roblox for making me have to do this
pub fn get_file_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ( "content-studio_svg_textures.zip", "/content/studio_svg_textures/" ),
        ( "content-models.zip",              "/content/models/" ),
        ( "content-textures3.zip",           "/PlatformContent/pc/textures/" ),
        ( "content-textures2.zip",           "/PlatformContent/pc/textures/" ),
        ( "content-terrain.zip",             "/PlatformContent/pc/terrain/" ),
        ( "content-platform-fonts.zip",      "/PlatformContent/pc/fonts/" ),
        ( "content-qt_translations.zip",     "/content/qt_translations/" ),    
        ( "content-api-docs.zip",            "/content/api_docs/" ),
        ( "content-avatar.zip",              "/content/avatar/" ),
        ( "content-configs.zip",             "/content/configs/" ),
        ( "content-sounds.zip",              "/content/sonds/" ),
        ( "content-sky.zip",                 "/content/sky/" ),
        ( "content-fonts.zip",               "/content/fonts/" ),
        ( "extracontent-scripts.zip",        "/ExtraContent/scripts/" ),
        ( "extracontent-luapackages.zip",    "/ExtraContent/LuaPackages/" ),
        ( "extracontent-translations.zip",   "/ExtraContent/translations/" ),
        ( "extracontent-models.zip",         "/ExtraContent/models/" ),
        ( "extracontent-textures.zip",       "/ExtraContent/textures/" ),
        ( "shaders.zip",                     "/shaders/" ),
        ( "BuiltInPlugins.zip",              "/BuiltInPlugins/" ),
        ( "BuiltInStandalonePlugins.zip",    "/BuiltInStandalonePlugions/" ),
        ( "LibrariesQt5.zip",                "/" ),
        ( "Plugins.zip",                     "/Plugins/" ),
        ( "Qml.zip",                         "/Qml/" ),
        ( "StudioFonts.zip",                 "/StudioFonts/" ),
        ( "ssl.zip",                         "/ssl/" ),
        ( "WebView2.zip",                    "/" ),
        ( "WebView2RuntimeInstaller.zip",    "/WebView2RuntimeInstaller/" ),
        ( "ApplicationConfig.zip",           "/ApplicationConfig/" ),
        ( "RobloxStudio.zip",                "/" ),
        ( "Libraries.zip" ,                  "/" ),
        ( "redist.zip",                      "/" ),
    ])
}