use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.13 Production Quality
///
/// The following table lists the options for content quality. These values are defined by the IAB;
/// refer to www.iab.com/wp-content/uploads/2015/03/long-form-video-final.pdf for more information.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum ProductionQuality {
    /// Unknown
    Unknown = 0,
    /// Professionally Produced
    Professional,
    /// Prosumer
    Prosumer,
    /// User Generated (UGC)
    UserGenerated,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ProductionQuality>("-1").is_err());

        let json = "[0,1]";
        let e1: Vec<ProductionQuality> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![ProductionQuality::Unknown, ProductionQuality::Professional]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
