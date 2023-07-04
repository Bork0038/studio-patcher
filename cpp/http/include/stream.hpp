#pragma once

#include "cstdint"
#include <vector>
#include <iostream>

class NetworkStream {

public:
	std::vector<uint8_t> data;
	int32_t read_pointer;

	void ignore_bytes(int32_t bytes) {
		read_pointer += bytes;
	}

	void write_bytes(unsigned char* bytes, int len) {
		for (int i = 0; i < len; i++) {
			data.push_back(bytes[i]);
		}
	}

	void write_byte(uint8_t byte) {
		data.push_back(byte);
	}

	template <typename T>
	void write_le(T t) {
		uint8_t* ptr = reinterpret_cast<uint8_t*>(&t);
		std::vector<uint8_t> bytes(ptr, ptr + sizeof(T));

		for (int i = 0; i < bytes.size(); i++) {
			data.push_back(bytes[i]);
		}
	}

	void write_string(const char* str) {
		int len = strlen(str);

		write_le(len);
		write_bytes((unsigned char*)str, len);
	}
};