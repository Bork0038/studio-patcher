#pragma once

class Pattern {
public:
    const char* pattern;
    const char* mask;

    Pattern(const char* pattern, const char* mask)
        : pattern(pattern), mask(mask) {};
};

namespace patterns {
    
    const auto client_on_receive = Pattern(
        "\x48\x89\x5C\x24\x08\x55\x56\x57\x41\x56\x41\x57\x48\x8D\xAC\x24\x50\xFC\xFF\xFF\x48\x81\xEC\xB0\x04\x00\x00\x48\x8B\xF2\x4C\x8B\xF1\x8B\x0D\xCC\xCC\xCC\xCC\xE8", 
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????x"
    );

    const auto rakpeer_send = Pattern(
        "\x48\x89\x5C\x24\x00\x48\x89\x74\x24\x00\x48\x89\x54\x24\x00\x55\x57\x41\x54\x41\x56\x41\x57\x48\x8D\x6C\x24\x00\x48\x81\xEC\x00\x00\x00\x00\x45\x8B\xF9\x45\x8B\xE0", 
        "xxxx?xxxx?xxxx?xxxxxxxxxxxx?xxx????xxxxxx"
    );

}