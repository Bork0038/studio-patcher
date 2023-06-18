#pragma once
#define WIN32_LEAN_AND_MEAN


#include <windows.h>
#include <iostream>

#include "scanner.hpp"
#include "patterns.hpp"
#include "structs.hpp"
#include "stream.hpp"
#include "kthook/kthook.hpp"
#include "requests.hpp"

namespace hooks {

    void send_packet( Opcodes opcode, PacketType type, SystemAddress address, unsigned char* data, unsigned int len ) {
        auto stream = NetworkStream {};
        stream.write_byte(opcode);
        stream.write_byte(type);
        stream.write_system_address(address);

        const auto packet_data = std::string((const char*)data, len);
        stream.write_le(len);
        stream.write_bytes(data, len);
  
        try {
            http::Request request{ "http://localhost:27773/raknet" };
            request.send(
                "POST",
                stream.data,
                {
                    { "Content-Type", "application/octet-stream" }
                },
                std::chrono::milliseconds(500)
            );
        }
        catch (const std::exception& e) {
            std::cerr << "Failed to send request: " << e.what() << std::endl;
        }
    }

    typedef __int64(__fastcall* client_on_receive_t)(__int64 self, Packet* packet);
    kthook::kthook_signal<client_on_receive_t> client_on_receive_hook {};

    void init_on_receive_hook() {
        const auto addr = aob_scan(patterns::client_on_receive);
        if (!addr) {
            throw std::runtime_error("failed to find RBX::Network::OnReceive");
        }

        auto callback = [](const auto& hook, __int64 &self, Packet *& packet) {
            send_packet(
                Opcodes::IncomingPackets,
                PacketType::StudioClient,
                packet->systemAddress,
                packet->data,
                packet->length
            );

            return std::nullopt;
        };

        client_on_receive_hook.set_dest( *addr );
        client_on_receive_hook.before.connect( callback );
        // client_on_receive_hook.set_cb( callback );
        client_on_receive_hook.install();
    }

    void init() {
        init_on_receive_hook();
    }

}