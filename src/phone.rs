//! 电话号码相关的工具函数
//!
//! # Example
//!
//! ```
//! use yansongda_utils::phone;
//!
//! // 是否是手机号码
//! assert!(phone::is_mobile("13800138000"));
//! // 是否是固定电话
//! assert!(phone::is_telephone("01012345678"));
//! // 是否是服务号码
//! assert!(phone::is_service("12345678"));
//! // 是否是长途
//! assert!(phone::is_idd("0012345678"));
//! ```

use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use crate::regex;

/// 电话类型
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PhoneType {
    /// 固定电话
    Tel,
    /// 手机
    Mobile,
    /// 长途
    Idd,
    /// 服务号码
    Service,
    /// 其它
    Others,
}

impl ToString for PhoneType {
    fn to_string(&self) -> String {
        match self {
            PhoneType::Tel => String::from("TEL"),
            PhoneType::Mobile => String::from("MOBILE"),
            PhoneType::Idd => String::from("IDD"),
            PhoneType::Service => String::from("SERVICE"),
            _ => String::from("OTHERS"),
        }
    }
}

impl From<String> for PhoneType {
    fn from(v: String) -> Self {
        match v.as_str() {
            "TEL" | "Tel" => PhoneType::Tel,
            "MOBILE" | "Mobile" => PhoneType::Mobile,
            "IDD" | "Idd" => PhoneType::Idd,
            "SERVICE" | "Service" => PhoneType::Service,
            _s => PhoneType::Others,
        }
    }
}

impl From<PhoneType> for String {
    fn from(v: PhoneType) -> Self {
        v.to_string()
    }
}

impl Serialize for PhoneType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let str = self.to_string();

        serializer.serialize_str(&str[..])
    }
}

struct PhoneTypeVisitor;

impl<'de> Visitor<'de> for PhoneTypeVisitor {
    type Value = PhoneType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("反序列化失败，值应该为 string/str.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(v.to_owned().into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(v.into())
    }
}

impl<'de> Deserialize<'de> for PhoneType {
    fn deserialize<D>(deserializer: D) -> Result<PhoneType, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(PhoneTypeVisitor)
    }
}

/// 电话运营商
#[derive(Clone, Debug)]
pub enum MobileVendor {
    /// 中国移动
    Mobile,
    /// 中国联通
    Unicom,
    /// 中国电信
    Telecom,
    /// 中国广电
    Cbn,
    /// 其它
    Others,
}

impl ToString for MobileVendor {
    fn to_string(&self) -> String {
        match self {
            MobileVendor::Unicom => String::from("10010 联通"),
            MobileVendor::Telecom => String::from("10000 电信"),
            MobileVendor::Mobile => String::from("10086 移动"),
            MobileVendor::Cbn => String::from("10099 广电"),
            _ => String::from("unknown 未知"),
        }
    }
}

impl From<String> for MobileVendor {
    fn from(v: String) -> Self {
        match v.as_str() {
            "10010 联通" | "10010" => MobileVendor::Unicom,
            "10000 电信" | "10000" => MobileVendor::Telecom,
            "10086 移动" | "10086" => MobileVendor::Mobile,
            "10099 广电" | "10099" => MobileVendor::Cbn,
            _s => MobileVendor::Others,
        }
    }
}

impl From<MobileVendor> for String {
    fn from(v: MobileVendor) -> Self {
        v.to_string()
    }
}

impl Serialize for MobileVendor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let str = self.to_string();

        serializer.serialize_str(&str[..])
    }
}

struct MobileVendorVisitor;

impl<'de> Visitor<'de> for MobileVendorVisitor {
    type Value = MobileVendor;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("反序列化失败，值应该为 string/str.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(v.to_owned().into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(v.into())
    }
}

impl<'de> Deserialize<'de> for MobileVendor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(MobileVendorVisitor)
    }
}

/// 给定的号码是否是正常的手机号
pub fn is_mobile(number: &str) -> bool {
    regex!(r"^(\+)?(86)?(0)?1[3-9]\d{9}$").is_match(number)
}

/// 给定的号码是否是正常的座机号(含分机)
pub fn is_telephone(number: &str) -> bool {
    regex!(r"^(\+)?(86)?0\d{9,11}([-,]\d{4,7})?$").is_match(number)
}

/// 给定的号码是否是正常的服务号码
pub fn is_service(number: &str) -> bool {
    regex!(r"^1\d{7}$|^[1,9]\d{4}$").is_match(number)
}

/// 给定的号码是否是正常的国际长途号码
pub fn is_idd(number: &str) -> bool {
    regex!(r"^00\d{8,}$").is_match(number)
}

/// 给定的号码是否是正常的电话号码
pub fn is_phone(number: &str) -> bool {
    is_mobile(number) || is_telephone(number) ||
        is_service(number) || is_idd(number)
}

/// 将号码转换为中国标准格式，即不带 +、+86、86、0 等形式
/// 如果是国际号码，维持不变
pub fn to_standard_format(number: &str) -> &str {
    let mut offset: usize = 0;

    if number[offset..].starts_with(char::from_str("+").unwrap()) {
        offset += 1;
    }

    if number[offset..].starts_with("86") {
        offset += 2;
    }

    if is_mobile(&number[offset..]) && number[offset..].starts_with(char::from_str("0").unwrap()) {
        offset += 1;
    }

    &number[offset..]
}

/// 获取号码的号段，以便区分运营商，获取号段详细归属地信息
pub fn get_segment(number: &str) -> (PhoneType, &str) {
    if is_mobile(number) {
        return (PhoneType::Mobile, &number[..7]);
    }

    if is_idd(number) {
        return (PhoneType::Idd, &number[2..6]);
    }

    if is_service(number) {
        return (PhoneType::Service, number);
    }

    if number.starts_with("010") || number[..2].starts_with("02") {
        return (PhoneType::Tel, &number[..3]);
    }

    (PhoneType::Tel, &number[..4])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mobile() {
        assert!(is_mobile("13800138000"));
        assert!(is_mobile("+8613800138000"));
    }

    #[test]
    fn test_is_telephone() {
        assert!(is_telephone("01012345678"));
        assert!(is_telephone("01012345678-1234"));
        assert!(is_telephone("01012345678,1234"));
        assert!(is_telephone("075512345678"));
        assert!(is_telephone("075512345678-1234"));
        assert!(is_telephone("075512345678,1234"));
    }

    #[test]
    fn test_is_service() {
        assert!(is_service("10000000"));
        assert!(is_service("10086"));
        assert!(is_service("95588"));
    }

    #[test]
    fn test_is_idd() {
        assert!(is_idd("0012345678"));
        assert!(is_idd("008512345678"));
    }

    #[test]
    fn test_is_phone() {
        assert!(is_phone("13800138000"));
        assert!(is_phone("+8613800138000"));
        assert!(is_phone("01012345678"));
        assert!(is_phone("01012345678-1234"));
        assert!(is_phone("01012345678,1234"));
        assert!(is_phone("075512345678"));
        assert!(is_phone("075512345678-1234"));
        assert!(is_phone("075512345678,1234"));
        assert!(is_phone("10000000"));
        assert!(is_phone("10086"));
        assert!(is_phone("95588"));
        assert!(is_phone("0012345678"));
        assert!(is_phone("008512345678"));
    }
}
