mod pattern;

pub use pattern::{ CodePat, IDAPat, Pattern };
use super::Binary;

impl Binary {

    pub fn scan<P: Pattern, S: Into<String>>( &mut self, pattern: &P, section: Option<S> ) -> Option<usize> {
        let name = match section {
            Some(section) => section.into(),
            None => return None
        };

        if let Some(section) = self.get_section_by_name( name ) {
            let data = &section.data;
            let len = data.len();

            for i in 0..len {
                let mut found = true;

                for j in 0..pattern.get_len() {
                    if i + j >= len {
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
        }

        None
    }

}