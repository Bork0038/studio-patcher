import Vector3 from "./vector3.js";
import { Buffer } from "buffer";

export default class NetworkStream {
    constructor( data ) {
        this.data = Buffer.from( data || "", "ascii" );
        this.pointer = 0;
    }

    static from( str, encoding ) {
        return new NetworkStream(
            Buffer.from( str, encoding )
        )
    }

    writeBytes( buffer ) {
        if (typeof buffer == "string") buffer = Buffer.from( buffer );

        const buf = Buffer.alloc( buffer.length + this.data.length );
        this.data.copy( buf, 0, 0, buf.length );
        buffer.copy( buf, this.data.length, 0, buffer.length );

        this.data = buf;
    }

    writeByte( int ) {
        this.writeBytes( 
            String.fromCodePoint( int )
        );
    }

    readByte() {
        return this.data.subarray(
            this.pointer,
            ++this.pointer
        )[0];
    }

    writeBool( bool ) {
        this.writeByte( 
            bool ? 0x01 : 0x00
        )
    }

    readBool( bool ){
        return readByte() == 0x01;
    }

    writeInt16LE( int ) {
        const buf = Buffer.alloc( 2 );
        buf.writeInt16LE( int );

        this.writeBytes( buf )
    }

    readInt16LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 2
        ).readInt16LE();

        this.pointer += 2;
        return res;
    }

    writeUInt16LE( int ) {
        const buf = Buffer.alloc( 2 );
        buf.writeUInt16LE( int );

        this.writeBytes( buf );
    }

    readUInt16LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 2
        ).readUInt16LE();

        this.pointer += 2;
        return res;
    }

    writeInt16BE( int ) {
        const buf = Buffer.alloc( 2 );
        buf.writeIntBELE( int );

        this.writeBytes( buf )
    }

    readInt16BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 2
        ).readInt16BE();

        this.pointer += 2;
        return res;
    }

    writeUInt16BE( int ) {
        const buf = Buffer.alloc( 2 );
        buf.writeUInt16BE( int );

        this.writeBytes( buf );
    }

    readUInt16BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 2
        ).readUInt16BE();

        this.pointer += 2;
        return res;
    }

    writeInt32LE( int ) {
        const buf = Buffer.alloc( 4 );
        buf.writeInt32LE( int );

        this.writeBytes( int );
    }

    readInt32LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 4
        ).readInt32LE();

        this.pointer += 4;
        return res;
    }

    writeUInt32LE( int ) {
        const buf = Buffer.alloc( 4 );
        buf.writeUInt32LE( int );

        this.writeBytes( buf );
    }

    readUInt32LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 4
        ).readUInt32LE();

        this.pointer += 4;
        return res;
    }

    writeInt32BE( int ) {
        const buf = Buffer.alloc( 4 );
        buf.writeInt32BE( int );

        this.writeBytes( int );
    }

    readInt32BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 2
        ).readInt16LE();

        this.pointer += 2;
        return res;
    }

    writeUInt32BE() {
        const buf = Buffer.alloc( 4 );
        buf.writeUInt32BE( int );

        this.writeBytes( buf );
    }

    readUInt32BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 4
        ).readUInt32BE();

        this.pointer += 4;
        return res;
    }

    writeInt64LE( int ) {
        const buf = Buffer.alloc( 8 );
        buf.writeBigInt64LE( 
            BigInt(int) 
        );

        this.writeBytes( int );
    }

    readInt64LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readBigInt64LE();

        this.pointer += 8;
        return res;
    }

    writeUInt64LE( int ) {
        const buf = Buffer.alloc( 8 );
        buf.writeBigUInt64LE( 
            BigInt(int)
        );

        this.writeBytes( buf );
    }

    readUInt64LE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readBigUInt64LE();

        this.pointer += 8;
        return res;
    }

    writeInt64BE( int ) {
        const buf = Buffer.alloc( 8 );
        buf.writeBigInt64BE( 
            BigInt(int)
        );

        this.writeBytes( int );
    }

    readInt64BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readBigInt64BE();

        this.pointer += 8;
        return res;
    }


    writeUInt64BE( int ) {
        const buf = Buffer.alloc( 8 );
        buf.writeBigUInt64BE( 
            BigInt(int) 
        );

        this.writeBytes( buf );
    }

    readUInt64BE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readBigUInt64BE();

        this.pointer += 8;
        return res;
    }

    writeString8( str ) {
        this.writeByte( str.length );
        this.writeBytes( str );
    }
    
    readString8() {
        const length = this.readByte();

        const out = this.data.subarray(
            this.pointer,
            this.pointer + length
        )
        this.pointer += length;

        return out.toString();
    }

    writeString16( str ) {
        this.writeUInt16LE( str.length );
        this.writeBytes( str );
    }
    
    readString16() {
        const length = this.readUInt16LE();

        const out = this.data.subarray(
            this.pointer,
            this.pointer + length
        )
        this.pointer += length;

        return out.toString();
    }

    writeString32( str ) {
        this.writeUInt32LE( str.length );
        this.writeBytes( str );
    }
    
    readString32() {
        const length = this.readUInt32LE();
        const out = this.data.subarray(
            this.pointer,
            this.pointer + length
        )
        this.pointer += length;

        return out.toString();
    }

    writeString64( str ) {
        this.writeBigUInt64LE( str.length );
        this.writeBytes( str );
    }
    
    readString64() {
        const length = this.readUInt64LE();

        const out = this.data.subarray(
            this.pointer,
            this.pointer + length
        )
        this.pointer += length;

        return out.toString();
    }
    
    writeVector3Int8( vector ) {
        this.writeByte( vector.X );
        this.writeByte( vector.Y );
        this.writeByte( vector.Z );
    }

    readVector3Int8() {
        return new Vector3(
            this.readByte(),
            this.readByte(),
            this.readByte()
        )
    }

    writeVector3Int16( vector ) {
        this.writeInt16LE( vector.x );
        this.writeInt16LE( vector.y );
        this.writeInt16LE( vector.z );
    }

    readVector3Int16() {
        return new Vector3(
            this.readInt16LE(),
            this.readInt16LE(),
            this.readInt16LE()
        )
    }

    writeVector3Int32( vector ) {
        this.writeInt32LE( vector.x );
        this.writeInt32LE( vector.y );
        this.writeInt32LE( vector.z );
    }

    readVector3Int32() {
        return new Vector3(
            this.readInt32LE(),
            this.readInt32LE(),
            this.readInt32LE()
        )
    }

    writeVector3Int64( vector ) {
        this.writeInt64LE( vector.x );
        this.writeInt64LE( vector.y );
        this.writeInt64LE( vector.z );
    }

    readVector3Int64() {
        return new Vector3(
            this.readInt64LE(),
            this.readInt64LE(),
            this.readInt64LE()
        )
    }

    writeVector3Float( vector ) {
        this.writeFloatLE( vector.x );
        this.writeFloatLE( vector.y );
        this.writeFloatLE( vector.z );
    }

    readVector3Float() {
        return new Vector3(
            this.readFloatLE(),
            this.readFloatLE(),
            this.readFloatLE()
        )
    }

    ignoreBytes( numBytes ) {
        this.pointer += numBytes;
    }

    writeFloatLE( float ) {
        const buf = Buffer.alloc( 4 );
        buf.writeFloatLE( float );

        this.writeBytes( buf );
    }

    readFloatLE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 4
        ).readFloatLE();

        this.pointer += 4;
        return res;
    }

    writeFloatBE( float ) {
        const buf = Buffer.alloc( 4 );
        buf.writeFloatLE( float );

        this.writeBytes( buf );
    }

    readFloatBE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 4
        ).readFloatLE();

        this.pointer += 4;
        return res;
    }

    writeDoubleLE( double ) {
        const buf = Buffer.alloc( 8 );
        buf.writeDoubleLE( double );

        this.writeBytes( buf );
    }

    readDoubleLE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readDoubleLE();

        this.pointer += 8;
        return res;
    }

    writeDoubleBE( double ) {
        const buf = Buffer.alloc( 8 );
        buf.writeDoubleBE( double );

        this.writeBytes( buf );
    }

    readDoubleBE() {
        const res = this.data.subarray(
            this.pointer,
            this.pointer + 8
        ).readDoubleBE();

        this.pointer += 8;
        return res;
    }

}
