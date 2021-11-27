use std::borrow::BorrowMut;
use std::collections::HashMap;
use actix_web::HttpResponse;
use actix_web::{ get, post };
use actix_web::Either::B;
use actix_web::web::{Form, Json, Path, Query};
use chrono::Local;
use crate::entity::{BlogDetails, BlogGroup, BlogInfo};
use crate::util::{html, head, PageParams, Paging, Results, date_utils};
use crate::util::tera::TeraEntity;

#[get("/admin/blog")]
pub async fn blog() -> HttpResponse  {
    html(TeraEntity::render("admin/blog/blog", &head!()).unwrap())
}

#[get("/admin/blog/edit/{id}")]
pub async fn edit_blog(id: Path<isize>) -> HttpResponse  {
    let mut context = head!();
    context.insert("blog_id", &id.0);
    html(TeraEntity::render("admin/blog/editBlog", &context).unwrap())
}

#[get("/admin/blog/add")]
pub async fn blog_add() -> HttpResponse  {
    let all = BlogGroup::query_all().await;
    let mut v: Vec<BlogGroup> = vec![];
    if all.is_err() {
        log::error!("查询所有用户异常, 异常信息为 {} ", all.err().unwrap());
    } else {
        v = all.unwrap();
    }

    let mut context = head!();
    context.insert("group", &v);

    html(TeraEntity::render("admin/blog/add", &context).unwrap())
}

#[get("/admin/blog/list")]
pub async fn blog_list(params: Query<PageParams>) -> Json<Paging<BlogInfo>> {
    let all = BlogInfo::query_all().await;
    if all.is_err() {
        log::error!("查询所有博客异常, 异常信息为 {} ", all.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        let paging: Paging<BlogInfo> = Paging {
            rows: vec![],
            total: 0
        };
        return Json(paging)
    }

    let all = all.unwrap();

    let page_size = params.page_size.unwrap();
    let page_num = params.page_number.unwrap();
    let paging = Paging::new(page_num, page_size, &all);
    Json(paging)
}


#[get("/admin/blog/del/{id}")]
pub async fn blog_del(id: Path<String>) -> Json<Results<String>> {
    BlogInfo::delete(id.0).await;
    Json(Results::success("删除成功!", String::new()))
}

#[get("/admin/blog/published/{id}")]
pub async fn published(id: Path<usize>) -> Json<Results<String>> {
    let result = BlogDetails::query_by_blog_info_id(id.0).await;
    if result.is_err() {
        log::error!("查询博客详情, 异常信息为 {} ", result.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return Json(Results::error("查询博客详情", String::new()));
    }
    let result = result.unwrap();

    match result {
        None => return Json(Results::error("未编辑文章,请编辑文章后发表博客!", String::new())),
        Some(_) => {}
    }

    BlogInfo::published(id.0).await;

    Json(Results::success("发表成功!", String::new()))
}

#[post("/admin/blog/save")]
pub async fn blog_save(mut params: Form<BlogInfo>) -> Json<Results<String>> {
    let mut info = params.0;
    println!("{:?}", info);
    info.user_account = Some(String::from("gxk"));
    info.read_count = Some(0);
    info.is_publish = Some(String::from("0"));
    info.create_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    info.update_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    let save_result = info.save().await;
    if save_result.is_err() {
        log::error!("保存失败, 异常信息为 {} ", save_result.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return Json(Results::error("保存失败, 请联系管理员!", String::new()))
    }
    Json(Results::success("成功!", String::new()))
}

#[post("/admin/blog/saveDetails")]
pub async fn save_details(mut params: Form<BlogDetails>) -> Json<Results<String>> {
    let details = params.0.borrow_mut();
    details.create_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    details.update_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    let result = details.save().await;
    if result.is_err() {
        log::error!("保存失败, 异常信息为 {} ", result.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return Json(Results::error("保存失败, 请联系管理员!", String::new()))
    }
    Json(Results::success("成功!", String::new()))
}
