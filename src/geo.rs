/// 3.2.19 Object: Geo
///
/// This object encapsulates various methods for specifying a geographic location. When subordinate
/// to a Device object, it indicates the location of the device which can also be interpreted as the
/// user’s current location. When subordinate to a User object, it indicates the location of the
/// user’s home base (i.e., not necessarily their current location).
///
/// The lat/lon attributes should only be passed if they conform to the accuracy depicted in the
/// type attribute. For example, the centroid of a geographic region such as postal code should not
/// be passed.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Geo<'a> {
    /// float
    /// Latitude from -90.0 to +90.0, where negative is south.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lat: Option<f32>,

    /// float
    /// Longitude from -180.0 to +180.0, where negative is west.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lon: Option<f32>,

    /// integer
    /// Source of location data; recommended when passing lat/lon. Refer to List 5.20.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::LocationType>,

    /// integer
    /// Estimated location accuracy in meters; recommended when lat/lon are specified and derived
    /// from a device’s location services (i.e., type = 1). Note that this is the accuracy as
    /// reported from the device. Consult OS specific documentation (e.g., Android, iOS) for exact
    /// interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<i32>,

    /// integer
    /// Number of seconds since this geolocation fix was established. Note that devices may cache
    /// location data across multiple fetches. Ideally, this value should be from the time the
    /// actual fix was taken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<i32>,

    /// integer
    /// Service or provider used to determine geolocation from IP address if applicable (i.e., type
    /// = 2). Refer to List 5.23.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipservice: Option<crate::IpLocationService>,

    /// string
    /// Country code using ISO-3166-1-alpha-3.
    // TODO: ISO-3166-1-alpha-3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Region code using ISO-3166-2; 2-letter state code if USA.
    // TODO: ISO-3166-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Region of a country using FIPS 10-4 notation. While OpenRTB supports this attribute, it has
    /// been withdrawn by NIST in 2008.
    // TODO: FIPS 10-4 notation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regionfips104: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Google metro code; similar to but not exactly Nielsen DMAs. See Appendix A for a link to
    /// the codes.
    // TODO: Google metro code http://code.google.com/apis/adwords/docs/appendix/metrocodes.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metro: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// City using United Nations Code for Trade & Transport Locations. See Appendix A for a link
    /// to the codes.
    // TODO: U.N. Code for Trade and Transport Locations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Zip or postal code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Local time as the number +/- of minutes from UTC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utcoffset: Option<i32>,

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
        let o1 = Geo::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Geo>(json)?);

        Ok(())
    }
}
