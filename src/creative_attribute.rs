use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.3 Creative Attributes
///
/// The following table specifies a standard list of creative attributes that can describe an ad
/// being served or serve as restrictions of thereof.
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
pub enum CreativeAttribute {
    /// Audio Ad (Auto-Play)
    AudioAd_AutoPlay = 1,
    /// Audio Ad (User Initiated)
    AudioAd_UserInitiated,
    /// Expandable (Automatic)
    Expandable_Automatic,
    /// Expandable (User Initiated - Click)
    Expandable_UserInitiated_Click,
    /// Expandable (User Initiated - Rollover)
    Expandable_UserInitiated_Rollover,
    /// In-Banner Video Ad (Auto-Play)
    InBannerVideoAd_AutoPlay,
    /// In-Banner Video Ad (User Initiated)
    InBannerVideoAd_UserInitiated,
    /// Pop (e.g., Over, Under, or Upon Exit)
    Pop,
    /// Provocative or Suggestive Imagery
    ProvocativeOrSuggestiveImagery,
    /// Shaky, Flashing, Flickering, Extreme Animation, Smileys
    Annoying,
    /// Surveys
    Surveys,
    /// Text Only
    TextOnly,
    /// User Interactive (e.g., Embedded Games)
    UserInteractive,
    /// Windows Dialog or Alert Style
    WindowsDialogOrAlertStyle,
    /// Has Audio On/Off Button
    HasAudioOnOffButton,
    /// Ad Provides Skip Button (e.g. VPAID-rendered skip button on pre-roll video)
    AdProvidesSkipButton,
    /// Adobe Flash
    AdobeFlash,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<CreativeAttribute>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<CreativeAttribute> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                CreativeAttribute::AudioAd_AutoPlay,
                CreativeAttribute::AudioAd_UserInitiated
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
