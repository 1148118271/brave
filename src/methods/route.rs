use actix_files::Files;
use actix_web::{App, HttpServer};

use crate::{config, methods, path};
use crate::util::auth;

pub async fn init() -> std::io::Result<()> {
    HttpServer::new(|| {
        let p = path::default();
        let conf = config::default();
        let static_path = format!("{}/static", p);
        // let favicon = format!("{}/static/img/favicon.ico", p);
        App::new()
            .wrap(auth::Auth)
            .service(Files::new("static/", &static_path))
            .service(Files::new("files/", &conf.file_upload_path))
            //.service(Files::new("favicon.ico", &favicon))
            .service(methods::blog::index)
            // .service(methods::blog::details)
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
    }).bind(("0.0.0.0", config::default().port))?.run().await
}