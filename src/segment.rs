/// 3.2.22 Object: Segment
///
/// Segment objects are essentially key-value pairs that convey specific units of data. The parent
/// Data object is a collection of such values from a given data provider. The specific segment
/// names and value options must be published by the exchange a priori to its bidders.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Segment {
    /// string
    /// ID of the data segment specific to the data provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Name of the data segment specific to the data provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// String representation of the data segment value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Segment::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Segment>(json)?);

        Ok(())
    }
}
