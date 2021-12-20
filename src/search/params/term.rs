use crate::util::*;
use chrono::{DateTime, Utc};
use std::{cmp::Ordering, convert::TryFrom};

/// Leaf term value
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Term(Option<Inner>);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum Inner {
    /// Boolean value
    Bool(bool),

    /// String value
    String(String),

    /// Signed integer value
    SignedInteger(i64),

    /// Unsigned integer value
    UnsignedInteger(u64),

    /// Floating 32-bit point value
    Float32(f32),

    /// Floating 64-bit point value
    Float64(f64),

    /// DateTime
    DateTime(DateTime<Utc>),
}

fn try_eq<L, R>(left: &L, right: &R) -> bool
where
    L: TryFrom<R> + PartialEq + Copy,
    R: PartialEq + Copy,
{
    L::try_from(*right).map_or(false, |r| left.eq(&r))
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Inner::Bool(s), Inner::Bool(o)) => s.eq(o),
            (Inner::String(s), Inner::String(o)) => s.eq(o),
            (Inner::SignedInteger(s), Inner::SignedInteger(o)) => s.eq(o),
            (Inner::SignedInteger(s), Inner::UnsignedInteger(o)) => try_eq(s, o),
            (Inner::UnsignedInteger(s), Inner::SignedInteger(o)) => try_eq(s, o),
            (Inner::UnsignedInteger(s), Inner::UnsignedInteger(o)) => s.eq(o),
            (Inner::Float32(s), Inner::Float32(o)) => s.eq(o),
            (Inner::Float32(s), Inner::Float64(o)) => try_eq(o, s),
            (Inner::Float64(s), Inner::Float32(o)) => try_eq(s, o),
            (Inner::Float64(s), Inner::Float64(o)) => s.eq(o),
            (Inner::DateTime(s), Inner::DateTime(o)) => s.eq(o),
            _ => false,
        }
    }
}

impl Eq for Inner {}

impl PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, other) {
            (Self::Bool(s), Self::Bool(o)) => s.partial_cmp(o),
            (Self::String(s), Self::String(o)) => s.partial_cmp(o),
            (Self::SignedInteger(s), Self::SignedInteger(o)) => s.partial_cmp(o),
            (Self::UnsignedInteger(s), Self::UnsignedInteger(o)) => s.partial_cmp(o),
            (Self::Float32(s), Self::Float32(o)) => s.partial_cmp(o),
            (Self::Float64(s), Self::Float64(o)) => s.partial_cmp(o),
            (Self::DateTime(s), Self::DateTime(o)) => s.partial_cmp(o),
            _ => Some(Ordering::Less),
        }
    }
}

impl Ord for Inner {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (Self::Bool(s), Self::Bool(o)) => s.cmp(o),
            (Self::String(s), Self::String(o)) => s.cmp(o),
            (Self::SignedInteger(s), Self::SignedInteger(o)) => s.cmp(o),
            (Self::UnsignedInteger(s), Self::UnsignedInteger(o)) => s.cmp(o),
            (Self::Float32(s), Self::Float32(o)) => s.partial_cmp(o).unwrap_or(Ordering::Less),
            (Self::Float64(s), Self::Float64(o)) => s.partial_cmp(o).unwrap_or(Ordering::Less),
            (Self::DateTime(s), Self::DateTime(o)) => s.cmp(o),
            _ => Ordering::Less,
        }
    }
}

impl From<bool> for Term {
    fn from(value: bool) -> Self {
        Self(Some(Inner::Bool(value)))
    }
}

impl From<String> for Term {
    fn from(value: String) -> Self {
        Self(Some(Inner::String(value)))
    }
}

impl From<&str> for Term {
    fn from(value: &str) -> Self {
        Self(Some(Inner::String(value.into())))
    }
}

impl From<i8> for Term {
    fn from(value: i8) -> Self {
        Self(Some(Inner::SignedInteger(value as i64)))
    }
}

impl From<i16> for Term {
    fn from(value: i16) -> Self {
        Self(Some(Inner::SignedInteger(value as i64)))
    }
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Self(Some(Inner::SignedInteger(value as i64)))
    }
}

impl From<i64> for Term {
    fn from(value: i64) -> Self {
        Self(Some(Inner::SignedInteger(value)))
    }
}

impl From<u8> for Term {
    fn from(value: u8) -> Self {
        Self(Some(Inner::UnsignedInteger(value as u64)))
    }
}

impl From<u16> for Term {
    fn from(value: u16) -> Self {
        Self(Some(Inner::UnsignedInteger(value as u64)))
    }
}

impl From<u32> for Term {
    fn from(value: u32) -> Self {
        Self(Some(Inner::UnsignedInteger(value as u64)))
    }
}

impl From<u64> for Term {
    fn from(value: u64) -> Self {
        Self(Some(Inner::UnsignedInteger(value)))
    }
}

impl From<f32> for Term {
    fn from(value: f32) -> Self {
        Self(Some(Inner::Float32(value)))
    }
}

impl From<f64> for Term {
    fn from(value: f64) -> Self {
        Self(Some(Inner::Float64(value)))
    }
}

impl From<DateTime<Utc>> for Term {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Some(Inner::DateTime(value)))
    }
}
impl From<Option<bool>> for Term {
    fn from(value: Option<bool>) -> Self {
        Self(value.map(Inner::Bool))
    }
}

impl From<Option<String>> for Term {
    fn from(value: Option<String>) -> Self {
        Self(value.map(Inner::String))
    }
}

impl From<Option<&str>> for Term {
    fn from(value: Option<&str>) -> Self {
        Self(value.map(Into::into).map(Inner::String))
    }
}

impl From<Option<i8>> for Term {
    fn from(value: Option<i8>) -> Self {
        Self(value.map(i64::from).map(Inner::SignedInteger))
    }
}

impl From<Option<i16>> for Term {
    fn from(value: Option<i16>) -> Self {
        Self(value.map(i64::from).map(Inner::SignedInteger))
    }
}

impl From<Option<i32>> for Term {
    fn from(value: Option<i32>) -> Self {
        Self(value.map(i64::from).map(Inner::SignedInteger))
    }
}

impl From<Option<i64>> for Term {
    fn from(value: Option<i64>) -> Self {
        Self(value.map(Inner::SignedInteger))
    }
}

impl From<Option<u8>> for Term {
    fn from(value: Option<u8>) -> Self {
        Self(value.map(u64::from).map(Inner::UnsignedInteger))
    }
}

impl From<Option<u16>> for Term {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(u64::from).map(Inner::UnsignedInteger))
    }
}

impl From<Option<u32>> for Term {
    fn from(value: Option<u32>) -> Self {
        Self(value.map(u64::from).map(Inner::UnsignedInteger))
    }
}

impl From<Option<u64>> for Term {
    fn from(value: Option<u64>) -> Self {
        Self(value.map(Inner::UnsignedInteger))
    }
}

impl From<Option<f32>> for Term {
    fn from(value: Option<f32>) -> Self {
        Self(value.map(Inner::Float32))
    }
}

impl From<Option<f64>> for Term {
    fn from(value: Option<f64>) -> Self {
        Self(value.map(Inner::Float64))
    }
}

impl From<Option<DateTime<Utc>>> for Term {
    fn from(value: Option<DateTime<Utc>>) -> Self {
        Self(value.map(Inner::DateTime))
    }
}

impl ShouldSkip for Term {
    fn should_skip(&self) -> bool {
        match &self.0 {
            None => true,
            Some(Inner::String(value)) => value.should_skip(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn partial_equality() {
        let values = vec![
            (Inner::Bool(true), Inner::Bool(true)),
            (Inner::String("a".into()), Inner::String("a".into())),
            (Inner::SignedInteger(16), Inner::SignedInteger(16)),
            (Inner::SignedInteger(16), Inner::UnsignedInteger(16)),
            (Inner::UnsignedInteger(16), Inner::SignedInteger(16)),
            (Inner::UnsignedInteger(16), Inner::UnsignedInteger(16)),
            (Inner::Float32(1.), Inner::Float32(1.)),
            (Inner::Float32(1.), Inner::Float64(1.)),
            (Inner::Float64(1.), Inner::Float32(1.)),
            (Inner::Float64(1.), Inner::Float64(1.)),
            (
                Inner::DateTime(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
                Inner::DateTime(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
            ),
        ];

        for (left, right) in values {
            assert_eq!(left, right);
        }
    }
}