use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.11 Playback Cessation Modes
///
/// The following table lists the various modes for when playback terminates.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum PlaybackCessationMode {
    /// On Video Completion or when Terminated by User
    CompletionOrUser = 1,
    /// On Leaving Viewport or when Terminated by User
    LeavingOrUser,
    ///  On Leaving Viewport Continues as a Floating/Slider Unit until Video Completion or when
    /// Terminated by User
    LeavingContinuesOrUser,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<PlaybackCessationMode>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<PlaybackCessationMode> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                PlaybackCessationMode::CompletionOrUser,
                PlaybackCessationMode::LeavingOrUser
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
