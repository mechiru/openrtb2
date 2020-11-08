use serde::{Deserialize, Serialize};

/// 5.12 Start Delay
///
/// The following table lists the various options for the video or audio start delay. If the start
/// delay value is greater than 0, then the position is mid-roll and the value indicates the start
/// delay.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StartDelay {
    /// Mid-Roll (value indicates start delay in second)
    MidRoll(i32),
    /// Pre-Roll
    PreRoll,
    /// Generic Mid-Roll
    GenericMidRoll,
    /// Generic Post-Roll
    GenericPostRoll,
}

impl Serialize for StartDelay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = match self {
            StartDelay::MidRoll(v) => *v,
            StartDelay::PreRoll => 0,
            StartDelay::GenericMidRoll => -1,
            StartDelay::GenericPostRoll => -2,
        };
        serializer.serialize_i32(v)
    }
}

impl<'de> Deserialize<'de> for StartDelay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match i32::deserialize(deserializer)? {
            v if v > 0 => StartDelay::MidRoll(v),
            0 => StartDelay::PreRoll,
            -1 => StartDelay::GenericMidRoll,
            -2 => StartDelay::GenericPostRoll,
            v => {
                let s = format!(
                    "invalid value: {}, expected 0 or -1 or -2 or greater than 0",
                    v
                );
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
        assert!(serde_json::from_str::<StartDelay>("-3").is_err());

        let json = "[10,0,-1,-2]";
        let delays: Vec<StartDelay> = serde_json::from_str(json)?;
        assert_eq!(
            delays,
            vec![
                StartDelay::MidRoll(10),
                StartDelay::PreRoll,
                StartDelay::GenericMidRoll,
                StartDelay::GenericPostRoll
            ]
        );
        assert_eq!(serde_json::to_string(&delays)?, json);

        Ok(())
    }
}
