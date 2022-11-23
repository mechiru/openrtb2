/// 3.2.13 Object: Site
///
/// This object should be included if the ad supported content is a website as opposed to a
/// non-browser application. A bid request must not contain both a Site and an App object. At a
/// minimum, it is useful to provide a site ID or page URL, but this is not strictly required.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Site {
    /// string; recommended
    /// Exchange-specific site ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Site name (may be aliased at the publisher’s request).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Domain of the site (e.g., “mysite.foo.com”).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories of the site. Refer to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<crate::ContentCategory>>,

    /// string array
    /// Array of IAB content categories that describe the current section of the site. Refer to
    /// List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<crate::ContentCategory>>,

    /// string array
    /// Array of IAB content categories that describe the current page or view of the site. Refer
    /// to List 5.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<crate::ContentCategory>>,

    /// string
    /// URL of the page where the impression will be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Referrer URL that caused navigation to the current page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Search string that caused navigation to the current page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Indicates if the site has been programmed to optimize layout when viewed on mobile devices,
    /// where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub mobile: Option<bool>,

    /// integer
    /// Indicates if the site has a privacy policy, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub privacypolicy: Option<bool>,

    /// object
    /// Details about the Publisher (Section 3.2.15) of the site.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<crate::Publisher>,

    /// object
    /// Details about the Content (Section 3.2.16) within the site.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::Content>,

    /// string
    /// Comma separated list of keywords about the site.
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
        let o1 = Site::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Site>(json)?);

        Ok(())
    }
}
