use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.23 IP Location Services
///
/// The following table lists the services and/or vendors used for resolving IP addresses to
/// geolocations.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum IpLocationService {
    /// ip2location
    Ip2Location = 1,
    /// Neustar (Quova)
    Neustar,
    /// MaxMind
    MaxMind,
    /// NetAcuity (Digital Element)
    NetAcuity,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<IpLocationService>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<IpLocationService> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![IpLocationService::Ip2Location, IpLocationService::Neustar]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
