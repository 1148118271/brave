use chrono::Local;
use rbatis::crud::CRUD;
use rbatis::{crud_table, py_sql, Page, PageRequest};
use rbatis::db::DBExecResult;
use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;
use serde:: {
    Deserialize,
    Serialize,
};
use crate::mysql;
use crate::util::date_utils;

#[crud_table(table_columns:"id, title, user_account, publish_time, label_key, read_count, is_publish, create_time, update_time")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogInfo {
    // 主键
    pub id:                 Option<usize>,
    // 标题
    pub title:              Option<String>,
    // 用户账号
    pub user_account:       Option<String>,
    // 用户名称
    pub user_name:          Option<String>,
    // 发布时间
    pub publish_time:       Option<date_utils::DateTimeUtil>,
    // 标签key
    pub label_key:           Option<String>,
    // 阅读次数
    pub read_count:         Option<u32>,
    // 是否发布 0 未发布, 1 已发布
    pub is_publish:         Option<String>,
    pub create_time:        Option<date_utils::DateTimeUtil>,
    pub update_time:        Option<date_utils::DateTimeUtil>
}


#[crud_table(table_columns:"`year`, `count`")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Archive {
    year: usize,
    count: u8
}

impl BlogInfo {

    pub async fn query_hot() -> Vec<Self> {
        let rb = mysql::default().await;

        let page = PageRequest::new(1, 3);

        let wrapper = rb.new_wrapper()
            .eq("is_publish", "1")
            .order_by(false, &["read_count"]);

        let result: rbatis::Result<Page<Self>> = rb.fetch_page_by_wrapper(wrapper, &page).await;

        match result {
            Ok(v) => v.records,
            Err(e) => {
                log::error!("查询热门文章异常, 异常信息为: {}", e);
                Vec::new()
            }
        }
    }


    pub async fn archive() -> Vec<Archive> {
        let rb = mysql::default().await;
        let v: rbatis::Result<Vec<Archive>> = rb.fetch(r#"
        select
        year(publish_time) as "year",
        count(publish_time) as "count"
        from
        blog_info
        group by YEAR(publish_time)
        order by year(publish_time) desc"#,
                         vec![]).await;
        match v {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询归档信息异常, 异常信息为: {}", e);
                Vec::new()
            }
        }
    }

    pub async fn query_page(page_no: u64) -> Option<Page<Self>> {
        let rb = mysql::default().await;
        let page = PageRequest::new(page_no, 5);
        let w = rb.new_wrapper()
            .eq("is_publish","1");
        let result: rbatis::Result<Page<Self>> = rb.fetch_page_by_wrapper(w, &page).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("根据博客信息异常, 异常信息为: {}", e);
                None
            }
        }
    }
}
