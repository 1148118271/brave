pub use date_utils::get_date_time;
pub use results::{html, html_err};
pub use results::Results;

pub use crate::head;

mod results;
pub mod date_utils;

#[macro_export]
macro_rules! head {
    () => {
        head()
    };
}

#[macro_export]
macro_rules! os_path {
    ($($x:expr),*) => {{
        let mut buf = std::path::PathBuf::new();
        $(
            buf = buf.join(std::path::Path::new($x));
        )*
        buf.into_os_string().into_string().unwrap()
    }};
}