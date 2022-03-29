use std::env;
use std::path::Path;

static mut PATH: Option<String> = None;

pub fn default() -> &'static String {
    unsafe {
        if PATH.is_none() {
            let args: Vec<String> = env::args().collect();
            let path;
            if args.len() >= 2 {
                path = Path::new(args[1].as_str())
            } else {
                path = Path::new(".");
            }
            let buf = path.canonicalize().expect("路径加载异常！");
            let x = buf.into_os_string().into_string().unwrap();
            PATH = Some(x)
        }
        if let Some(v) = &PATH {
            return v;
        }
        panic!("路径加载异常！")
    }
}
