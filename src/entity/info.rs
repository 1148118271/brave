use bson::Bson;
use rbatis::{crud_table, Page, PageRequest};
use rbatis::crud::CRUD;
use rbatis::executor::Executor;
use serde:: {
    Deserialize,
    Serialize,
};

use crate::mysql;
use crate::util::date_utils;

#[crud_table(table_columns:"id, title, publish_time, label_key, read_count, is_publish, create_time, update_time")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogInfo {
    // 主键
    pub id:                 Option<usize>,
    // 标题
    pub title:              Option<String>,
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


impl BlogInfo {

    pub async fn query_all() -> Vec<Self> {
        let rb = mysql::default().await;
        let wrapper = rb.new_wrapper()
            .eq("is_publish","1")
            .order_by(false, &["publish_time"]);
        let result: rbatis::Result<Vec<BlogInfo>> = rb.fetch_list_by_wrapper(wrapper).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("根据博客信息异常, 异常信息为: {}", e);
                vec![]
            }
        }
    }

    pub async fn query_page(page_no: u64) -> Option<Page<Self>> {
        let rb = mysql::default().await;
        let page = PageRequest::new(page_no, 5);
        let w = rb.new_wrapper()
            .eq("is_publish","1")
            .order_by(false, &["publish_time"]);
        let result: rbatis::Result<Page<Self>> = rb.fetch_page_by_wrapper(w, &page).await;
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("根据博客信息异常, 异常信息为: {}", e);
                None
            }
        }
    }


    pub async fn query_by_id(id: usize) -> Option<Self> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Option<Self>> =
            rb.fetch_by_column("id", &id)
                .await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("根据id查询博客信息异常, 异常信息为: {}", e);
                None
            }
        }
    }

    pub async fn add_read_count(id: usize) {
        let rb = mysql::default().await;
        let result:rbatis::Result<Option<u8>> = rb.fetch("update blog_info set read_count = read_count+1 where id = ?",
                              vec![Bson::Int64(id as i64)]).await;
       if let Err(e) = result {
           log::error!("查看次数增加时异常, 异常信息为: {}", e)
       }
    }
}
