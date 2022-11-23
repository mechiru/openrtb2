/// 3.2.10 Object: Format
///
/// This object represents an allowed size (i.e., height and width combination) or Flex Ad
/// parameters for a banner impression. These are typically used in an array where multiple sizes
/// are permitted. It is recommended that either the w/h pair or the wratio/hratio/wmin set (i.e.,
/// for Flex Ads) be specified.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Format<'a> {
    /// integer
    /// Width in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// integer
    /// Height in device independent pixels (DIPS).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// integer
    /// Relative width when expressing size as a ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// integer
    /// Relative height when expressing size as a ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// integer
    /// The minimum width in device independent pixels (DIPS) at which the ad will be displayed the
    /// size is expressed as a ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

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
        let o1 = Format::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Format>(json)?);

        Ok(())
    }
}
