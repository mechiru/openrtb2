macro_rules! test_json {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() -> serde_json::Result<()> {
            let json = include_str!($path);
            let req = serde_json::from_str::<openrtb2::BidRequest>(json)?;
            assert_eq!(serde_json::to_string_pretty(&req)?, json);
            Ok(())
        }
    };
}

test_json!(simple_banner, "json/6.3.1_simple_banner.json");
test_json!(expandable_creative, "json/6.3.2_expandable_creative.json");
test_json!(mobile, "json/6.3.3_mobile.json");
test_json!(video, "json/6.3.4_video.json");
test_json!(pmp_with_direct_deal, "json/6.3.5_pmp_with_direct_deal.json");
test_json!(native_ad, "json/6.3.6_native_ad.json");
