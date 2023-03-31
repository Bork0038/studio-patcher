use std::mem::transmute;

#[repr(C)]
pub struct QArrayData {
    ref_count: u32,
    size: u32,
    alloc: u32,
    capacity: u32,
    offset: u64,
}

impl QArrayData{
    
    pub unsafe fn serialize<'a>( data: &str ) -> Vec<u8> {
        let chars: Vec<u16> = data
            .encode_utf16()
            .collect();

        let len = chars.len() as u32;

        let array_data = QArrayData { 
            ref_count: 0, 
            size: len, 
            alloc: len + 1, 
            capacity: 0xBAADF00D, 
            offset: 0x18, 
        };

        let mut out = Vec::new();
        out.append(
            &mut transmute::<QArrayData, [ u8; 0x18 ]>( array_data ).to_vec()
        );
       
        chars
            .iter()
            .for_each(|char| {
                out.push( (char & 0xFFu16) as u8 );
                out.push( (char >> 8) as u8 ); 
            });

        out.push( 0x00 );

        out
    }

}