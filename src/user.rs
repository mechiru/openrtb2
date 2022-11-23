/// 3.2.20 Object: User
///
/// This object contains information known or derived about the human user of the device (i.e., the
/// audience for advertising). The user id is an exchange artifact and may be subject to rotation or
/// other privacy policies. However, this user ID must be stable long enough to serve reasonably as
/// the basis for frequency capping and retargeting.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct User<'a> {
    /// string; recommended
    /// Exchange-specific ID for the user. At least one of id or buyeruid is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string; recommended
    /// Buyer-specific ID for the user as mapped by the exchange for the buyer. At least one of
    /// buyeruid or id is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Year of birth as a 4-digit integer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,

    /// string
    /// Gender, where “M” = male, “F” = female, “O” = known to be other (i.e., omitted is unknown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<crate::Gender>,

    /// string
    /// Comma separated list of keywords, interests, or intent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Optional feature to pass bidder data that was set in the exchange’s cookie. The string must
    /// be in base85 cookie safe characters and be in any format. Proper JSON encoding must be used
    /// to include “escaped” quotation marks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customdata: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Location of the user’s home base defined by a Geo object (Section 3.2.19). This is not
    /// necessarily their current location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<crate::Geo<'a>>,

    /// object array
    /// Additional user data. Each Data object (Section 3.2.21) represents a different data source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::Data<'a>>>,

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
        let o1 = User::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<User>(json)?);

        Ok(())
    }
}
