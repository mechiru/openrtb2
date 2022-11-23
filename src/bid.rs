/// 4.2.3 Object: Bid
///
/// A SeatBid object contains one or more Bid objects, each of which relates to a specific
/// impression in the bid request via the impid attribute and constitutes an offer to buy that
/// impression for a given price.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Bid {
    /// string; required
    /// Bidder generated bid ID to assist with logging/tracking.
    #[serde(borrow)]
    pub id: std::borrow::Cow<'a, str>,

    /// string; required
    /// ID of the Imp object in the related bid request.
    #[serde(borrow)]
    pub impid: std::borrow::Cow<'a, str>,

    /// float; required
    /// Bid price expressed as CPM although the actual transaction is for a unit impression only.
    /// Note that while the type indicates float, integer math is highly recommended when handling
    /// currencies (e.g., BigDecimal in Java).
    pub price: f64,

    /// string
    /// Win notice URL called by the exchange if the bid wins (not necessarily indicative of a
    /// delivered, viewed, or billable ad); optional means of serving ad markup. Substitution
    /// macros (Section 4.4) may be included in both the URL and optionally returned markup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nurl: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Billing notice URL called by the exchange when a winning bid becomes billable based on
    /// exchange-specific business policy (e.g., typically delivered, viewed, etc.). Substitution
    /// macros (Section 4.4) may be included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub burl: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Loss notice URL called by the exchange when a bid is known to have been lost. Substitution
    /// macros (Section 4.4) may be included. Exchange-specific policy may preclude support for
    /// loss notices or the disclosure of winning clearing prices resulting in ${AUCTION_PRICE}
    /// macros being removed (i.e., replaced with a zero-length string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lurl: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Optional means of conveying ad markup in case the bid wins; supersedes the win notice if
    /// markup is included in both. Substitution macros (Section 4.4) may be included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adm: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// ID of a preloaded ad to be served if the bid wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adid: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Advertiser domain for block list checking (e.g., “ford.com”). This can be an array of for
    /// the case of rotating creatives. Exchanges can mandate that only one domain is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adomain: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string
    /// A platform-specific application identifier intended to be unique to the app and independent
    /// of the exchange. On Android, this should be a bundle or package name (e.g.,
    /// com.foo.mygame). On iOS, it is a numeric ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// URL without cache-busting to an image that is representative of the content of the campaign
    /// for ad quality/safety checking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iurl: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Campaign ID to assist with ad quality checking; the collection of creatives for which iurl
    /// should be representative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Creative ID to assist with ad quality checking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crid: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Tactic ID to enable buyers to label bids for reporting to the exchange the tactic through
    /// which their bid was submitted. The specific usage and meaning of the tactic ID should be
    /// communicated between buyer and exchanges a priori.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tactic: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// IAB content categories of the creative. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// integer array
    /// Set of attributes describing the creative. Refer to List 5.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr: Option<Vec<crate::CreativeAttribute>>,

    /// integer
    /// API required by the markup if applicable. Refer to List 5.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<crate::ApiFramework>,

    /// integer
    /// Video response protocol of the markup if applicable. Refer to List 5.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::Protocol>,

    /// integer
    /// Creative media rating per IQG guidelines. Refer to List 5.19.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<crate::IqgMediaRating>,

    /// string
    /// Language of the creative using ISO-639-1-alpha-2. The non- standard code “xx” may also be
    /// used if the creative has no linguistic content (e.g., a banner with just a company logo).
    // TODO: ISO-639-1-alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Reference to the deal.id from the bid request if this bid pertains to a private marketplace
    /// direct deal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dealid: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Width of the creative in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// integer
    /// Height of the creative in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// integer
    /// Relative width of the creative when expressing size as a ratio. Required for Flex Ads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// integer
    /// Relative height of the creative when expressing size as a ratio. Required for Flex Ads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// integer
    /// Advisory as to the number of seconds the bidder is willing to wait between the auction and
    /// the actual impression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,

    /// object
    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Bid>("{}").is_err());

        let json = r#"{"id":"","impid":"","price":0.0}"#;
        let o1 = Bid::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Bid>(json)?);

        Ok(())
    }
}
