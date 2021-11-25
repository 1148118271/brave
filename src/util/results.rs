use actix_web::HttpResponse;
use serde:: {
    Serialize,
    Deserialize,
};

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


pub fn html(string: String) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(string)
}

pub fn html_err() -> HttpResponse {
    let html_str =
        r#"
            <!DOCTYPE html>
            <html lang="zh-CN">
            <head>
                <meta charset="UTF-8">
                <title>页面异常</title>
            </head>
            <body>
                <h1> 访问异常, 请联系管理员! </h1>
            </body>
            </html>

        "#;
    HttpResponse::Ok().content_type("text/html").body(html_str)
}