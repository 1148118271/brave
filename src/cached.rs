use cached_rs::Cached;

static mut CACHED: Option<Cached> = None;

pub fn default() -> &'static mut Cached {
    unsafe {
        if CACHED.is_none() {
            let cached = Cached::connect("127.0.0.1:9200").unwrap();
            CACHED = Some(cached);
        }
        CACHED.as_mut().unwrap()
    }
}


pub const CONF_KEY: &str = "conf";
pub const POST_HTML_KEY: &str = "html";