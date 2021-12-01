mod blog_user;
mod blog_info;
mod blog_group;
mod blog_details;
mod blog_comments;


pub use blog_user::BlogUser;
pub use blog_info::BlogInfo;
pub use blog_group::BlogGroup;
pub use blog_details::BlogDetails;
pub use blog_comments::BlogComments;


pub mod vo {
    use serde::{ Deserialize, Serialize };
    use rbatis::{ crud_table };
    use rbatis::executor::Executor;
    use crate::util::mysql;

    #[crud_table]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GroupRes {
        // 分组名称
        pub group_name: String,
        // 分组编码
        pub group_id: usize,
        // 分组下博客数量
        pub count: u32,
    }

    impl GroupRes {
        pub async fn query_blog_group() -> rbatis::Result<Vec<Self>> {
            let rb = mysql::this();
            let x = rb.fetch(
                r#"
                    select
                        bg.group_value group_name,
                        bi.group_id group_id,
                        count(bi.group_id) count
                    from
                        blog_info bi
                    left join
                        blog_group bg
                    on
                        bi.group_id = bg.id
                    where
                        bi.is_publish = 1
                    group by
                        bi.group_id;
                    "#, Vec::new()).await?;
            Ok(x)
        }
    }


    #[derive(Serialize, Deserialize)]
    pub struct BlogPage {
        pub page_num: Option<isize>,
        pub limit_num: Option<isize>,
        pub page_count: Option<isize>,

        pub group_id: Option<isize>,
    }
}