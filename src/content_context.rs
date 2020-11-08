use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.18 Content Context
///
/// The following table lists the various options for indicating the type of content being used or
/// consumed by the user in which the impression will appear. This OpenRTB table has values derived
/// from the Inventory Quality Guidelines (IQG). Practitioners should keep in sync with updates to
/// the IQG values.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum ContentContext {
    /// Video (i.e., video file or stream such as Internet TV broadcasts)
    Video = 1,
    /// Game (i.e., an interactive software game)
    Game,
    /// Music (i.e., audio file or stream such as Internet radio broadcasts)
    Music,
    /// Application (i.e., an interactive software application)
    Application,
    /// Text (i.e., primarily textual document such as a web page, eBook, or news article)
    Text,
    /// Other (i.e., none of the other categories applies)
    Other,
    /// Unknown
    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ContentContext>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<ContentContext> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![ContentContext::Video, ContentContext::Game]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
