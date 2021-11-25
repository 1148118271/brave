use actix_web::{get, post, HttpResponse};
use actix_web::web::Form;
use actix_web::cookie::Cookie;
use tera::Context;
use crate::entity::BlogUser;
use crate::util::{html, Results, head, session};
use crate::util::tera::TeraEntity;

#[get("/admin")]
pub async fn admin() -> HttpResponse {
    html(TeraEntity::render("admin/login", &Context::new()).unwrap())
}


#[get("/admin/index")]
pub async fn index() -> HttpResponse {
    html(TeraEntity::render("admin/index", &head!()).unwrap())
}

#[get("/admin/main")]
pub async fn main() -> HttpResponse {
    html(TeraEntity::render("admin/main", &head!()).unwrap())
}

#[post("/admin/login")]
pub async fn login(data: Form<BlogUser>) -> HttpResponse {
    let mut builder = HttpResponse::Ok();

    let blog_user = data.into_inner();
    let req_password = blog_user.password.unwrap();

    let account = blog_user.account.unwrap();
    let blog_user = BlogUser::query_by_account(&account).await;
    if blog_user.is_err() {
        log::error!("根据账号查询用户信息异常, 异常信息为 {}!", blog_user.err().unwrap());
        return builder.json(Results::error("根据账号查询用户信息异常, 请联系管理员!", ""))
    }

    let blog_user = blog_user.unwrap();

    let blog_user = match blog_user {
        None => return builder.json(Results::error("账号不存在或者密码不正确!", "")),
        Some(b) => b,
    };

    let digest = md5::compute(req_password.as_bytes());
    let d = format!("{:x}", digest);

    if !d.eq(&blog_user.password.unwrap()) {
        return builder.json(Results::error("账号不存在或者密码不正确!", ""))
    }

    let account = blog_user.account.unwrap();

    let token = format!("{:x}", md5::compute(account.as_bytes()));

    session::put(token.to_string(), account.to_owned());
    let mut cookie = Cookie::new("token", &token);
    cookie.set_path("/");

    HttpResponse::Ok().cookie(cookie).json(Results::success("成功!", &account))
}

