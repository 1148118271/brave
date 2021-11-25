#[cfg(test)]
mod tests {
    use std::detect::__is_feature_detected::mmx;
    use std::ops::Deref;
    use bstr::ByteSlice;
    use serde_json::Value;
    use serde::{Deserialize, Serialize};

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


    #[test]
    fn test() {
        let params = Params(User {
            name: "".to_string(),
            age: 0
        });

        println!("{}", params.age);
    }


}
