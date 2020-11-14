/// 3.2.5 Object: Metric
///
/// This object is associated with an impression as an array of metrics. These metrics can offer
/// insight into the impression to assist with decisioning such as average recent viewability,
/// click-through rate, etc. Each metric is identified by its type, reports the value of the metric,
/// and optionally identifies the source or vendor measuring the value.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Metric<'a> {
    /// string; required
    /// Type of metric being presented using exchange curated string names which should be
    /// published to bidders a priori.
    #[serde(borrow)]
    pub r#type: std::borrow::Cow<'a, str>,

    /// float; required
    /// Number representing the value of the metric. Probabilities must be in the range 0.0 – 1.0.
    pub value: f32,

    /// string; recommended
    /// Source of the value using exchange curated string names which should be published to
    /// bidders a priori. If the exchange itself is the source versus a third party, “EXCHANGE” is
    /// recommended.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<json_ext::Object<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Metric>("{}").is_err());

        let json = r#"{"type":"","value":0.0}"#;
        let o1 = Metric::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Metric>(json)?);

        Ok(())
    }
}
