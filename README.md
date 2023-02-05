# rust 中一些常用工具

[![Linter Status](https://github.com/yansongda/rust-utils/workflows/linter/badge.svg)](https://github.com/yansongda/rust-utils/actions)
[![Tests Status](https://github.com/yansongda/rust-utils/workflows/tests/badge.svg)](https://github.com/yansongda/rust-utils/actions)
[![doc.rs](https://docs.rs/yansongda-utils/badge.svg)](https://docs.rs/yansongda-utils/)
[![doc.rs](https://img.shields.io/crates/v/yansongda-utils)](https://docs.rs/yansongda-utils/)

## 安装

```toml
[dependencies]
yansongda-utils = { version = "~1.0.0", features = ["phone"] }
```

## 文档

[点击传送](https://docs.rs/yansongda-utils/)

## 使用

### Phone 模块

```rust
use yansongda_utils::phone;

// 是否是手机号码
assert!(phone::is_mobile("13800138000"));
// 是否是固定电话
assert!(phone::is_telephone("01012345678"));
// 是否是服务号码
assert!(phone::is_service("12345678"));
// 是否是长途
assert!(phone::is_idd("0012345678"));
```