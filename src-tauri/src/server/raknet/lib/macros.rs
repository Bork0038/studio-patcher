#[allow(unused_macros)]
#[allow(unused_imports)]
pub mod Macros {
    macro_rules! htons {
        ($a:literal) => {
            ($a as u16).to_be()
        };
        ($a:ident) => {
            ($a as u16).to_be()
        }
    }

    macro_rules! ntohs {
        ($a:literal) => {
            u16::from_be($a)
        };
        ($a:ident) => {
            u16::from_be($a)
        }
    }
    
    macro_rules! htonl {
        ($a:literal) => {
            ($a as u32).to_be()
        };
        ($a:ident) => {
            ($a as u32).to_be()
        }
    }

    macro_rules! ntohl {
        ($a:literal) => {
            u32::from_be($a)
        };
        ($a:ident) => {
            u32::from_be($a)
        }   
    }

    pub(crate) use htons;
    pub(crate) use ntohs;
    pub(crate) use htonl;
    pub(crate) use ntohl;
}
