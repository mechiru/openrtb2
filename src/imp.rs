/// 3.2.4 Object: Imp
///
/// This object describes an ad placement or impression being auctioned. A single bid request can
/// include multiple Imp objects, a use case for which might be an exchange that supports selling
/// all ad positions on a given page. Each Imp object has a required ID so that bids can reference
/// them individually.
///
/// The presence of Banner (Section 3.2.6), Video (Section 3.2.7), and/or Native (Section 3.2.9)
/// objects subordinate to the Imp object indicates the type of impression being offered. The
/// publisher can choose one such type which is the typical case or mix them at their discretion.
/// However, any given bid for the impression must conform to one of the offered types.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Imp<'a> {
    /// string; required
    /// A unique identifier for this impression within the context of the bid request (typically,
    /// starts with 1 and increments.
    #[serde(borrow)]
    pub id: std::borrow::Cow<'a, str>,

    /// object array
    /// An array of Metric object (Section 3.2.5).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<Vec<crate::Metric<'a>>>,

    /// object
    /// A Banner object (Section 3.2.6); required if this impression is offered as a banner ad
    /// opportunity.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<crate::Banner<'a>>,

    /// object
    /// A Video object (Section 3.2.7); required if this impression is offered as a video ad
    /// opportunity.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub video: Option<crate::Video<'a>>,

    /// object
    /// An Audio object (Section 3.2.8); required if this impression is offered as an audio ad
    /// opportunity.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<crate::Audio<'a>>,

    /// object
    /// A Native object (Section 3.2.9); required if this impression is offered as a native ad
    /// opportunity.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub native: Option<crate::Native<'a>>,

    /// object
    /// A Pmp object (Section 3.2.11) containing any private marketplace deals in effect for this
    /// impression.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub pmp: Option<crate::Pmp<'a>>,

    /// string
    /// Name of ad mediation partner, SDK technology, or player responsible for rendering ad
    /// (typically video or mobile). Used by some ad servers to customize ad code by partner.
    /// Recommended for video and/or apps.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub displaymanager: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Version of ad mediation partner, SDK technology, or player responsible for rendering ad
    /// (typically video or mobile). Used by some ad servers to customize ad code by partner.
    /// Recommended for video and/or apps.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub displaymanagerver: Option<std::borrow::Cow<'a, str>>,

    /// integer; default 0
    /// 1 = the ad is interstitial or full screen, 0 = not interstitial.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub instl: bool,

    /// string
    /// Identifier for specific ad placement or ad tag that was used to initiate the auction. This
    /// can be useful for debugging of any issues, or for optimization by the buyer.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tagid: Option<std::borrow::Cow<'a, str>>,

    /// float; default 0
    /// Minimum bid for this impression expressed in CPM.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub bidfloor: f64,

    /// string; default “USD”
    /// Currency specified using ISO-4217 alpha codes. This may be different from bid currency
    /// returned by bidder if this is allowed by the exchange.
    // TODO: ISO-4217 alpha
    #[serde(
        borrow,
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default"
    )]
    pub bidfloorcur: std::borrow::Cow<'a, str>,

    /// integer
    /// Indicates the type of browser opened upon clicking the creative in an app, where 0 =
    /// embedded, 1 = native. Note that the Safari View Controller in iOS 9.x devices is considered
    /// a native browser for purposes of this attribute.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub clickbrowser: Option<bool>,

    /// integer
    /// Flag to indicate if the impression requires secure HTTPS URL creative assets and markup,
    /// where 0 = non-secure, 1 = secure. If omitted, the secure state is unknown, but non-secure
    /// HTTP support can be assumed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub secure: Option<bool>,

    /// string array
    /// Array of exchange-specific names of supported iframe busters.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub iframebuster: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// integer
    /// Advisory as to the number of seconds that may elapse between the auction and the actual
    /// impression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,

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
        assert!(serde_json::from_str::<Imp>("{}").is_err());

        let json = r#"{"id":""}"#;
        let o1 = Imp::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Imp>(json)?);

        Ok(())
    }
}
