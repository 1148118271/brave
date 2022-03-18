pub use date_utils::{get_date, get_date_time};
pub use paging::paging;
pub use results::{html, html_err};
pub use head::head;
pub use results::Results;
pub use results::Paging;
pub use params::PageParams;

pub use crate::paging;

pub use crate::head;

mod results;
mod paging;
pub mod date_utils;
pub mod multipart_file;
mod head;
pub mod auth;
pub mod session;
mod params;

#[macro_export]
macro_rules! paging {
    ($v1:expr,$v2:expr,$v3:expr) => {
        paging($v1,$v2,$v3)
    };
}

#[macro_export]
macro_rules! head {
    () => {
        head()
    };
}