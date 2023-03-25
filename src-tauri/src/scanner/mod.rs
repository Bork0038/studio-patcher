mod pattern;

pub use pattern::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Scanner {
    data: Rc<RefCell<Vec<u8>>>
}

impl Scanner {

    pub fn new( data: Rc<RefCell<Vec<u8>>> ) -> Self {
        Scanner { 
            data 
        }
    }

    pub fn scan<P: Pattern>( &self, pattern: &P ) -> Option<usize> {
        let data = self.data.borrow_mut();

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