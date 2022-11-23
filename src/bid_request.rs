/// 3.2.1 Object: BidRequest
///
/// The top-level bid request object contains a globally unique bid request or auction ID. This id
/// attribute is required as is at least one impression object (Section 3.2.4). Other attributes in
/// this top-level object establish rules and restrictions that apply to all impressions being
/// offered.
///
/// There are also several subordinate objects that provide detailed data to potential buyers. Among
/// these are the Site and App objects, which describe the type of published media in which the
/// impression(s) appear. These objects are highly recommended, but only one applies to a given bid
/// request depending on whether the media is browser-based web content or a non-browser
/// application, respectively.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct BidRequest {
    /// string; required
    /// Unique ID of the bid request, provided by the exchange.
    #[serde(borrow)]
    pub id: std::borrow::Cow<'a, str>,

    /// object array; required
    /// Array of Imp objects (Section 3.2.4) representing the impressions offered. At least 1 Imp
    /// object is required.
    #[serde(borrow)]
    pub imp: Vec<crate::Imp>,

    /// object; recommended
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<crate::DistributionChannel>,

    /// object; recommended
    /// Details via a Device object (Section 3.2.18) about the user’s device to which the
    /// impression will be delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::Device>,

    /// object; recommended
    /// Details via a User object (Section 3.2.20) about the human user of the device; the
    /// advertising audience.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::User>,

    /// integer; default 0
    /// Indicator of test mode in which auctions are not billable, where 0 = live mode, 1 = test
    /// mode.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub test: bool,

    /// integer; default 2
    /// Auction type, where 1 = First Price, 2 = Second Price Plus. Exchange-specific auction types
    /// can be defined using values greater than 500.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub at: crate::AuctionType,

    /// integer
    /// Maximum time in milliseconds the exchange allows for bids to be received including Internet
    /// latency to avoid timeout. This value supersedes any a priori guidance from the exchange.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i32>,

    /// string array
    /// White list of buyer seats (e.g., advertisers, agencies) allowed to bid on this impression.
    /// IDs of seats and knowledge of the buyer’s customers to which they refer must be coordinated
    /// between bidders and the exchange a priori. At most, only one of wseat and bseat should be
    /// used in the same request. Omission of both implies no seat restrictions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// Block list of buyer seats (e.g., advertisers, agencies) restricted from bidding on this
    /// impression. IDs of seats and knowledge of the buyer’s customers to which they refer must be
    /// coordinated between bidders and the exchange a priori. At most, only one of wseat and bseat
    /// should be used in the same request. Omission of both implies no seat restrictions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bseat: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// integer; default 0
    /// Flag to indicate if Exchange can verify that the impressions offered represent all of the
    /// impressions available in context (e.g., all on the web page, all video spots such as
    /// pre/mid/post roll) to support road-blocking. 0 = no or unknown, 1 = yes, the impressions
    /// offered represent all that are available.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub allimps: bool,

    /// string array
    /// Array of allowed currencies for bids on this bid request using ISO-4217 alpha codes.
    /// Recommended only if the exchange accepts multiple currencies.
    // TODO: ISO-4217 alpha
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// White list of languages for creatives using ISO-639-1-alpha-2. Omission implies no specific
    /// restrictions, but buyers would be advised to consider language attribute in the Device
    /// and/or Content objects if available.
    // TODO: ISO-639-1-alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wlang: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// Blocked advertiser categories using the IAB content categories. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<crate::ContentCategory>>,

    /// string array
    /// Block list of advertisers by their domains (e.g., “ford.com”).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// Block list of applications by their platform-specific exchange- independent application
    /// identifiers. On Android, these should be bundle or package names (e.g., com.foo.mygame). On
    /// iOS, these are numeric IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bapp: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// object
    /// A Sorce object (Section 3.2.2) that provides data about the inventory source and which
    /// entity makes the final decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::Source>,

    /// object
    /// A Regs object (Section 3.2.3) that specifies any industry, legal, or governmental
    /// regulations in force for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regs: Option<crate::Regs>,

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
        assert!(serde_json::from_str::<BidRequest>("{}").is_err());

        let json = r#"{"id":"","imp":[]}"#;
        let o1 = BidRequest::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<BidRequest>(json)?);

        Ok(())
    }
}
