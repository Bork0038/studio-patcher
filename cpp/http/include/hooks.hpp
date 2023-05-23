#pragma once

#include <iostream>
#include <cstdint>

#include "scanner.hpp"
#include "polyhook2/IHook.hpp"
#include "polyhook2/Detour/x64Detour.hpp"
#include "polyhook2/Exceptions/BreakPointHook.hpp"

namespace hooks {

    typedef void(__fastcall* http_res_t)(__int64 self, __int64* res);
    http_res_t old_http_res = nullptr;
    uint64_t addr_http_res = 0;
    void __fastcall hook_http_res(__int64 self, __int64* res) {
     /*   std::thread task(log_req, *res);
        task.join();*/
        std::cout << "a" << std::endl;

        return (*PLH::FnCast(addr_http_res, &old_http_res))(self, res);
    }


    void init() {
        const auto res = aob_scan(patterns::http_res);
        if (!res) {
            throw std::runtime_error("failed to find RBX::HttpClient::logRes");
        }
        
        old_http_res = (http_res_t)*res;
        PLH::x64Detour detour(*res, (uint64_t)hook_http_res, &addr_http_res);
        detour.hook();
    }
}