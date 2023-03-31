mod pattern;

pub use pattern::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Scanner {
    data: Vec<u8>
}

impl Scanner {
    pub fn new( data: Rc<RefCell<Vec<u8>>> ) -> Self {
        Scanner { 
            data: data.borrow().clone()
        }
    }

    pub fn scan<P: Pattern>( &self, pattern: &P ) -> Option<usize> {
        for i in 0..self.data.len() {
            let mut found = true;

            for j in 0..pattern.get_len() {
                if i + j >= self.data.len() {
                    found = false;
                    break;
                }

                if !pattern.scan( self.data[i + j], j ) {
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