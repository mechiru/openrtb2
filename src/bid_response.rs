/// 4.2.1 Object: BidResponse
///
/// This object is the top-level bid response object (i.e., the unnamed outer JSON object). The id
/// attribute is a reflection of the bid request ID for logging purposes. Similarly, bidid is an
/// optional response tracking ID for bidders. If specified, it can be included in the subsequent
/// win notice call if the bidder wins. At least one seatbid object is required, which contains at
/// least one bid for an impression. Other attributes are optional.
///
/// To express a “no-bid”, the options are to return an empty response with HTTP 204. Alternately if
/// the bidder wishes to convey to the exchange a reason for not bidding, just a BidResponse object
/// is returned with a reason code in the nbr attribute.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct BidResponse {
    /// string; required
    /// ID of the bid request to which this is a response.
    #[serde(borrow)]
    pub id: std::borrow::Cow<'a, str>,

    /// object array
    /// Array of seatbid objects; 1+ required if a bid is to be made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<crate::SeatBid>>,

    /// string
    /// Bidder generated response ID to assist with logging/tracking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bidid: Option<std::borrow::Cow<'a, str>>,

    /// string; default “USD”
    /// Bid currency using ISO-4217 alpha codes.
    // TODO: ISO-4217 alpha
    #[serde(
        borrow,
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default"
    )]
    pub cur: std::borrow::Cow<'a, str>,

    /// string
    /// Optional feature to allow a bidder to set data in the exchange’s cookie. The string must be
    /// in base85 cookie safe characters and be in any format. Proper JSON encoding must be used to
    /// include “escaped” quotation marks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customdata: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Reason for not bidding. Refer to List 5.24.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbr: Option<crate::NoBidReason>,

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
        assert!(serde_json::from_str::<BidResponse>("{}").is_err());

        let json = r#"{"id":""}"#;
        let o1 = BidResponse::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<BidResponse>(json)?);

        Ok(())
    }
}
