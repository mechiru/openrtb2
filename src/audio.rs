/// 3.2.8 Object: Audio
///
/// This object represents an audio type impression. Many of the fields are non-essential for
/// minimally viable transactions, but are included to offer fine control when needed. Audio in
/// OpenRTB generally assumes compliance with the DAAST standard. As such, the notion of companion
/// ads is supported by optionally including an array of Banner objects (refer to the Banner object
/// in Section 3.2.6) that define these companion ads.
///
/// The presence of a Audio as a subordinate of the Imp object indicates that this impression is
/// offered as an audio type impression. At the publisher’s discretion, that same impression may
/// also be offered as banner, video, and/or native by also including as Imp subordinates objects of
/// those types. However, any given bid for the impression must conform to one of the offered types.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Audio<'a> {
    /// string array; required
    /// Content MIME types supported (e.g., “audio/mp4”).
    #[serde(borrow)]
    pub mimes: Vec<std::borrow::Cow<'a, str>>,

    /// integer; recommended
    /// Minimum audio ad duration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// integer; recommended
    /// Maximum audio ad duration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// integer array; recommended
    /// Array of supported audio protocols. Refer to List 5.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<crate::Protocol>>,

    /// integer; recommended
    /// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll ad placements.
    /// Refer to List 5.12.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<crate::StartDelay>,

    /// integer
    /// If multiple ad impressions are offered in the same bid request, the sequence number will
    /// allow for the coordinated delivery of multiple creatives.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// integer array
    /// Blocked creative attributes. Refer to List 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battr: Option<crate::CreativeAttribute>,

    /// integer
    /// Maximum extended ad duration if extension is allowed. If blank or 0, extension is not
    /// allowed. If -1, extension is allowed, and there is no time limit imposed. If greater than
    /// 0, then the value represents the number of seconds of extended play supported beyond the
    /// maxduration value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxextended: Option<crate::MaxExtendedAdDuration>,

    /// integer
    /// Minimum bit rate in Kbps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// integer
    /// Maximum bit rate in Kbps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// integer array
    /// Supported delivery methods (e.g., streaming, progressive). If none specified, assume all
    /// are supported. Refer to List 5.15.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<crate::ContentDeliveryMethod>>,

    /// object array
    /// Array of Banner objects (Section 3.2.6) if companion ads are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<crate::Banner<'a>>>,

    /// integer array
    /// List of supported API frameworks for this impression. Refer to List 5.6. If an API is not
    /// explicitly listed, it is assumed not to be supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<crate::ApiFramework>>,

    /// integer array
    /// Supported DAAST companion ad types. Refer to List 5.14. Recommended if companion Banner
    /// objects are included via the companionad array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<crate::CompanionType>>,

    /// integer
    /// The maximum number of ads that can be played in an ad pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxseq: Option<i32>,

    /// integer
    /// Type of audio feed. Refer to List 5.16.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed: Option<crate::FeedType>,

    /// integer
    /// Indicates if the ad is stitched with audio content or delivered independently, where 0 =
    /// no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub stitched: Option<bool>,

    /// integer
    /// Volume normalization mode. Refer to List 5.17.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nvol: Option<crate::VolumeNormalizationMode>,

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
        assert!(serde_json::from_str::<Audio>("{}").is_err());

        let json = r#"{"mimes":[]}"#;
        let o1 = Audio::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Audio>(json)?);

        Ok(())
    }
}
