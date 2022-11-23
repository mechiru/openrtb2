/// 3.2.11 Object: Pmp
///
/// This object is the private marketplace container for direct deals between buyers and sellers
/// that may pertain to this impression. The actual deals are represented as a collection of Deal
/// objects. Refer to Section 7.3 for more details.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Pmp<'a> {
    /// integer; default 0
    /// Indicator of auction eligibility to seats named in the Direct Deals object, where 0 = all
    /// bids are accepted, 1 = bids are restricted to the deals specified and the terms thereof.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub private_auction: bool,

    /// object array
    /// Array of Deal (Section 3.2.12) objects that convey the specific deals applicable to this
    /// impression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<crate::Deal<'a>>>,

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
        let json = "{}";
        let o1 = Pmp::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Pmp>(json)?);

        Ok(())
    }
}
