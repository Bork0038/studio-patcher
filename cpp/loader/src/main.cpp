#include "Windows.h"
#include "cstdint"

#pragma comment(linker, "/export:GetAvailableCoreWebView2BrowserVersionString=WebView2LoaderOld.GetAvailableCoreWebView2BrowserVersionString")
#pragma comment(linker, "/export:CreateCoreWebView2EnvironmentWithOptions=WebView2LoaderOld.CreateCoreWebView2EnvironmentWithOptions")
#pragma comment(linker, "/export:CompareBrowserVersions=WebView2LoaderOld.CompareBrowserVersions")

int __stdcall DllMain( HMODULE hmod, uint32_t reason, void*) {
    LoadLibraryA("http.dll");

    return true;
}