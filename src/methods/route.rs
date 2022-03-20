use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, web};
use tera::Context;

use crate::{config, html, methods, path};
use crate::util::auth;

pub async fn init() -> std::io::Result<()> {
    HttpServer::new(|| {
        let p = path::default();
        let conf = config::default();
        let static_path = format!("{}/static", p);
        // let favicon = format!("{}/static/img/favicon.ico", p);
        App::new()
           //  .wrap(auth::Auth)
            .service(Files::new("static/", &static_path))
            .service(Files::new("files/", &conf.file_upload_path))
            .service(Files::new("/favicon.ico", "static/images/favicon.ico"))
            //.service(Files::new("favicon.ico", &favicon))
            .service(web::resource("/").to(home))
            .service(web::resource("/about").to(about))
            .service(methods::index::index)
            .service(methods::details::details)
            .service(methods::details::submit_comments)
            .service(methods::timeline::timeline)
            // .service(methods::blog::comment)
            // .service(methods::blog::comment_save)
            // .service(methods::blog::group)
            // .service(methods::blog::friend_links)
            // .service(methods::blog::links_save)
            // .service(methods::blog::about)
            .service(methods::file::upload)
            // .service(methods::admin::login::admin)
            // .service(methods::admin::login::login)
            // .service(methods::admin::login::index)
            // .service(methods::admin::login::main)
            // .service(methods::admin::user::user)
            // .service(methods::admin::user::user_list)
            // .service(methods::admin::user::user_add)
            // .service(methods::admin::user::user_save)
            // .service(methods::admin::blog::blog)
            // .service(methods::admin::blog::blog_list)
            // .service(methods::admin::blog::blog_add)
            // .service(methods::admin::blog::blog_save)
            // .service(methods::admin::blog::blog_del)
            // .service(methods::admin::blog::edit_blog)
            // .service(methods::admin::blog::save_details)
            // .service(methods::admin::blog::published)
            // .service(methods::admin::file::file)
            // .service(methods::admin::file::file_list)
            // .service(methods::admin::file::file_save)
            // .service(methods::admin::file::file_del)
            // .default_service(
            //     web::to(default)
            // )
    }).bind(("0.0.0.0", config::default().port))?.run().await
}

pub async fn default() -> HttpResponse {
    html!{"error/404".to_string(), &Context::new()}
}

pub async fn home() -> HttpResponse {
    html!{"home".to_string(), &Context::new()}
}
pub async fn about() -> HttpResponse {
    html!{"about".to_string(), &Context::new()}
}