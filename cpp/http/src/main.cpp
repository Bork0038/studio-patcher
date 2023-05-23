#define WIN32_LEAN_AND_MEAN

#include "windows.h"
#include <cstdint>
#include <stdexcept>
#include <ios>
#include <iostream>
#include "scanner.hpp"
#include "hooks.hpp"

inline void alloc_console() {
    AllocConsole();

    FILE* file;
    freopen_s( &file, "CONOUT$", "w", stdout );
    freopen_s( &file, "CONOUT$", "w", stderr );
    freopen_s( &file, "CONIN$" , "r", stdin  );

    SetConsoleTitle( TEXT("studio patcher") );
}

inline void entry( HMODULE hmod ) {
    try {
        hooks::init();
    } catch( const std::runtime_error& error ) {

    }
}

int __stdcall DllMain( HMODULE hmod, uint32_t reason, void*) {
    if ( reason == DLL_PROCESS_ATTACH ) {
        alloc_console();
        entry( hmod );
    }

    return true;
}