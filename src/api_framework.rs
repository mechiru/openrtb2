use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.6 API Frameworks
///
/// The following table is a list of API frameworks supported by the publisher.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1 = 1,
    /// VPAID 2.0
    Vpaid2,
    /// MRAID-1
    Mraid1,
    /// ORMMA
    Ormma,
    /// MRAID-2
    Mraid2,
    /// MRAID-3
    Mraid3,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ApiFramework>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<ApiFramework> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![ApiFramework::Vpaid1, ApiFramework::Vpaid2]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
