use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.2 Banner Ad Types
///
/// The following table indicates the types of ads that can be accepted by the exchange unless
/// restricted by publisher site settings.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd = 1,
    /// XHTML Banner Ad. (usually mobile)
    XhtmlBannerAd,
    /// JavaScript Ad; must be valid XHTML (i.e., Script Tags Included)
    JavaScriptAd,
    /// iframe
    Iframe,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<BannerAdType>("-1").is_err());

        let json = "[1,2,3,4]";
        let e1: Vec<BannerAdType> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                BannerAdType::XhtmlTextAd,
                BannerAdType::XhtmlBannerAd,
                BannerAdType::JavaScriptAd,
                BannerAdType::Iframe,
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
