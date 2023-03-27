pub trait Pattern {
    fn scan( &self, byte: u8, pos: usize ) -> bool;
    fn get_len( &self ) -> usize;
}

pub struct PatternByte {
    pub byte: Option<char>,
    pub pos: usize
}

pub struct IDAPat {
    pub pattern: &'static str,
    pub bytes: Vec<PatternByte>
}

impl IDAPat {

    pub fn new( pattern: &'static str ) -> Self {
        let mut vec = Vec::new();
        let sections = pattern.split(" ");

        let mut i = 0;
        for section in sections {
            if section == "?" || section == "??" {
                vec.push(
                    PatternByte { byte: None, pos: i }
                )
            } else {
                let byte = u8::from_str_radix( section, 16 ).unwrap();
                vec.push(
                    PatternByte { 
                        byte: Some(byte as char), 
                        pos: i 
                    }
                )
            }

            i += 1;
        }
        
        IDAPat {
            bytes: vec,
            pattern
        }
    }
}

pub struct CodePat {
    pub pattern: &'static str,
    pub mask: &'static str,
    pub bytes: Vec<PatternByte>
}

impl CodePat {

    pub fn new(pattern: &'static str, mask: &'static str) -> Self {
        let bytes: Vec<PatternByte> = pattern
            .chars()
            .zip(mask.chars())
            .enumerate()
            .map(|(i,(pattern, mask))| PatternByte {
                byte: match mask {
                    'x' => Some(pattern),
                    _ => None,
                },
                pos: i,
            })
            .collect();

        CodePat {
            pattern,
            mask,
            bytes,
        }
    }
    
}

impl Pattern for CodePat {

    fn get_len( &self ) -> usize {
        self.bytes.len()
    }

    fn scan( &self, data: u8, pos: usize ) -> bool {
        let byte = match self.bytes.get(pos) {
            Some(byte) => byte,
            None => return false
        };

        if let Some(b) = byte.byte {
            b == data as char && pos == byte.pos
        } else {
            true
        }
    }

}

impl Pattern for IDAPat {

    fn get_len( &self ) -> usize {
        self.bytes.len()
    }

    fn scan( &self, data: u8, pos: usize ) -> bool {
        let byte = match self.bytes.get(pos) {
            Some(byte) => byte,
            None => return false
        };

        if let Some(b) = byte.byte {
            b == data as char && pos == byte.pos
        } else {
            true
        }
    }

}