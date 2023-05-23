#pragma once

#include <optional>
#include <cstdint>
#include "patterns.hpp"

std::optional<std::uintptr_t> aob_scan(const char* pattern, const char* mask);
std::optional<std::uintptr_t> aob_scan(Pattern pattern);