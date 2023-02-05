//! 关于自己的常用的一些工具函数
//!
//! # Features
//!
//! 默认情况下，所有功能都是禁用的，需要单独制定相关 feature 来启用。
//!
//! ## `macros`
//! 一些有用的宏
//!
//! ## `phone`
//! 电话号码相关的工具函数

#![forbid(unsafe_code)]

#[cfg(feature = "macros")]
pub mod macros;
#[cfg(feature = "phone")]
pub mod phone;

#[cfg(feature = "once_cell")]
pub use once_cell;
#[cfg(feature = "regex")]
pub use regex as regex_crate;
#[cfg(feature = "serde")]
pub use serde;
