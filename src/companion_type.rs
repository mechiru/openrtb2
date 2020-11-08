use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.14 Companion Types
///
/// The following table lists the options to indicate markup types allowed for companion ads that
/// apply to video and audio ads. This table is derived from VAST 2.0+ and DAAST 1.0 specifications.
/// Refer to www.iab.com/guidelines/digital-video-suite for more information.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum CompanionType {
    /// Static Resource
    Static = 1,
    /// HTML Resource
    Html,
    /// iframe Resource
    Iframe,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<CompanionType>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<CompanionType> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![CompanionType::Static, CompanionType::Html]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
