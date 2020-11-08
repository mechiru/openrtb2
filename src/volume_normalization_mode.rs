use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.17 Volume Normalization Modes
///
/// The following table lists the types of volume normalization modes, typically for audio.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum VolumeNormalizationMode {
    /// None
    None = 0,
    /// Ad Volume Average Normalized to Content
    AverageVolume,
    /// Ad Volume Peak Normalized to Content
    PeakVolume,
    /// Ad Loudness Normalized to Content
    Loudness,
    /// Custom Volume Normalization
    CustomVolume,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<VolumeNormalizationMode>("-1").is_err());

        let json = "[0,1]";
        let e1: Vec<VolumeNormalizationMode> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                VolumeNormalizationMode::None,
                VolumeNormalizationMode::AverageVolume,
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
