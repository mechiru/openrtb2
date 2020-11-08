macro_rules! test_json {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() -> serde_json::Result<()> {
            let json = include_str!($path);
            let res = serde_json::from_str::<openrtb2::BidResponse>(json)?;
            assert_eq!(serde_json::to_string_pretty(&res)?, json);
            Ok(())
        }
    };
}

test_json!(
    ad_served_on_win_notice,
    "json/6.4.1_ad_served_on_win_notice.json"
);
test_json!(
    vast_xml_document_returned_inline,
    "json/6.4.2_vast_xml_document_returned_inline.json"
);
test_json!(
    direct_deal_ad_served_on_win_notice,
    "json/6.4.3_direct_deal_ad_served_on_win_notice.json"
);
test_json!(
    native_markup_returned_inline,
    "json/6.4.4_native_markup_returned_inline.json"
);
