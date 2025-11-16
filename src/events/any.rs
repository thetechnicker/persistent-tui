use std::any::Any;
use std::marker;
use std::sync::Arc;

type Int = u64;
type Float = f64;

#[derive(Clone, Debug)]
pub enum AnyType {
    AnyString(String),
    AnyInt(Int),
    AnyFloat(Float),
    AnyChar(char),
    AnyCustom(Arc<dyn Any + marker::Send + marker::Sync>),
}

trait StringHelper: Into<String> {}

impl StringHelper for String {}
impl StringHelper for &str {}

impl<T> From<T> for AnyType
where
    T: StringHelper,
{
    fn from(other: T) -> AnyType {
        AnyType::AnyString(other.into())
    }
}

impl From<usize> for AnyType {
    fn from(other: usize) -> AnyType {
        AnyType::AnyInt(other as Int)
    }
}
impl From<u8> for AnyType {
    fn from(other: u8) -> AnyType {
        AnyType::AnyInt(other as Int)
    }
}
impl From<u16> for AnyType {
    fn from(other: u16) -> AnyType {
        AnyType::AnyInt(other as Int)
    }
}
impl From<u32> for AnyType {
    fn from(other: u32) -> AnyType {
        AnyType::AnyInt(other as Int)
    }
}
impl From<u64> for AnyType {
    fn from(other: u64) -> AnyType {
        AnyType::AnyInt(other as Int)
    }
}

impl From<f32> for AnyType {
    fn from(other: f32) -> AnyType {
        AnyType::AnyFloat(other as Float)
    }
}
impl From<f64> for AnyType {
    fn from(other: f64) -> AnyType {
        AnyType::AnyFloat(other as Float)
    }
}

impl From<char> for AnyType {
    fn from(other: char) -> AnyType {
        AnyType::AnyChar(other)
    }
}
