use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, web};
use tera::Context;

use crate::{config, html, methods, os_path, path};

pub async fn init() -> std::io::Result<()> {
    HttpServer::new(|| {
        let p = path::default();
        let conf = config::default();
        let static_path = os_path!(p, "static");
        App::new()
            .service(Files::new("static/", static_path))
            .service(Files::new("files/", &conf.file_upload_path))
            .service(methods::base::base_conf)
            .service(methods::index::index)
            .service(methods::post::post)
            .service(methods::post::submit_comments)
            .service(methods::timeline::timeline)
            .service(methods::links::links)
            .service(methods::about::about)
            .default_service(web::to(default))
    }).bind(("0.0.0.0", config::default().port))?.run().await
}

pub async fn default() -> HttpResponse {
    html!{"error/404".to_string(), &Context::new()}
}