/// 3.2.7 [`Video#maxextended`], 3.2.8 [`Audio#maxextended`]
///
/// Maximum extended ad duration if extension is allowed. If blank or 0, extension is not allowed.
/// If -1, extension is allowed, and there is no time limit imposed. If greater than 0, then the
/// value represents the number of seconds of extended play supported beyond the maxduration value.
///
/// [`Video#maxextended`]: ./struct.Video.html#structfield.maxextended
/// [`Audio#maxextended`]: ./struct.Audio.html#structfield.maxextended
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MaxExtendedAdDuration {
    NoLimit,
    NotAllowed,
    Specific(i32),
}

impl Default for MaxExtendedAdDuration {
    fn default() -> Self {
        Self::NotAllowed
    }
}

impl serde::Serialize for MaxExtendedAdDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = match self {
            Self::NoLimit => -1,
            Self::NotAllowed => 0,
            Self::Specific(v) => *v,
        };
        serializer.serialize_i32(v)
    }
}

impl<'de> serde::Deserialize<'de> for MaxExtendedAdDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = match i32::deserialize(deserializer)? {
            -1 => Self::NoLimit,
            0 => Self::NotAllowed,
            v if v > 0 => Self::Specific(v),
            v => {
                let s = format!("invalid value: {}, expected -1 or 0 or greater than 0", v);
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
        assert!(serde_json::from_str::<MaxExtendedAdDuration>("-2").is_err());

        let json = "[-1,0,1]";
        let e1: Vec<MaxExtendedAdDuration> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                MaxExtendedAdDuration::NoLimit,
                MaxExtendedAdDuration::NotAllowed,
                MaxExtendedAdDuration::Specific(1),
            ]
        );

        Ok(())
    }
}
