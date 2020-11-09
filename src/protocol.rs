use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.8 Protocols
///
/// The following table lists the options for the various bid response protocols that could be
/// supported by an exchange.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum Protocol {
    /// VAST 1.0
    Vast1 = 1,
    /// VAST 2.0
    Vast2,
    /// VAST 3.0
    Vast3,
    /// VAST 1.0 Wrapper
    Vast1Wrapper,
    /// VAST 2.0 Wrapper
    Vast2Wrapper,
    /// VAST 3.0 Wrapper
    Vast3Wrapper,
    /// VAST 4.0
    Vast4,
    /// VAST 4.0 Wrapper
    Vast4Wrapper,
    /// DAAST 1.0
    Daast1,
    /// DAAST 1.0 Wrapper
    Daast1Wrapper,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Protocol>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<Protocol> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![Protocol::Vast1, Protocol::Vast2]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
