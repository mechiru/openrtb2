/// 4.2.2 Object: SeatBid
///
/// A bid response can contain multiple SeatBid objects, each on behalf of a different bidder seat
/// and each containing one or more individual bids. If multiple impressions are presented in the
/// request, the group attribute can be used to specify if a seat is willing to accept any
/// impressions that it can win (default) or if it is only interested in winning any if it can win
/// them all as a group.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct SeatBid<'a> {
    /// object array; required
    /// Array of 1+ Bid objects (Section 4.2.3) each related to an impression. Multiple bids can
    /// relate to the same impression.
    #[serde(borrow)]
    pub bid: Vec<crate::Bid<'a>>,

    /// string
    /// ID of the buyer seat (e.g., advertiser, agency) on whose behalf this bid is made.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub seat: Option<std::borrow::Cow<'a, str>>,

    /// integer; default 0
    /// 0 = impressions can be won individually; 1 = impressions must be won or lost as a group.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub group: bool,

    /// object
    /// Placeholder for bidder-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<SeatBid>("{}").is_err());

        let json = r#"{"bid":[]}"#;
        let o1 = SeatBid::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<SeatBid>(json)?);

        Ok(())
    }
}
