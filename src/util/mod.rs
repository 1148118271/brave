pub use date_utils::{get_date, get_date_time};
pub use results::{html, html_err};
pub use results::Results;

pub use crate::head;

mod results;
pub mod date_utils;
pub mod multipart_file;

#[macro_export]
macro_rules! head {
    () => {
        head()
    };
}