use num_traits::{ FromPrimitive, ToPrimitive };
use num_derive::{ FromPrimitive, ToPrimitive };
use std::{net::SocketAddrV4, str::FromStr};
use dns_lookup::{ AddrFamily };
use serde::{Deserialize, Serialize};

pub type InPortT = u16;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct In6Addr {
    pub s6_addr: [u8; 16],
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct SockAddrIn6 {
    pub sin6_family: u8,
    pub sin6_port: InPortT,
    pub sin6_flowinfo: u32,
    pub sin6_addr: In6Addr,
    pub sin6_scope_id: u32
}

impl SockAddrIn6 {
    pub fn default() -> Self {
        SockAddrIn6 { 
            sin6_family: AddrFamily::Inet6 as u8, 
            sin6_port: 0, 
            sin6_flowinfo: 0, 
            sin6_addr: In6Addr { s6_addr: [0; 16] }, 
            sin6_scope_id: 0
         }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct InAddr {
    pub s_addr: u32
}

impl InAddr {

    pub fn from_str( str: &str ) -> Self {
        if let Ok(addr) = SocketAddrV4::from_str( str ) {
            InAddr {
                s_addr: u32::from_be_bytes( addr.ip().octets() )
            }
        } else {
            InAddr { 
                s_addr: 0 
            }
        }
    }

}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct SockAddrIn {
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [char; 8]
} 

impl SockAddrIn {
    pub fn default() -> Self {
        SockAddrIn { 
            sin_family: AddrFamily::Inet as u8, 
            sin_port: 0, 
            sin_addr: InAddr { s_addr: 0 },
            sin_zero: ['\0'; 8]
        }
    }
}