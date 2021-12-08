use brave::{methods, util};
use brave::util::{path, tera, config, mysql};

#[actix_web::main]
async fn main() {
    log::info!("开始加载项目, 时间为:[{}]...", util::get_date_time());

    path::init();
    config::init();
    let p = path::this();
    let path = format!("{}/conf/log4rs.yaml", p);
    log4rs::init_file(path, Default::default()).unwrap();
    tera::TeraEntity::init();
    mysql::init().await;
    methods::init().await.unwrap();
}