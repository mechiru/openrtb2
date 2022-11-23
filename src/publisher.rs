/// 3.2.15 Object: Publisher
///
/// This object describes the publisher of the media in which the ad will be displayed. The
/// publisher is typically the seller in an OpenRTB transaction.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Publisher<'a> {
    /// string
    /// Exchange-specific publisher ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Publisher name (may be aliased at the publisher’s request).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories that describe the publisher. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// string
    /// Highest level domain of the publisher (e.g., “publisher.com”).
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
        let o1 = Publisher::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Publisher>(json)?);

        Ok(())
    }
}
