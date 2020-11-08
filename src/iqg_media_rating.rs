use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.19 IQG Media Ratings
///
/// The following table lists the media ratings used in describing content based on the IQG 2.1
/// categorization. Refer to www.iab.com/guidelines/digital-video-suite for more information.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum IqgMediaRating {
    /// All Audiences
    AllAudiences = 1,
    /// Everyone Over 12
    EveryoneOver12,
    /// Mature Audiences
    MatureAudiences,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<IqgMediaRating>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<IqgMediaRating> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![IqgMediaRating::AllAudiences, IqgMediaRating::EveryoneOver12]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
