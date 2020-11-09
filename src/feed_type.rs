use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.16 Feed Types
///
/// The following table lists the types of feeds, typically for audio.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum FeedType {
    /// Music Service
    MusicService = 1,
    /// FM/AM Broadcast
    Broadcast,
    /// Podcast
    Podcast,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<FeedType>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<FeedType> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![FeedType::MusicService, FeedType::Broadcast]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
