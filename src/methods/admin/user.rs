use actix_web::HttpResponse;
use actix_web::{ get, post };
use actix_web::web::{Form, Json, Query};
use chrono::Local;

use crate::entity::BlogUser;
use crate::util::{html, Results, tera::TeraEntity, head, Paging, PageParams, date_utils};


#[get("/admin/user")]
pub async fn user() -> HttpResponse  {
    html(TeraEntity::render("admin/user/user", &head!()).unwrap())
}

#[get("/admin/user/add")]
pub async fn user_add() -> HttpResponse  {
    html(TeraEntity::render("admin/user/add", &head!()).unwrap())
}


#[get("/admin/user/list")]
pub async fn user_list(params: Query<PageParams>) -> Json<Paging<BlogUser>> {
    let all = BlogUser::query_all().await;
    if all.is_err() {
        log::error!("查询所有用户异常, 异常信息为 {} ", all.err().unwrap());
        let paging: Paging<BlogUser> = Paging {
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

#[post("/admin/user/save")]
pub async fn user_save(mut params: Form<BlogUser>) -> Json<Results<String>> {
    let option = params.password.clone();
    let digest = md5::compute(option.unwrap_or("123456".to_string()).as_bytes());
    params.password = Some(format!("{:x}", digest));
    params.create_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    params.update_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    params.del = Some(1);
    let save_result = params.save().await;
    if save_result.is_err() {
        log::error!("保存失败, 异常信息为 {} ", save_result.err().unwrap());
        return Json(Results::error("保存失败, 请联系管理员!", String::new()))
    }
    Json(Results::success("成功!", String::new()))
}