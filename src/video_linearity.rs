use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.7 Video Linearity
///
/// The following table indicates the options for video linearity. “In-stream” or “linear” video
/// refers to pre- roll, post-roll, or mid-roll video ads where the user is forced to watch ad in
/// order to see the video content. “Overlay” or “non-linear” refer to ads that are shown on top of
/// the video content.
///
/// This OpenRTB table has values derived from the Inventory Quality Guidelines (IQG). Practitioners
/// should keep in sync with updates to the IQG values.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum VideoLinearity {
    /// Linear / In-Stream
    Linear = 1,
    /// Non-Linear / Overlay
    NonLinear,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<VideoLinearity>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<VideoLinearity> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![VideoLinearity::Linear, VideoLinearity::NonLinear]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
