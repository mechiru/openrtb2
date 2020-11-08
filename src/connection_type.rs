use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.22 Connection Type
///
/// The following table lists the various options for the type of device connectivity.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum ConnectionType {
    /// Unknown
    Unknown = 0,
    /// Ethernet
    Ethernet,
    /// WIFI
    WiFi,
    /// Cellular Network – Unknown Generation
    CellUnknown,
    /// Cellular Network – 2G
    Cell2G,
    /// Cellular Network – 3G
    Cell3G,
    /// Cellular Network – 4G
    Cell4G,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ConnectionType>("-1").is_err());

        let json = "[0,1]";
        let e1: Vec<ConnectionType> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![ConnectionType::Unknown, ConnectionType::Ethernet]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
