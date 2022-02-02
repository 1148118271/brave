use actix_multipart::Multipart;
use actix_web::{get, post, HttpResponse};
use actix_web::test::config;
use actix_web::web::{Form, Json, Path, Query};
use chrono::Local;
use crate::entity::BlogFiles;
use crate::util::{date_utils, head, html_err, html, PageParams, Paging, Results, config};
use crate::util::tera::TeraEntity;


#[get("/admin/file")]
pub async fn file() -> HttpResponse {
    match TeraEntity::render("admin/file/file", &head!()) {
        Ok(v) => html(v),
        Err(_) => html_err()
    }
}


#[get("/admin/file/fileList")]
pub async fn file_list(params: Query<PageParams>) -> Json<Paging<BlogFiles>> {
    let all = BlogFiles::query_all().await;
    if all.is_err() {
        log::error!("查询所有文件异常, 异常信息为 {} ", all.err().unwrap());
        let paging: Paging<BlogFiles> = Paging {
            rows: vec![],
            total: 0
        };
        return Json(paging)
    }

    let all = all.unwrap();

    let page_size = params.page_size.unwrap();
    let page_num = params.page_number.unwrap();
    let mut paging = Paging::new(page_num, page_size, &all);

    let config = config::this();

    for x in &mut paging.rows {
        if let Some(v) = &x.file_url {
            x.file_url = Some(format!("{}{}", config.domain_name, v))
        }
    }

    Json(paging)
}

#[post("/admin/file/save")]
pub async fn file_save(mut params: Form<BlogFiles>) -> Json<Results<String>> {

    if params.file_url.is_none() {
        log::error!("保存失败, file url 为空!");
        return Json(Results::error("保存失败, file url 为空!", String::new()))
    }

    params.upload_time = Some(date_utils::DateTimeUtil::from(Local::now()));

    if let Err(e) = params.save().await {
        log::error!("保存失败, 异常信息为 {} ", e);
        return Json(Results::error("保存失败, 请联系管理员!", String::new()))
    }

    Json(Results::success("成功!", String::new()))
}

#[get("/admin/file/del/{id}")]
pub async fn file_del(id: Path<String>) -> Json<Results<String>> {
    BlogFiles::delete(id.0).await;
    Json(Results::success("删除成功!", String::new()))
}