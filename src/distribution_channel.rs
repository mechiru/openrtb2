/// 3.2.13 [`Site`], 3.2.14 [`App`].
///
/// [`Site`]: ./struct.Site.html
/// [`App`]: ./struct.App.html
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DistributionChannel {
    /// object; recommended
    /// Details via a Site object (Section 3.2.13) about the publisher’s website. Only applicable
    /// and recommended for websites.
    Site(crate::Site),
    /// object; recommended
    /// Details via an App object (Section 3.2.14) about the publisher’s app (i.e., non-browser
    /// applications). Only applicable and recommended for apps.
    App(crate::App),
}

impl DistributionChannel {
    /// Returns true if the `DistributionChannel` is a Site. Returns false otherwise.
    ///
    /// ```
    /// # use openrtb2::DistributionChannel;
    /// assert!(DistributionChannel::Site(Default::default()).is_site());
    /// assert!(!DistributionChannel::App(Default::default()).is_site());
    /// ```
    pub fn is_site(&self) -> bool {
        self.as_site().is_some()
    }

    /// If the `DistributionChannel` is a Site, returns the associated `Site`. Returns None
    /// otherwise.
    pub fn as_site(&self) -> Option<&crate::Site> {
        match self {
            Self::Site(ref site) => Some(site),
            _ => None,
        }
    }

    /// If the `DistributionChannel` is a Site, returns the associated mutable `Site`. Returns None
    /// otherwise.
    pub fn as_site_mut(&mut self) -> Option<&mut crate::Site> {
        match self {
            Self::Site(ref mut site) => Some(site),
            _ => None,
        }
    }

    /// Returns true if the `DistributionChannel` is an App. Returns false otherwise.
    ///
    /// ```
    /// # use openrtb2::DistributionChannel;
    /// assert!(!DistributionChannel::Site(Default::default()).is_app());
    /// assert!(DistributionChannel::App(Default::default()).is_app());
    /// ```
    pub fn is_app(&self) -> bool {
        self.as_app().is_some()
    }

    /// If the `DistributionChannel` is an App, returns the associated `App`. Returns None
    /// otherwise.
    pub fn as_app(&self) -> Option<&crate::App> {
        match self {
            Self::App(ref app) => Some(app),
            _ => None,
        }
    }

    /// If the `DistributionChannel` is an App, returns the associated mutable `App`. Returns None
    /// otherwise.
    pub fn as_app_mut(&mut self) -> Option<&mut crate::App> {
        match self {
            Self::App(ref mut app) => Some(app),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<DistributionChannel>("{}").is_err());

        let j1 = r#"{"site":{}}"#;
        let o1 = DistributionChannel::Site(Default::default());
        assert_eq!(serde_json::to_string(&o1)?, j1);
        assert_eq!(o1, serde_json::from_str::<DistributionChannel>(j1)?);

        let j2 = r#"{"app":{}}"#;
        let o2 = DistributionChannel::App(Default::default());
        assert_eq!(serde_json::to_string(&o2)?, j2);
        assert_eq!(o2, serde_json::from_str::<DistributionChannel>(j2)?);

        Ok(())
    }
}
