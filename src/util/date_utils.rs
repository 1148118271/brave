use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bson::{Bson, DateTime};
use chrono::{Local, NaiveDateTime};
use serde::{Deserializer, Serializer};
use serde::de::Error;

// 日期 + 时间
pub const FMT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";


pub fn get_date_time() -> String {
    let date_time = Local::now();
    date_time.format(FMT_Y_M_D_H_M_S).to_string()
}


/// Rbatis DateTime
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTimeUtil {
    pub inner: chrono::NaiveDateTime,
}

impl From<bson::DateTime> for DateTimeUtil {
    fn from(arg: DateTime) -> Self {
        Self {
            inner: arg.to_chrono().with_timezone(&chrono::Local).naive_local(),
        }
    }
}

impl From<chrono::DateTime<Local>> for DateTimeUtil {
    fn from(arg: chrono::DateTime<Local>) -> Self {
        DateTimeUtil {
            inner: arg.naive_local()
        }
    }
}

impl From<&chrono::DateTime<Local>> for DateTimeUtil {
    fn from(arg: &chrono::DateTime<Local>) -> Self {
        DateTimeUtil {
            inner: arg.clone().naive_local()
        }
    }
}

impl serde::Serialize for DateTimeUtil {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        return serializer.serialize_str(&format!("{}", self.inner));
    }
}

impl<'de> serde::Deserialize<'de> for DateTimeUtil {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::DateTime(date) => {
                return Ok(Self {
                    inner: date.to_chrono().with_timezone(&chrono::Local).naive_local(),
                });
            }
            Bson::String(s) => {
                return if s.starts_with("DateTimeNative(") && s.ends_with(")") {
                    let inner_data = &s["DateTimeNative(".len()..(s.len() - 1)];
                    Ok(Self {
                        inner: chrono::NaiveDateTime::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    })
                } else {
                    Ok(Self {
                        inner: chrono::NaiveDateTime::from_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    })
                }
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}

impl std::fmt::Display for DateTimeUtil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("调用 fmt::Display");
        let result = NaiveDateTime::parse_from_str(&self.inner.to_string(), FMT_Y_M_D_H_M_S).unwrap();
        write!(f, "{}", result)

    }
}

impl std::fmt::Debug for DateTimeUtil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = NaiveDateTime::parse_from_str(&self.inner.to_string(), FMT_Y_M_D_H_M_S).unwrap();
        write!(f, "{}", result)
    }
}

impl Deref for DateTimeUtil {
    type Target = chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DateTimeUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

