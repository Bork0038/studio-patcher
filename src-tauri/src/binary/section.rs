use object::pe::ImageSectionHeader;

pub struct Section {
    pub header: ImageSectionHeader,
    pub data: Vec<u8>
}

impl Section {

    pub fn new<D: AsRef<[u8]>>( header: ImageSectionHeader, data: D ) -> Self {
        Section {
            header,
            data: data.as_ref().to_vec()
        }
    }

    pub fn set_data<D: AsRef<[u8]>>( &mut self, data: D ) {
        self.data = data.as_ref().to_vec();
    }

    pub fn get_name( &mut self ) -> String {
        String::from_utf8(
            self.header.name.to_vec()
        ).map_or( 
            String::new(), 
            | s | String::from( s.trim_end_matches("\0") )  
        )
    }

}