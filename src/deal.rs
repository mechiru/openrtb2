/// 3.2.12 Object: Deal
///
/// This object constitutes a specific deal that was struck a priori between a buyer and a seller.
/// Its presence with the Pmp collection indicates that this impression is available under the terms
/// of that deal. Refer to Section 7.3 for more details.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Deal {
    /// string; required
    /// A unique identifier for the direct deal.
    #[serde(borrow)]
    pub id: String,

    /// float; default 0
    /// Minimum bid for this impression expressed in CPM.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub bidfloor: f64,

    /// string; default ”USD”
    /// Currency specified using ISO-4217 alpha codes. This may be different from bid currency
    /// returned by bidder if this is allowed by the exchange.
    // TODO: ISO-4217 alpha
    #[serde(
        borrow,
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default"
    )]
    pub bidfloorcur: String,

    /// integer
    /// Optional override of the overall auction type of the bid request, where 1 = First Price, 2
    /// = Second Price Plus, 3 = the value passed in bidfloor is the agreed upon deal price.
    /// Additional auction types can be defined by the exchange.
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub at: crate::AuctionType,

    /// string array
    /// Whitelist of buyer seats (e.g., advertisers, agencies) allowed to bid on this deal. IDs of
    /// seats and the buyer’s customers to which they refer must be coordinated between bidders and
    /// the exchange a priori. Omission implies no seat restrictions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wseat: Option<Vec<String>>,

    /// string array
    /// Array of advertiser domains (e.g., advertiser.com) allowed to bid on this deal. Omission
    /// implies no advertiser restrictions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wadomain: Option<Vec<String>>,

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
        assert!(serde_json::from_str::<Deal>("{}").is_err());

        let json = r#"{"id":""}"#;
        let o1 = Deal::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Deal>(json)?);

        Ok(())
    }
}
