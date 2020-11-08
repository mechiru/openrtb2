use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.10 Playback Methods
///
/// The following table lists the various playback methods.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum PlaybackMethod {
    /// Initiates on Page Load with Sound On
    AutoPlaySoundOn = 1,
    /// Initiates on Page Load with Sound Off by Default
    AutoPlaySoundOff,
    /// Initiates on Click with Sound On
    ClickToPlay,
    /// Initiates on Mouse-Over with Sound On
    MouseOver,
    /// Initiates on Entering Viewport with Sound On
    EnterSoundOn,
    /// Initiates on Entering Viewport with Sound Off by Default
    EnterSoundOff,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<PlaybackMethod>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<PlaybackMethod> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                PlaybackMethod::AutoPlaySoundOn,
                PlaybackMethod::AutoPlaySoundOff
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
