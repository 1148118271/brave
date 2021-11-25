use std::cell::RefCell;
use std::collections::HashMap;

static mut VALUE: Option<RefCell<HashMap<String, String>>> = None;


pub fn put(k: String, v: String) -> Option<String> {
    unsafe {
        match &VALUE {
            None => {
                let mut map = HashMap::new();
                let option = map.insert(k, v);
                VALUE = Some(RefCell::new(map));
                option
            }
            Some(map) => {
                let mut ref_mut = map.borrow_mut();
                ref_mut.insert(k, v)
            }
        }
    }
}

pub fn get(k: &str) -> Option<String> {
    unsafe{
        match &VALUE {
            None => None,
            Some(v) => {
                let x = v.borrow();
                match x.get(k) {
                    None => None,
                    Some(s) => Some(s.to_owned())
                }

            }
        }
    }
}