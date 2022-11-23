/// 3.2.3 Object: Regs
///
/// This object contains any legal, governmental, or industry regulations that apply to the request.
/// The coppa flag signals whether or not the request falls under the United States Federal Trade
/// Commission’s regulations for the United States Children’s Online Privacy Protection Act
/// (“COPPA”).
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Regs<'a> {
    /// integer
    /// Flag indicating if this request is subject to the COPPA regulations established by the USA
    /// FTC, where 0 = no, 1 = yes. Refer to Section 7.5 for more information.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub coppa: Option<bool>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Regs::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Regs>(json)?);

        Ok(())
    }
}
