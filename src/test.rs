#[cfg(test)]
mod tests {
    use std::detect::__is_feature_detected::mmx;
    use std::ops::Deref;
    use bstr::ByteSlice;
    use rbatis::rbatis::Rbatis;
    use rbatis::py_sql;
    use rbatis::sql;
    use serde_json::Value;
    use serde::{Deserialize, Serialize};

    use rbatis::{ crud_table };
    use rbatis::crud::CRUD;
    use rbatis::executor::Executor;
    use crate::entity::BlogUser;

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        name: String,
        age: u32,
    }

    struct Params<T>(T);

    impl<T> Deref for Params<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }


    #[crud_table]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Res {
        group_name: String,
        group_id: usize,
        count: u32,
    }


    #[test]
    fn test() {
        //groub_by(&rb).await.unwrap();
    }


    // #[sql("select
    //    (select group_value from blog_group where group_id = group_id) group_name,
    //    group_id group_id,
    //    count(id) count from blog_info group by group_id")]
    // async fn groub_by(rbatis: &Rbatis) -> Vec<Res> {}

    #[actix_rt::test]
    async fn test1() {
        let rb = Rbatis::new();
        rb.link("mysql://root:root@localhost:3306/blogs").await.unwrap();
       // let wrapper = rb.new_wrapper().push("select (select group_value from blog_group where group_id = group_id) group_name, group_id group_id, count(id) count from blog_info group by group_id;", Vec::new());
       // let wrapper = rb.new_wrapper()
       //
       //  let x:Option<Res> = rb.fetch_by_wrapper(wrapper).await.unwrap();
        //let x: Vec<BlogUser> = rb.fetch_list().await.unwrap();


        let x: Vec<Res> = rb.fetch(r#"select
         (select group_value from blog_group where group_id = group_id)
         group_name,
         group_id group_id,
          count(id) count
          from
          blog_info
          group by
          group_id;"#, Vec::new()).await.unwrap();
        println!("x => {:?}", x);
    }





}
