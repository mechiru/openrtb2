/// 3.2.14 Object: App
///
/// This object should be included if the ad supported content is a non-browser application
/// (typically in mobile) as opposed to a website. A bid request must not contain both an App and a
/// Site object. At a minimum, it is useful to provide an App ID or bundle, but this is not strictly
/// required.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct App<'a> {
    /// string; recommended
    /// Exchange-specific app ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// App name (may be aliased at the publisher’s request).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// A platform-specific application identifier intended to be unique to the app and independent
    /// of the exchange. On Android, this should be a bundle or package name (e.g.,
    /// com.foo.mygame). On iOS, it is typically a numeric ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Domain of the app (e.g., “mygame.foo.com”).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// App store URL for an installed app; for IQG 2.1 compliance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories of the app. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// string array
    /// Array of IAB content categories that describe the current section of the app. Refer to List
    /// 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<crate::ContentCategory>>,

    /// string array
    /// Array of IAB content categories that describe the current page or view of the app. Refer to
    /// List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<crate::ContentCategory>>,

    /// string
    /// Application version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Indicates if the app has a privacy policy, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub privacypolicy: Option<bool>,

    /// integer
    /// 0 = app is free, 1 = the app is a paid version.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub paid: Option<bool>,

    /// object
    /// Details about the Publisher (Section 3.2.15) of the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<crate::Publisher<'a>>,

    /// object
    /// Details about the Content (Section 3.2.16) within the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::Content<'a>>,

    /// string
    /// Comma separated list of keywords about the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = App::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<App>(json)?);

        Ok(())
    }
}
