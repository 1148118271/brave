use actix_files::Files;
use actix_web::{App, HttpServer};

use crate::methods;
use crate::util::{path, config, auth};

pub async fn init() -> std::io::Result<()> {
    HttpServer::new(|| {
        let p = path::this();
        let conf = config::this();
        let static_path = format!("{}/static", p);
        let favicon = format!("{}/static/img/favicon.ico", p);
        App::new()
            .wrap(auth::Auth)
            .service(Files::new("static/", &static_path))
            .service(Files::new("files/", &conf.file_upload_path))
            .service(Files::new("favicon.ico", &favicon))
            .service(methods::blog::index)
            .service(methods::blog::details)
            .service(methods::blog::comment)
            .service(methods::blog::comment_save)
            .service(methods::file::upload)
            .service(methods::admin::login::admin)
            .service(methods::admin::login::login)
            .service(methods::admin::login::index)
            .service(methods::admin::login::main)
            .service(methods::admin::user::user)
            .service(methods::admin::user::user_list)
            .service(methods::admin::user::user_add)
            .service(methods::admin::user::user_save)
            .service(methods::admin::blog::blog)
            .service(methods::admin::blog::blog_list)
            .service(methods::admin::blog::blog_add)
            .service(methods::admin::blog::blog_save)
            .service(methods::admin::blog::blog_del)
            .service(methods::admin::blog::edit_blog)
            .service(methods::admin::blog::save_details)
            .service(methods::admin::blog::published)
    }).bind(("0.0.0.0", config::this().port))?.run().await
}