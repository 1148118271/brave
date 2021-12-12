use std::env;
use std::path::Path;

static mut PATH: Option<String> = None;

pub fn init() {
    let args: Vec<String> = env::args().collect();
    let mut path = Path::new("./");
    if args.len() >= 2 {
        unsafe { path = Path::new(args[1].as_str()) }
    }
    let buf = path.canonicalize().unwrap();
    let x = buf.to_str().unwrap();
    unsafe { PATH = Some(x.to_string()) }
}

pub fn this() -> &'static String {
    unsafe {
        match &PATH {
            None => panic!("默认配置路径加载异常!"),
            Some(v) => v
        }
    }
}