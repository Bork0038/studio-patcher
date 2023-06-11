use num_traits::NumCast;
use base64_url::{ encode, decode };

#[derive(Clone)]
pub struct NetworkStream {
    data: Vec<u8>,
    pub read_pointer: u32
}

pub trait ReadWrite {
    fn from_le_bytes(bytes: Vec<u8>) -> Self;
    fn from_be_bytes(bytes: Vec<u8>) -> Self;
    fn to_le_bytes( self ) -> Vec<u8>;
    fn to_be_bytes( self ) -> Vec<u8>;
}

macro_rules! impl_Read_Write (( $($int:ident),* ) => {
    $(
        impl ReadWrite for $int {
            fn from_le_bytes( bytes: Vec<u8> ) -> Self { 
                Self::from_le_bytes(
                    bytes
                        .as_slice()
                        .try_into()
                        .unwrap()
                ) 
            }

            fn from_be_bytes( bytes:  Vec<u8> ) -> Self {
                Self::from_be_bytes(  
                    bytes
                        .as_slice()
                        .try_into()
                        .unwrap()
                ) 
            }

            fn to_le_bytes( self ) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }

            fn to_be_bytes( self ) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    )*
});

impl_Read_Write!( usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64 );

pub trait Serializable<T> {
    fn write( &mut self, stream: &mut NetworkStream );
    fn read( stream: &mut NetworkStream ) -> T;
}

impl NetworkStream {
    
    pub fn clone( &mut self ) -> NetworkStream {
        NetworkStream { 
            data: self.data.clone(),
            read_pointer: self.read_pointer
        }
    }

    pub fn new() -> NetworkStream {
        NetworkStream { 
            data: Vec::new(), 
            read_pointer: 0 
        }
    }

    pub fn from( s: Vec<u8> ) -> NetworkStream {
        NetworkStream { 
            data: s,
            read_pointer: 0 
        }
    }

    pub fn from_encoded( s: String ) -> NetworkStream {
        NetworkStream {
            data: decode( &s ).unwrap(),
            read_pointer: 0
        }
    }

    pub fn encode( &mut self ) -> String {
        encode( &self.get_data() )
    }


    pub fn get_data( &mut self ) -> Vec<u8> {
        self.data.clone()
    }

    pub fn ignore_bytes( &mut self, num_bytes: u32 ) {
        self.read_pointer += num_bytes;
    }

    pub fn write_bytes( &mut self, mut bytes: Vec<u8> ) {
        self.data.append( &mut bytes );
    }

    pub fn read_bytes( &mut self, num_bytes: u32 ) -> Vec<u8> {
        let slice = &self.data[ self.read_pointer as usize..(self.read_pointer + num_bytes) as usize];
        self.read_pointer += num_bytes;
       
        slice.to_vec()
    }
    
    pub fn write_byte( &mut self, int: u8 ) {
        self.data.append( &mut vec![ int ] );
    }

    pub fn read_byte( &mut self ) -> u8 {
        self.read_pointer += 1;
        self.data[ ( self.read_pointer - 1 ) as usize ]
    }

    pub fn write_bool( &mut self, b: bool ) {
        self.write_byte(
            if b { 0x01 } else { 0x00 }
        );
    }

    pub fn read_bool( &mut self ) -> bool {
        self.read_byte() == 0x01
    }

    pub fn read_le<T>( &mut self ) -> T
    where T: ReadWrite {
        T::from_le_bytes(
            self.read_bytes( std::mem::size_of::<T>() as u32 )
        )
    }

    pub fn read_be<T>( &mut self ) -> T
    where T: ReadWrite {
        T::from_be_bytes(
            self.read_bytes( std::mem::size_of::<T>() as u32 )
        )
    }

    pub fn write_le<T>( &mut self, val: T )
    where T: ReadWrite {
        self.write_bytes(
            T::to_le_bytes( val )
        );
    }

    pub fn write_be<T>( &mut self, val: T )
    where T: ReadWrite {
        self.write_bytes(
            T::to_be_bytes( val )
        );
    }

    pub fn write_string<T>( &mut self, s: &str )
    where T: ReadWrite + NumCast {
        self.write_le::<T>( T::from( s.len() ).unwrap() );

        self.write_bytes( s.as_bytes().to_vec() );
    }

    pub fn read_string<T>( &mut self ) -> String
    where T: ReadWrite + NumCast {
        let len = self.read_le::<T>();

        String::from_utf8(
            self.read_bytes( <u32 as NumCast>::from( len ).unwrap() )
        ).unwrap()
    }

    pub fn write_string_be<T>( &mut self, s: &str )
    where T: ReadWrite + NumCast {
        self.write_be::<T>( T::from( s.len() ).unwrap() );

        self.write_bytes( s.as_bytes().to_vec() );
    }

    pub fn read_string_be<T>( &mut self ) -> String
    where T: ReadWrite + NumCast {
        let len = self.read_be::<T>();

        String::from_utf8(
            self.read_bytes( <u32 as NumCast>::from( len ).unwrap() )
        ).unwrap()
    }

    pub fn write_string_dyn( &mut self, s: &str ) {
        let len = s.len();
        match len {
            0..=255 => {
                self.write_le::<u8>( 0 );
                self.write_string::<u8>( s );
            }
            256..=65535 => {
                self.write_le::<u8>( 1 );
                self.write_string::<u16>( s );
            }
            65536..=2147483647 => {
                self.write_le::<u8>( 2 );
                self.write_string::<u32>( s );
            }
            _ => { 
                self.write_le::<u8>( 3 );
                self.write_string::<u64>( s );
            }
        }
    }

    pub fn read_string_dyn( &mut self ) -> String {
        let size_t = self.read_byte();

        match size_t {
            0 => self.read_string::<u8>(),
            1 => self.read_string::<u16>(),
            2 => self.read_string::<u32>(),
            _ => self.read_string::<u64>()
        }
    }

    pub fn write<T>( &mut self, mut obj: T ) 
    where T: Serializable<T> {
        obj.write( self );
    }

    pub fn read<T>( &mut self ) -> T 
    where T: Serializable<T> {
        T::read( self )
    }
    
    pub fn read_to_end( &mut self ) -> &[u8] {
        &self.data[ self.read_pointer as usize .. self.data.len() ]
    }

}