/// 3.2.6 Object: Banner
///
/// This object represents the most general type of impression. Although the term “banner” may have
/// very specific meaning in other contexts, here it can be many things including a simple static
/// image, an expandable ad unit, or even in-banner video (refer to the Video object in Section
/// 3.2.7 for the more generalized and full featured video ad units). An array of Banner objects can
/// also appear within the Video to describe optional companion ads defined in the VAST
/// specification.
///
/// The presence of a Banner as a subordinate of the Imp object indicates that this impression is
/// offered as a banner type impression. At the publisher’s discretion, that same impression may
/// also be offered as video, audio, and/or native by also including as Imp subordinates objects of
/// those types. However, any given bid for the impression must conform to one of the offered types.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Banner<'a> {
    /// object array; recommended
    /// Array of format objects (Section 3.2.10) representing the banner sizes permitted. If none
    /// are specified, then use of the h and w attributes is highly recommended.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<crate::Format<'a>>>,

    /// integer
    /// Exact width in device independent pixels (DIPS); recommended if no format objects are
    /// specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// integer
    /// Exact height in device independent pixels (DIPS); recommended if no format objects are
    /// specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// integer; DEPRECATED
    /// Maximum width in device independent pixels (DIPS).
    #[deprecated(since = "0.1.0", note = "Please use the format field instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wmax: Option<i32>,

    /// integer; DEPRECATED
    /// Maximum height in device independent pixels (DIPS).
    #[deprecated(since = "0.1.0", note = "Please use the format field instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmax: Option<i32>,

    /// integer; DEPRECATED
    /// Minimum width in device independent pixels (DIPS).
    #[deprecated(since = "0.1.0", note = "Please use the format field instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// integer; DEPRECATED
    /// Minimum height in device independent pixels (DIPS).
    #[deprecated(since = "0.1.0", note = "Please use the format field instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,

    /// integer array
    /// Blocked banner ad types. Refer to List 5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub btype: Option<Vec<crate::BannerAdType>>,

    /// integer array
    /// Blocked creative attributes. Refer to List 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<crate::CreativeAttribute>>,

    /// integer
    /// Ad position on screen. Refer to List 5.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<crate::AdPosition>,

    /// string array
    /// Content MIME types supported. Popular MIME types may include
    /// “application/x-shockwave-flash”, “image/jpg”, and “image/gif”.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// integer
    /// Indicates if the banner is in the top frame as opposed to an iframe, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub topframe: Option<bool>,

    /// integer array
    /// Directions in which the banner may expand. Refer to List 5.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expdir: Option<Vec<crate::ExpandableDirection>>,

    /// integer array
    /// List of supported API frameworks for this impression. Refer to List 5.6. If an API is not
    /// explicitly listed, it is assumed not to be supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<crate::ApiFramework>>,

    /// string
    /// Unique identifier for this banner object. Recommended when Banner objects are used with a
    /// Video object (Section 3.2.7) to represent an array of companion ads. Values usually start
    /// at 1 and increase with each object; should be unique within an impression.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Relevant only for Banner objects used with a Video object (Section 3.2.7) in an array of
    /// companion ads. Indicates the companion banner rendering mode relative to the associated
    /// video, where 0 = concurrent, 1 = end-card.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub vcm: Option<bool>,

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
        let json = "{}";
        let o1 = Banner::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Banner>(json)?);

        Ok(())
    }
}
