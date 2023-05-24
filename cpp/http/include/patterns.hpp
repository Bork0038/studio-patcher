#pragma once

class Pattern {
public:
    const char* pattern;
    const char* mask;

    Pattern(const char* pattern, const char* mask)
        : pattern(pattern), mask(mask) {};
};

namespace patterns {
    
    const auto http_res = Pattern(
       "\x48\x89\x5C\x24\x08\x48\x89\x6C\x24\x10\x48\x89\x74\x24\x18\x57\x41\x56\x41\x57\x48\x83\xEC\x40\x4C\x8B\xFA\x48\x8B\xE9\x80\x3D\xFD\xBD\xAA\x03\x00\x0F\x84\x9C\x01\x00\x00\xBE\xFF\xFF\xFF\xFF\x80\x3D\xD1\xD6\x39\x03\x00", 
       "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    );

}