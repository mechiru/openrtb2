/// 3.2.9 Object: Native
///
/// This object represents a native type impression. Native ad units are intended to blend
/// seamlessly into the surrounding content (e.g., a sponsored Twitter or Facebook post). As such,
/// the response must be well-structured to afford the publisher fine-grained control over
/// rendering.
///
/// The Native Subcommittee has developed a companion specification to OpenRTB called the Dynamic
/// Native Ads API. It defines the request parameters and response markup structure of native ad
/// units. This object provides the means of transporting request parameters as an opaque string so
/// that the specific parameters can evolve separately under the auspices of the Dynamic Native Ads
/// API. Similarly, the ad markup served will be structured according to that specification.
///
/// The presence of a Native as a subordinate of the Imp object indicates that this impression is
/// offered as a native type impression. At the publisher’s discretion, that same impression may
/// also be offered as banner, video, and/or audio by also including as Imp subordinates objects of
/// those types. However, any given bid for the impression must conform to one of the offered types.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Native {
    /// string; required
    /// Request payload complying with the Native Ad Specification.
    #[serde(borrow)]
    pub request: std::borrow::Cow<'a, str>,

    /// string; recommended
    /// Version of the Dynamic Native Ads API to which request complies; highly recommended for
    /// efficient parsing.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<std::borrow::Cow<'a, str>>,

    /// integer array
    /// List of supported API frameworks for this impression. Refer to List 5.6. If an API is not
    /// explicitly listed, it is assumed not to be supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<crate::ApiFramework>>,

    /// integer array
    /// Blocked creative attributes. Refer to List 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<crate::CreativeAttribute>>,

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
        assert!(serde_json::from_str::<Native>("{}").is_err());

        let json = r#"{"request":""}"#;
        let o1 = Native::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Native>(json)?);

        Ok(())
    }
}
