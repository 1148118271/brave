mod methods;
mod util;
mod entity;
mod path;
mod config;
mod mysql;
mod tera;

#[actix_web::main]
async fn main() {
    log::info!("开始加载项目, 时间为:[{}]...", util::get_date_time());
    let p = path::default();
    let path = format!("{}/conf/log4rs.yaml", p);
    log4rs::init_file(path, Default::default()).unwrap();
    methods::init().await.expect("方法初始化时异常！");
}