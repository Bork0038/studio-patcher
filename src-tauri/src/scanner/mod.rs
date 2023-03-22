mod pattern;

pub use pattern::*;

pub struct Scanner<'a> {
    data: &'a Box<Vec<u8>>
}

impl<'a> Scanner<'a> {

    pub fn new( data: &'a Box<Vec<u8>> ) -> Self {
        Scanner { 
            data 
        }
    }

    pub fn scan<P: Pattern>( &self, pattern: &P ) -> Option<usize> {
        let data = self.data;

        for i in 0..data.len() {
            let mut found = true;

            for j in 0..pattern.get_len() {
                if i + j >= data.len() {
                    found = false;
                    break;
                }

                if !pattern.scan( data[i + j], j ) {
                    found = false;
                    break;
                }
            }

            if found {
                return Some( i as usize );
            }
        }
        None
    }

}