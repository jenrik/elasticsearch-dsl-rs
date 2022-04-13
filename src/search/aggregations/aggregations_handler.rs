use serde_json::Value;
/// Aggregations response handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregationsHandler<'a> {
    aggregations: Option<&'a Value>,
}

impl<'a> AggregationsHandler<'a> {
    /// Creates a new instance of [AggregationsHandler]
    pub fn new(aggregations: Option<&'a Value>) -> Self {
        Self { aggregations }
    }

    /// Returns terms aggregations container
    pub fn terms<N>(&self, aggregation_name: N) -> Option<&Value>
    where
        N: AsRef<str>,
    {
        let agg_name = aggregation_name.as_ref();

        let a = &self.aggregations?[agg_name];

        Some(a)
    }
}
