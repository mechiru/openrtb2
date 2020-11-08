/// 3.2.1 [`BidRequest#at`], 3.2.12 [`Deal#at`]
///
/// Auction type, where 1 = First Price, 2 = Second Price Plus. Exchange-specific auction types can
/// be defined using values greater than 500.
///
/// [`BidRequest#at`]: ./struct.BidRequest.html#structfield.at
/// [`Deal#at`]: ./struct.Deal.html#structfield.at
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AuctionType {
    FirstPrice,
    SecondPricePlus,
    ExchangeSpecific(i32),
}

impl Default for AuctionType {
    fn default() -> Self {
        Self::SecondPricePlus
    }
}

impl serde::Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = match self {
            Self::FirstPrice => 1,
            Self::SecondPricePlus => 2,
            Self::ExchangeSpecific(v) => *v,
        };
        serializer.serialize_i32(v)
    }
}

impl<'de> serde::Deserialize<'de> for AuctionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match i32::deserialize(deserializer)? {
            1 => Self::FirstPrice,
            2 => Self::SecondPricePlus,
            v if v > 500 => Self::ExchangeSpecific(v),
            v => {
                let s = format!("invalid value: {}, expected 1 or 2 or greater than 500", v);
                return Err(serde::de::Error::custom(s));
            }
        };
        Ok(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<AuctionType>("3").is_err());
        assert!(serde_json::from_str::<AuctionType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<AuctionType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                AuctionType::FirstPrice,
                AuctionType::SecondPricePlus,
                AuctionType::ExchangeSpecific(501)
            ]
        );

        Ok(())
    }
}
