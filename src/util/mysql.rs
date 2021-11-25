use rbatis::rbatis::Rbatis;
use super::config;

static mut RB: Option<Rbatis> = None;

pub async fn init() {
    let conf = config::this();
    unsafe {
        let rbatis = Rbatis::new();
        rbatis.link(&conf.database_url).await.unwrap();
        RB = Some(rbatis);
    };

}

pub fn this() -> &'static Rbatis {
    unsafe {
        match &RB {
            None => panic!("加载数据库异常!"),
            Some(v) => v
        }
    }
}