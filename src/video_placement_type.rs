use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.9 Video Placement Types
///
/// The following table lists the various types of video placements derived largely from the IAB
/// Digital Video Guidelines.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum VideoPlacementType {
    /// In-Stream
    /// Played before, during or after the streaming video content that the consumer has requested
    /// (e.g., Pre-roll, Mid-roll, Post-roll).
    InStream = 1,
    /// In-Banner
    /// Exists within a web banner that leverages the banner space to deliver a video experience as
    /// opposed to another static or rich media format. The format relies on the existence of
    /// display ad inventory on the page for its delivery.
    InBanner,
    /// In-Article
    /// Loads and plays dynamically between paragraphs of editorial content; existing as a
    /// standalone branded message.
    InArticle,
    /// In-Feed
    /// Found in content, social, or product feeds.
    InFeed,
    /// Interstitial/Slider/Floating
    /// Covers the entire or a portion of screen area, but is always on screen while displayed
    /// (i.e. cannot be scrolled out of view). Note that a full-screen interstitial (e.g., in
    /// mobile) can be distinguished from a floating/slider unit by the imp.instl field.
    Floating,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<VideoPlacementType>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<VideoPlacementType> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![VideoPlacementType::InStream, VideoPlacementType::InBanner]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
