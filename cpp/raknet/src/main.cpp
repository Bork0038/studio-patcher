#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <ios>
#include <iostream>
#include <cstdint>
#include <stdexcept>
#include "hooks.hpp"

inline void allocate_console() {
    AllocConsole();

    FILE* file;
    freopen_s(&file, "CONOUT$", "w", stdout);
    freopen_s(&file, "CONOUT$", "w", stderr);
    freopen_s(&file, "CONIN$", "r", stdin);

    SetConsoleTitle( TEXT("studio packet logger") );
}

void entry(HMODULE hmod) {
    try {
        hooks::init();
    } catch (const std::runtime_error& e) {
        std::cout << e.what() << std::endl;
    }
}


int __stdcall DllMain(HMODULE hmod, std::uint32_t reason, void*) {
    if (reason == DLL_PROCESS_ATTACH) {
        allocate_console();
        entry(hmod);
    }

    return true;
}