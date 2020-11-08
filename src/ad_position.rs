#![allow(deprecated)]

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.4 Ad Position
///
/// The following table specifies the position of the ad as a relative measure of visibility or
/// prominence. This OpenRTB table has values derived from the Inventory Quality Guidelines (IQG).
/// Practitioners should keep in sync with updates to the IQG values as published on IAB.com. Values
/// “4” - “7” apply to apps per the mobile addendum to IQG version 2.1.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum AdPosition {
    /// Unknown
    Unknown = 0,
    /// Above the Fold
    AboveTheFold,
    /// DEPRECATED - May or may not be initially visible depending on screen size/resolution.
    #[deprecated(since = "0.1.0", note = "Please use the BelowTheFold variant instead")]
    LikelyBelowTheFold,
    /// Below the Fold
    BelowTheFold,
    /// Header
    Header,
    /// Footer
    Footer,
    /// Sidebar
    Sidebar,
    /// Full Screen
    FullScreen,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<AdPosition>("-1").is_err());

        let json = "[0,1]";
        let e1: Vec<AdPosition> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![AdPosition::Unknown, AdPosition::AboveTheFold]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
