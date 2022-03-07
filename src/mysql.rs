use rbatis::rbatis::Rbatis;
use crate::config;

static mut RB: Option<Rbatis> = None;

pub async fn default() -> &'static Rbatis {
    unsafe {
        if RB.is_none() {
            let conf = config::default();
            let rbatis = Rbatis::new();
            rbatis.link(&conf.database_url).await.expect("数据库连接异常！");
            RB = Some(rbatis);
        }

        if let Some(v) = &RB {
            return v;
        }

        panic!("数据库连接异常！")
    }
}