use actix_web::HttpResponse;
use serde:: {
    Serialize,
    Deserialize,
};
use tera::Context;

use super::paging;

#[derive(Serialize, Deserialize)]
pub struct Results<T> {
    pub code: u16,
    pub msg: String,
    pub data: T
}

impl<T> Results<T> {

    pub fn success(msg: &str, data: T) -> Self {
        Results {
            code: 200,
            msg: msg.to_string(),
            data
        }
    }

    pub fn error(msg: &str, data: T) -> Self {
        Results {
            code: 500,
            msg: msg.to_string(),
            data
        }
    }

}


#[derive(Serialize, Deserialize)]
pub struct Paging<T> {
    pub rows: Vec<T>,
    pub total: usize
}

impl<T: Clone> Paging<T> {
    pub fn new(page_num: isize, page_size: isize, data: &Vec<T>) -> Paging<T> {
        let total = data.len();
        let rows = paging!(page_num, page_size, data);
        Paging {
            rows,
            total
        }
    }
}

#[macro_export]
macro_rules! html {
    ($h:expr,$c:expr) => {
        let hn;
        if !$h.contains(".html") {
            hn = format!("{}.html", $h)
        } else {
            hn = $h
        }
        let t = crate::tera::default();
        let string = t.render(&hn, &$c).unwrap();
        actix_web::HttpResponse::Ok().content_type("text/html").body(string)
    };
}

pub fn html(string: String) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(string)
}

pub fn html_err() -> HttpResponse {
    html!{"error/404".to_string(), &Context::new()}
}