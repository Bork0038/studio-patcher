use dns_lookup::AddrFamily;
use super::super::{ structs::{ InAddr, SockAddrIn }, Macros::{ ntohs, htons } };
use std::{net::{SocketAddrV4, Ipv4Addr}, str::FromStr};
use crate::stream::{ NetworkStream, Serializable };
use super::SystemIndex;

pub struct SystemAddress {
    pub address: SockAddrIn,
    pub debug_port: u16,
    pub system_index: SystemIndex
}

impl SystemAddress {

    pub fn new( addr: Option<&str>, port: Option<u16> ) -> Self {
        if let Some(addr) = addr {
            SystemAddress { 
                address: SockAddrIn {
                    sin_addr: InAddr::from_str( addr ),
                    sin_family: AddrFamily::Inet as u8,
                    sin_port: if let Some(port) = port { htons!( port ) } else { 0 },
                    sin_zero: ['\0'; 8]
                }, 
                debug_port: if let Some(port) = port { port } else { 1 }, // -65535 
                system_index: 65535 // -1, 
            }
        } else {
            SystemAddress { 
                address: SockAddrIn::default(), 
                debug_port: 1, // -65535 
                system_index: 65535 // -1, 
            }
        }
    }

    pub fn set_port( &mut self, port: u16 ) {
        self.address.sin_port = htons!( port );
        self.debug_port = port;
    }

    pub fn from_string( &mut self, str: &str ) -> Result<(), ()> {
        if let Ok(addr) = SocketAddrV4::from_str( str ) {
            let port = addr.port();

            self.address.sin_addr.s_addr = u32::from_be_bytes( addr.ip().octets() );
            self.address.sin_port = port;
            self.debug_port = ntohs!( port );

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn to_string( &mut self ) -> String {
        let addr = Ipv4Addr::from( self.address.sin_addr.s_addr );

        format!("({}) {}:{}", self.system_index, addr.to_string(), self.address.sin_port )
    }

    pub fn from_explicit_port( &mut self, str: &str, port: u16 ) {
        if let Ok(_) = self.from_string( str ) {
            self.set_port( port );
        }
    }

}

impl Serializable<SystemAddress> for SystemAddress {

    fn read( stream: &mut NetworkStream ) -> SystemAddress {
        let debug_port: u16 = stream.read_le();
        let system_index: SystemIndex = stream.read_le();

        let port: u16 = stream.read_le();
        let addr = stream.read_string::<u32>();

        let mut system_address = SystemAddress::new( Some( &addr ), Some( port ));
        system_address.debug_port = debug_port;
        system_address.system_index = system_index;

        system_address
    }

    fn write( &mut self, stream: &mut NetworkStream ) {
        stream.write_le( self.debug_port );
        stream.write_le( self.system_index );
        stream.write_le( self.address.sin_port );
        
        let addr = Ipv4Addr::from( 
            self.address.sin_addr.s_addr
        );
        stream.write_string::<u32>( &addr.to_string() )
    }

}