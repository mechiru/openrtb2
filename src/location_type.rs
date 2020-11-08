use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.20 Location Type
///
/// The following table lists the options to indicate how the geographic information was determined.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum LocationType {
    /// GPS/Location Services
    GpsLocation = 1,
    /// IP Address
    IpAddress,
    /// User provided (e.g., registration data)
    UserProvided,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<LocationType>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<LocationType> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![LocationType::GpsLocation, LocationType::IpAddress]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
