use serde::{Deserialize, Serialize};

/// 3.2.18 Object: Device
///
/// This object provides information pertaining to the device through which the user is interacting.
/// Device information includes its hardware, platform, location, and carrier data. The device can
/// refer to a mobile handset, a desktop computer, set top box, or other digital device.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Device<'a> {
    /// string; recommended
    /// Browser user agent string.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ua: Option<std::borrow::Cow<'a, str>>,

    /// object; recommended
    /// Location of the device assumed to be the user’s current location defined by a Geo object
    /// (Section 3.2.19).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<crate::Geo<'a>>,

    /// integer; recommended
    /// Standard “Do Not Track” flag as set in the header by the browser, where 0 = tracking is
    /// unrestricted, 1 = do not track.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub dnt: Option<bool>,

    /// integer; recommended
    /// “Limit Ad Tracking” signal commercially endorsed (e.g., iOS, Android), where 0 = tracking
    /// is unrestricted, 1 = tracking must be limited per commercial guidelines.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub lmt: Option<bool>,

    /// string; recommended
    /// IPv4 address closest to device.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// IP address closest to device as IPv6.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// The general type of device. Refer to List 5.21.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<crate::DeviceType>,

    /// string
    /// Device make (e.g., “Apple”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub make: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Device model (e.g., “iPhone”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub model: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Device operating system (e.g., “iOS”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub os: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Device operating system version (e.g., “3.1.2”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub osv: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Hardware version of the device (e.g., “5S” for iPhone 5S).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub hwv: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Physical height of the screen in pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// integer
    /// Physical width of the screen in pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// integer
    /// Screen size as pixels per linear inch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,

    /// float
    /// The ratio of physical pixels to device independent pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<i32>,

    /// integer
    /// Support for JavaScript, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub js: Option<bool>,

    /// integer
    /// Indicates if the geolocation API will be available to JavaScript code running in the
    /// banner, where 0 = no, 1 = yes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::i32_as_opt_bool"
    )]
    pub geofetch: Option<bool>,

    /// string
    /// Version of Flash supported by the browser.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub flashver: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Browser language using ISO-639-1-alpha-2.
    // TODO: ISO-639-1-alpha-2
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub language: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Carrier or ISP (e.g., “VERIZON”) using exchange curated string names which should be
    /// published to bidders a priori.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub carrier: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Mobile carrier as the concatenated MCC-MNC code (e.g., “310-005” identifies Verizon
    /// Wireless CDMA in the USA). Refer to https://en.wikipedia.org/wiki/Mobile_country_code for further
    /// examples. Note that the dash between the MCC and MNC parts is required to remove parsing
    /// ambiguity.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Network connection type. Refer to List 5.22.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connectiontype: Option<crate::ConnectionType>,

    /// string
    /// ID sanctioned for advertiser use in the clear (i.e., not hashed).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ifa: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Hardware device ID (e.g., IMEI); hashed via SHA1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub didsha1: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Hardware device ID (e.g., IMEI); hashed via MD5.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub didmd5: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Platform device ID (e.g., Android ID); hashed via SHA1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub dpidsha1: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Platform device ID (e.g., Android ID); hashed via MD5.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub dpidmd5: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// MAC address of the device; hashed via SHA1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// MAC address of the device; hashed via MD5.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Device::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Device>(json)?);

        Ok(())
    }
}
