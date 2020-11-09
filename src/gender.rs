/// 3.2.20 [User#gender](./struct.User.html#structfield.gender)
///
/// Gender, where “M” = male, “F” = female, “O” = known to be other.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    #[serde(rename = "M")]
    Male,
    #[serde(rename = "F")]
    Female,
    #[serde(rename = "O")]
    Other,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Gender>("X").is_err());

        let json = r#"["M","F","O"]"#;
        let o1: Vec<Gender> = serde_json::from_str(json)?;
        assert_eq!(o1, vec![Gender::Male, Gender::Female, Gender::Other]);
        assert_eq!(serde_json::to_string(&o1)?, json);
        Ok(())
    }
}
