use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.25 Loss Reason Codes
///
/// The following table lists the options for an exchange to inform a bidder as to the reason why
/// they did not win an impression.
#[allow(non_camel_case_types)]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum LossReason {
    /// Bid Won
    BidWon = 0,
    /// Internal Error
    InternalError,
    /// Impression Opportunity Expired
    ImpressionOpportunityExpired,
    /// Invalid Bid Response
    InvalidBidResponse,
    /// Invalid Deal ID
    InvalidDealId,
    /// Invalid Auction ID
    InvalidAuctionId,
    /// Invalid (i.e., malformed) Advertiser Domain
    InvalidAdvertiserDomain,
    /// Missing Markup
    MissingMarkup,
    /// Missing Creative ID
    MissingCreativeId,
    /// Missing Bid Price
    MissingBidPrice,
    /// Missing Minimum Creative Approval Data
    MissingMinCreativeApprovalData,
    /// Bid was Below Auction Floor
    BidBelowAuctionFloor = 100,
    /// Bid was Below Deal Floor
    BidBelowDealFloor,
    /// Lost to Higher Bid
    LostHigherBid,
    /// Lost to a Bid for a PMP Deal
    LostPmpDeal,
    /// Buyer Seat Blocked
    BuyerSeatBlocked,
    /// Creative Filtered - General; reason unknown.
    CreativeFiltered_General = 200,
    /// Creative Filtered - Pending processing by Exchange (e.g., approval, transcoding, etc.)
    CreativeFiltered_Pending,
    /// Creative Filtered - Disapproved by Exchange
    CreativeFiltered_Disapproved,
    /// Creative Filtered - Size Not Allowed
    CreativeFiltered_SizeNotAllowed,
    /// Creative Filtered - Incorrect Creative Format
    CreativeFiltered_IncorrectFormat,
    /// Creative Filtered - Advertiser Exclusions
    CreativeFiltered_AdvertiserExclusions,
    /// Creative Filtered – App Bundle Exclusions
    CreativeFiltered_AppBundleExclusions,
    /// Creative Filtered - Not Secure
    CreativeFiltered_NotSecure,
    /// Creative Filtered - Language Exclusions
    CreativeFiltered_LanguageExclusions,
    /// Creative Filtered - Category Exclusions
    CreativeFiltered_CategoryExclusions,
    /// Creative Filtered - Creative Attribute Exclusions
    CreativeFiltered_AttributeExclusions,
    /// Creative Filtered - Ad Type Exclusions
    CreativeFiltered_AdTypeExclusions,
    /// Creative Filtered - Animation Too Long
    CreativeFiltered_AnimationTooLong,
    /// Creative Filtered - Not Allowed in PMP Deal
    CreativeFiltered_NotAllowedPmpDeal,
    // TODO: ≥ 1000 Exchange specific (should be communicated to bidders a priori)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<LossReason>("-1").is_err());

        let json = "[0,1,100,101,200,201]";
        let e1: Vec<LossReason> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![
                LossReason::BidWon,
                LossReason::InternalError,
                LossReason::BidBelowAuctionFloor,
                LossReason::BidBelowDealFloor,
                LossReason::CreativeFiltered_General,
                LossReason::CreativeFiltered_Pending,
            ]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }

    #[test]
    fn repr() {
        assert_eq!(LossReason::BidWon as i32, 0);
        assert_eq!(LossReason::InternalError as i32, 1);
        assert_eq!(LossReason::BidBelowAuctionFloor as i32, 100);
        assert_eq!(LossReason::BidBelowDealFloor as i32, 101);
        assert_eq!(LossReason::CreativeFiltered_General as i32, 200);
        assert_eq!(LossReason::CreativeFiltered_Pending as i32, 201);
    }
}
