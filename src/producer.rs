/// 3.2.17 Object: Producer
///
/// This object defines the producer of the content in which the ad will be shown. This is
/// particularly useful when the content is syndicated and may be distributed through different
/// publishers and thus when the producer and publisher are not necessarily the same entity.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Producer<'a> {
    /// string
    /// Content producer or originator ID. Useful if content is syndicated and may be posted on a
    /// site using embed tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Content producer or originator name (e.g., “Warner Bros”).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories that describe the content producer. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// string
    /// Highest level domain of the content producer (e.g., “producer.com”).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<std::borrow::Cow<'a, str>>,

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
        let o1 = Producer::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Producer>(json)?);

        Ok(())
    }
}
