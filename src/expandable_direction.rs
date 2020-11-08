use serde_repr::{Deserialize_repr, Serialize_repr};

/// 5.5 Expandable Direction
///
/// The following table lists the directions in which an expandable ad may expand, given the
/// positioning of the ad unit on the page and constraints imposed by the content.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum ExpandableDirection {
    /// Left
    Left = 1,
    /// Right
    Right,
    /// Up
    Up,
    /// Down
    Down,
    /// Full Screen
    FullScreen,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ExpandableDirection>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<ExpandableDirection> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![ExpandableDirection::Left, ExpandableDirection::Right]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
