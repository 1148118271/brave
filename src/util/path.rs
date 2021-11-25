use std::path::Path;

static mut PATH: Option<String> = None;

pub fn init() {
    let path = Path::new("./");
    let buf = path.canonicalize().unwrap();
    let x = buf.to_str().unwrap();
    unsafe { PATH = Some(x.to_string()) }
}

pub fn this() -> &'static String {
    unsafe {
        match &PATH {
            None => panic!("加载数据库异常!"),
            Some(v) => v
        }
    }
}