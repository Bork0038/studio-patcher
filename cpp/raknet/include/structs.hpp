#pragma once
#include <stdio.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <cstdint>

typedef unsigned short SystemIndex;
typedef uint32_t BitSize_t;

struct RakNetGUID
{
	uint64_t g;
	SystemIndex systemIndex;
};

struct SystemAddress
{
	union
	{
		struct sockaddr_in6 addr6;
		struct sockaddr_in addr4;
	} address;

	unsigned short debugPort;
	SystemIndex systemIndex;
};

struct Packet
{
	SystemAddress systemAddress;
	RakNetGUID guid;
	unsigned int length;
	BitSize_t bitSize;

	unsigned char* data;
	bool deleteData;
	bool wasGeneratedLocally;
};

struct AddressOrGUID
{
	RakNetGUID rakNetGuid;
	SystemAddress systemAddress;
};

enum Opcodes {
    IncomingPackets,
	OutgoingPackets
};

enum PacketType {
    StudioClient,
    TestClient,
    Server
};

enum PacketPriority {
	ImmediatePriority,
	HighPriority,
	MediumPriority,
	LowPriority,
	NumberOfPriorities
};

enum PacketReliability {
	Unreliable,
	UnreliableSequenced,
	Reliable,
	ReliableOrdered,
	ReliableSequenced,
	UnreliableWithAckReceipt,
	ReliableWithAckReceipt,
	ReliableOrderedWithAckReceipt,
	NumberOfReliabilites,
};