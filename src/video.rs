/// 3.2.7 Object: Video
///
/// This object represents an in-stream video impression. Many of the fields are non-essential for
/// minimally viable transactions, but are included to offer fine control when needed. Video in
/// OpenRTB generally assumes compliance with the VAST standard. As such, the notion of companion
/// ads is supported by optionally including an array of Banner objects (refer to the Banner object
/// in Section 3.2.6) that define these companion ads.
///
/// The presence of a Video as a subordinate of the Imp object indicates that this impression is
/// offered as a video type impression. At the publisher’s discretion, that same impression may also
/// be offered as banner, audio, and/or native by also including as Imp subordinates objects of
/// those types. However, any given bid for the impression must conform to one of the offered types.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Video {
    /// string array; required
    /// Content MIME types supported (e.g., “video/x-ms-wmv”, “video/mp4”).
    #[serde(borrow)]
    pub mimes: Vec<std::borrow::Cow<'a, str>>,

    /// integer; recommended
    /// Minimum video ad duration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minduration: Option<i32>,

    /// integer; recommended
    /// Maximum video ad duration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<i32>,

    /// integer array; recommended
    /// Array of supported video protocols. Refer to List 5.8. At least one supported protocol must
    /// be specified in either the protocol or protocols attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<crate::Protocol>>,

    /// integer; recommended
    /// Width of the video player in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// integer; recommended
    /// Height of the video player in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// integer; recommended
    /// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll ad placements.
    /// Refer to List 5.12 for additional generic values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startdelay: Option<crate::StartDelay>,

    /// integer
    /// Placement type for the impression. Refer to List 5.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<crate::VideoPlacementType>,

    /// integer
    /// Indicates if the impression must be linear, nonlinear, etc. If none specified, assume all
    /// are allowed. Refer to List 5.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linearity: Option<crate::VideoLinearity>,

    /// integer
    /// Indicates if the player will allow the video to be skipped, where 0 = no, 1 = yes.
    /// If a bidder sends markup/creative that is itself skippable, the Bid object should include
    /// the attr array with an element of 16 indicating skippable video. Refer to List 5.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub skip: Option<bool>,

    /// integer; default 0
    /// Videos of total duration greater than this number of seconds can be skippable; only
    /// applicable if the ad is skippable.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub skipmin: i32,

    /// integer; default 0
    /// Number of seconds a video must play before skipping is enabled; only applicable if the ad
    /// is skippable.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub skipafter: i32,

    /// integer
    /// If multiple ad impressions are offered in the same bid request, the sequence number will
    /// allow for the coordinated delivery of multiple creatives.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// integer array
    /// Blocked creative attributes. Refer to List 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<crate::CreativeAttribute>>,

    /// integer
    /// Maximum extended ad duration if extension is allowed. If blank or 0, extension is not
    /// allowed. If -1, extension is allowed, and there is no time limit imposed. If greater than
    /// 0, then the value represents the number of seconds of extended play supported beyond the
    /// maxduration value.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub maxextended: crate::MaxExtendedAdDuration,

    /// integer
    /// Minimum bit rate in Kbps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<i32>,

    /// integer
    /// Maximum bit rate in Kbps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<i32>,

    /// integer; default 1
    /// Indicates if letter-boxing of 4:3 content into a 16:9 window is allowed, where 0 = no, 1 =
    /// yes.
    #[serde(
        default = "default_boxingallowed",
        skip_serializing_if = "is_default_boxingallowed",
        with = "crate::serde::i32_as_bool"
    )]
    pub boxingallowed: bool,

    /// integer array
    /// Playback methods that may be in use. If none are specified, any method may be used. Refer
    /// to List 5.10. Only one method is typically used in practice. As a result, this array may be
    /// converted to an integer in a future version of the specification. It is strongly advised to
    /// use only the first element of this array in preparation for this change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playbackmethod: Option<Vec<crate::PlaybackMethod>>,

    /// integer
    /// The event that causes playback to end. Refer to List 5.11.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playbackend: Option<crate::PlaybackCessationMode>,

    /// integer array
    /// Supported delivery methods (e.g., streaming, progressive). If none specified, assume all
    /// are supported. Refer to List 5.15.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<crate::ContentDeliveryMethod>>,

    /// integer
    /// Ad position on screen. Refer to List 5.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<crate::AdPosition>,

    /// object array
    /// Array of Banner objects (Section 3.2.6) if companion ads are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companionad: Option<Vec<crate::Banner>>,

    /// integer array
    /// List of supported API frameworks for this impression. Refer to List 5.6. If an API is not
    /// explicitly listed, it is assumed not to be supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<crate::ApiFramework>>,

    /// integer array
    /// Supported VAST companion ad types. Refer to List 5.14. Recommended if companion Banner
    /// objects are included via the companionad array. If one of these banners will be rendered as
    /// an end-card, this can be specified using the vcm attribute with the particular banner
    /// (Section 3.2.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companiontype: Option<Vec<crate::CompanionType>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            mimes: Default::default(),
            minduration: Default::default(),
            maxduration: Default::default(),
            protocols: Default::default(),
            w: Default::default(),
            h: Default::default(),
            startdelay: Default::default(),
            placement: Default::default(),
            linearity: Default::default(),
            skip: Default::default(),
            skipmin: Default::default(),
            skipafter: Default::default(),
            sequence: Default::default(),
            battr: Default::default(),
            maxextended: Default::default(),
            minbitrate: Default::default(),
            maxbitrate: Default::default(),
            boxingallowed: default_boxingallowed(),
            playbackmethod: Default::default(),
            playbackend: Default::default(),
            delivery: Default::default(),
            pos: Default::default(),
            companionad: Default::default(),
            api: Default::default(),
            companiontype: Default::default(),
            ext: Default::default(),
        }
    }
}

fn default_boxingallowed() -> bool {
    true
}

fn is_default_boxingallowed(v: &bool) -> bool {
    *v == default_boxingallowed()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Video>("{}").is_err());

        let json = r#"{"mimes":[]}"#;
        let o1 = Video::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Video>(json)?);

        Ok(())
    }
}
