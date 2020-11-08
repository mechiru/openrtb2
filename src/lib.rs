//! An implementation of [`OpenRTB 2.5 FINAL`].
//!
//! [`OpenRTB 2.5 FINAL`]: https://iabtechlab.com/wp-content/uploads/2016/07/OpenRTB-API-Specification-Version-2-5-FINAL.pdf

// ===== 3 bid request =====

// 3.2.1
mod bid_request;
pub use bid_request::*;

// 3.2.2
mod source;
pub use source::*;

// 3.2.3
mod regs;
pub use regs::*;

// 3.2.4
mod imp;
pub use imp::*;

// 3.2.5
mod metric;
pub use metric::*;

// 3.2.6
mod banner;
pub use banner::*;

// 3.2.7
mod video;
pub use video::*;

// 3.2.8
mod audio;
pub use audio::*;

// 3.2.9
mod native;
pub use native::*;

// 3.2.10
mod format;
pub use format::*;

// 3.2.11
mod pmp;
pub use pmp::*;

// 3.2.12
mod deal;
pub use deal::*;

// 3.2.13
mod site;
pub use site::*;

// 3.2.14
mod app;
pub use app::*;

// 3.2.15
mod publisher;
pub use publisher::*;

// 3.2.16
mod content;
pub use content::*;

// 3.2.17
mod producer;
pub use producer::*;

// 3.2.18
mod device;
pub use device::*;

// 3.2.19
mod geo;
pub use geo::*;

// 3.2.20
mod user;
pub use user::*;

// 3.2.21
mod data;
pub use data::*;

// 3.2.22
mod segment;
pub use segment::*;

// ===== 4 bid response =====

// 4.2.1
mod bid_response;
pub use bid_response::*;

// 4.2.2
mod seat_bid;
pub use seat_bid::*;

// 4.2.3
mod bid;
pub use bid::*;

// ===== 5 enum =====

// 5.1
mod content_category;
pub use content_category::*;

// 5.2
mod banner_ad_type;
pub use banner_ad_type::*;

// 5.3
mod creative_attribute;
pub use creative_attribute::*;

// 5.4
mod ad_position;
pub use ad_position::*;

// 5.5
mod expandable_direction;
pub use expandable_direction::*;

// 5.6
mod api_framework;
pub use api_framework::*;

// 5.7
mod video_linearity;
pub use video_linearity::*;

// 5.8
mod protocol;
pub use protocol::*;

// 5.9
mod video_placement_type;
pub use video_placement_type::*;

// 5.10
mod playback_method;
pub use playback_method::*;

// 5.11
mod playback_cessation_mode;
pub use playback_cessation_mode::*;

// 5.12
mod start_delay;
pub use start_delay::*;

// 5.13
mod production_quality;
pub use production_quality::*;

// 5.14
mod companion_type;
pub use companion_type::*;

// 5.15
mod content_delivery_method;
pub use content_delivery_method::*;

// 5.16
mod feed_type;
pub use feed_type::*;

// 5.17
mod volume_normalization_mode;
pub use volume_normalization_mode::*;

// 5.18
mod content_context;
pub use content_context::*;

// 5.19
mod iqg_media_rating;
pub use iqg_media_rating::*;

// 5.20
mod location_type;
pub use location_type::*;

// 5.21
mod device_type;
pub use device_type::*;

// 5.22
mod connection_type;
pub use connection_type::*;

// 5.23
mod ip_location_service;
pub use ip_location_service::*;

// 5.24
mod no_bid_reason;
pub use no_bid_reason::*;

// 5.25
mod loss_reason;
pub use loss_reason::*;

// ===== etc =====

mod distribution_channel;
pub use distribution_channel::*;

mod auction_type;
pub use auction_type::*;

mod gender;
pub use gender::*;

mod max_extended_ad_duration;
pub use max_extended_ad_duration::*;

// ===== internal =====

mod serde;
