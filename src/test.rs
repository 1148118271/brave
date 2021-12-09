#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::cell;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::FromStr;
    use bstr::ByteSlice;
    use rbatis::rbatis::Rbatis;
    use rbatis::py_sql;
    use rbatis::sql;
    use serde_json::Value;
    use serde::{Deserialize, Deserializer, Serialize};

    use rbatis::{ crud_table };
    use rbatis::crud::CRUD;
    use rbatis::executor::Executor;
    use crate::entity::BlogUser;

    #[derive(Debug)]
    struct Test {
        name: String,
        age: String
    }


    #[test]
    fn test() {

        let t = String::from("aa");


        let s = &t;
        println!("{:?}", t);
        let a = s as *const String as *mut String;
        unsafe { *a = "ss".to_string() }

        println!("{:?}", t)
    }


    #[actix_rt::test]
    async fn test1() {

    }





}
