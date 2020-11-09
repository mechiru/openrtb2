use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.24 No-Bid Reason Codes
///
/// The following table lists the options for a bidder to signal the exchange as to why it did not
/// offer a bid for the impression.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum NoBidReason {
    /// Unknown Error
    UnknownError = 0,
    /// Technical Error
    TechnicalError,
    /// Invalid Request
    InvalidRequest,
    /// Known Web Spider
    KnownWebSpider,
    /// Suspected Non-Human Traffic
    SuspectedNonHumanTraffic,
    /// Cloud, Data center, or Proxy IP
    CloudDataCenterProxyIp,
    /// Unsupported Device
    UnsupportedDevice,
    /// Blocked Publisher or Site
    BlockedPublisher,
    /// Unmatched User
    UnmatchedUser,
    /// Daily Reader Cap Met
    DailyReaderCap,
    /// Daily Domain Cap Met
    DailyDomainCap,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<NoBidReason>("-1").is_err());

        let json = "[0,1]";
        let e1: Vec<NoBidReason> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![NoBidReason::UnknownError, NoBidReason::TechnicalError]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
