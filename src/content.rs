/// 3.2.16 Object: Content
///
/// This object describes the content in which the impression will appear, which may be syndicated
/// or non- syndicated content. This object may be useful when syndicated content contains
/// impressions and does not necessarily match the publisher’s general content. The exchange might
/// or might not have knowledge of the page where the content is running, as a result of the
/// syndication method. For example might be a video impression embedded in an iframe on an unknown
/// web property or device.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Content<'a> {
    /// string
    /// ID uniquely identifying the content.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Episode number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,

    /// string
    /// Content title.
    /// Video Examples: “Search Committee” (television), “A New Hope” (movie), or “Endgame” (made
    /// for web).
    /// Non-Video Example: “Why an Antarctic Glacier Is Melting So Quickly” (Time magazine
    /// article).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub title: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Content series.
    /// Video Examples: “The Office” (television), “Star Wars” (movie), or “Arby ‘N’ The Chief”
    /// (made for web).
    /// Non-Video Example: “Ecocentric” (Time Magazine blog).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub series: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Content season (e.g., “Season 3”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub season: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Artist credited with the content.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Genre that best describes the content (e.g., rock, pop, etc).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Album to which the content belongs; typically for audio.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub album: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// International Standard Recording Code conforming to ISO-3901.
    // TODO: ISO- 3901
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub isrc: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Details about the content Producer (Section 3.2.17).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<crate::Producer<'a>>,

    /// string
    /// URL of the content, for buy-side contextualization or review.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub url: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories that describe the content producer. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// integer
    /// Production quality. Refer to List 5.13.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prodq: Option<crate::ProductionQuality>,

    /// integer; DEPRECATED
    /// Video quality. Refer to List 5.13.
    #[deprecated(since = "0.1.0", note = "Please use the prodq field instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videoquality: Option<crate::ProductionQuality>,

    /// integer
    /// Type of content (game, video, text, etc.). Refer to List 5.18.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<crate::ContentContext>,

    /// string
    /// Content rating (e.g., MPAA).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub contentrating: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// User rating of the content (e.g., number of stars, likes, etc.).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub userrating: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Media rating per IQG guidelines. Refer to List 5.19.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<crate::IqgMediaRating>,

    /// string
    /// Comma separated list of keywords describing the content.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// 0 = not live, 1 = content is live (e.g., stream, live blog).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub livestream: Option<bool>,

    /// integer
    /// 0 = indirect, 1 = direct.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub sourcerelationship: Option<bool>,

    /// integer
    /// Length of content in seconds; appropriate for video or audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// string
    /// Content language using ISO-639-1-alpha-2.
    // TODO: ISO-639-1-alpha-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,

    /// integer
    /// Indicator of whether or not the content is embeddable (e.g., an embeddable video player),
    /// where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub embeddable: Option<bool>,

    /// object array
    /// Additional content data. Each Data object (Section 3.2.21) represents a different data
    /// source.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::Data<'a>>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<json_ext::Object<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Content::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Content>(json)?);

        Ok(())
    }
}
