#![feature(test)]

extern crate test;

use test::Bencher;

use openrtb2::BidRequest;

const BID_REQUEST: &str = include_str!("../tests/json/6.3.4_video.json");

#[bench]
fn bench_deserialize(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<BidRequest>(BID_REQUEST).unwrap());
}
