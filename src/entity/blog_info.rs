use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::{crud_table, py_sql, DateTimeNative, Page, PageRequest};
use rbatis::executor::RbatisExecutor;
use serde:: {
    Serialize,
    Deserialize,
};
use crate::entity::{ BlogGroup, BlogUser};
use crate::util::{ mysql, date_utils };

#[crud_table(table_columns:"id, title, generalize, user_account, publish_time, group_id, read_count, is_publish, create_time, update_time")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogInfo {
    // 主键
    id: usize,
    // 标题
    title: String,
    // 简介/概括
    generalize: String,
    // 用户账号
    user_account: String,
    // 用户名称
    user_name: Option<String>,
    // 发布时间
    publish_time: date_utils::DateTimeUtil,
    // 分组关联id
    group_id: usize,
    // 分组名称
    group_name: Option<String>,
    // 评论次数
    comment_count: Option<u32>,
    // 阅读次数
    read_count: u32,
    // 是否发布 0 未发布, 1 已发布
    is_publish: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime
}

impl BlogInfo {
    pub async fn query_paging(page_num: isize, limit_num: isize, params: Option<isize>) -> rbatis::Result<(Vec<BlogInfo>, u64)> {
        let rb = mysql::this();
        let wrapper = match params {
            None => {rb.new_wrapper().order_by(false, &["publish_time"])}
            Some(v) => {rb.new_wrapper().eq("group_id", v).order_by(false, &["publish_time"])}
        };
        let pr = PageRequest::new(page_num as u64, limit_num as u64);
        let page: Page<Self> = rb.fetch_page_by_wrapper(wrapper, &pr).await?;

        let pages = page.pages;

        let mut result = page.records;
        if result.len() == 0 {
           return Ok((result, 1u64))
        }

        for mut x in &mut result {
            let account = BlogUser::query_by_account(&x.user_account).await?;
            let user_name = match account {
                None => "子木".to_string(),
                Some(u) => u.name.unwrap_or("子木".to_string())
            };
            x.user_name = Some(user_name);

            let blog_group = BlogGroup::query_by_id(x.group_id).await?;
            let group_name = match blog_group {
                None => "默认分组".to_string(),
                Some(v) => v.group_value
            };

            x.group_name = Some(group_name);

            x.comment_count = Some(0);

        }

        Ok((result, pages))
    }


    //
    // #[py_sql(
    //     "select
    //         (select group_value from blog_group where group_id = group_id) \\\"group_name\\\",
    //         group_id "group_id",
    //         count(id) "count"
    //     from
    //         blog_info
    //     group by
    //         group_id"
    // )]
    // pub async fn query_group_by_group_id(rb: &mut RbatisExecutor<'_,'_>) -> Option<Vec<serde_json::Value>> {}
}

