/// Aggregation name
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AggregationName(String);

impl<T> From<T> for AggregationName
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
impl AsRef<str> for AggregationName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
