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
    use crate::entity::{BlogGroup, BlogInfo};

    #[crud_table]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GroupRes {
        // 分组名称
        pub group_name: String,
        // 分组编码
        pub group_id: usize,
        // 分组下博客数量
        pub count: u32,
        pub blog_info: Vec<BlogInfo>
    }

    impl GroupRes {
        pub async fn query_blog_group() -> rbatis::Result<Vec<Self>> {
            let mut r = vec![];
            let vec = BlogGroup::query_all().await?;
            for x in vec {
                let publish = BlogInfo::query_by_group_id_is_publish(x.id).await?;
                r.push(GroupRes {
                    group_name: x.group_value.clone(),
                    group_id: x.id,
                    count: publish.len() as u32,
                    blog_info: publish
                })
            }
            Ok(r)
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