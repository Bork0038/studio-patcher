#pragma once

#include <iostream>
#include <cstdint>

#include "scanner.hpp"
#include "kthook/kthook.hpp"
#include "requests.hpp"
#include "stream.hpp"
#include <chrono>
#include <thread>
namespace hooks {
    


    typedef std::pair<std::string, std::string> header;

    const char* get_string_at_addr( __int64 addr ) {
        const auto len = *reinterpret_cast<__int64*>(addr + 0x10);
        const auto ptr = len >= 0x10
            ? *reinterpret_cast<__int64*>(addr)
            : addr;

        return (const char*)ptr;
        return "";
    }

    void log_req(__int64 self) {
        const auto status_code = *reinterpret_cast<int*>(self + 0x18);
        const auto headers = reinterpret_cast<std::string*>(self + 0x38);
        const auto body = reinterpret_cast<std::string*>(self + 0x58);

        const auto req = *reinterpret_cast<__int64*>(self + 0xD0);
        const auto url = reinterpret_cast<std::string*>(req + 0x50);
        const auto method = reinterpret_cast<std::string*>(req + 0x110);
        const auto req_body = reinterpret_cast<std::string*>(req + 0x130);
           
        auto stream = NetworkStream{};
        stream.write_le(status_code);
        stream.write_string(headers->c_str());
        stream.write_string(body->data());
        stream.write_string(url->c_str());
        stream.write_string(method->c_str());
        stream.write_string(req_body->c_str());

        std::vector<std::pair<std::string, std::string>> header_list = {};
        auto start = *reinterpret_cast<__int64*>(req + 0x18);
        const auto end = *reinterpret_cast<__int64*>(req + 0x20);

        if (start != end) {
            while (true) {
                const auto header_name = get_string_at_addr(start);
                const auto header_val = get_string_at_addr(start + 0x20);

                std::pair<std::string, std::string> header = { header_name, header_val };
                header_list.push_back(header);

                start += 0x40;
                if (start == end) break;
            }
        }

        stream.write_le((int)header_list.size());
        for (const auto [key, val] : header_list) {
            stream.write_string(key.c_str());
            stream.write_string(val.c_str());
        }
        
        try {
            http::Request request{ "http://localhost:27773/http" };
            request.send(
                "POST",
                stream.data,
                {
                    { "Content-Type", "application/octet-stream" }
                },
                std::chrono::seconds(1)
            );
        }
        catch (const std::exception& e) {
            std::cerr << "Failed to send request: " << e.what() << std::endl;
        }
    }

    typedef void(__fastcall* http_res_t)(__int64 self, __int64* res);


    kthook::kthook_simple<http_res_t> hook{};
    void init() {
        const auto addr = aob_scan(patterns::http_res);
        if (!addr) {
            throw std::runtime_error("failed to find RBX::HttpClient::logRes");
        }

        auto callback = [](const auto& hook, __int64 &self, __int64 *&res) {
            log_req( *res );

            return false;
        };
        
        hook.set_dest( *addr );
        hook.set_cb( callback );
        hook.install();
    }
}
// namespace hooks {

//     // typedef void(__fastcall* http_res_t)(__int64 self, __int64* res);
//     // http_res_t old_http_res = nullptr;
//     // uint64_t addr_http_res = 0;
//     // void __fastcall hook_http_res(__int64 self, __int64* res) {
//     //  /*   std::thread task(log_req, *res);
//     //     task.join();*/
//     //     std::cout << "a" << std::endl;

//     //     // return (*PLH::FnCast(addr_http_res, &old_http_res))(self, res);
//     // }


   
// }