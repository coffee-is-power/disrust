use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
    str::FromStr,
};

pub trait Number {}
impl Number for u8 {}
impl Number for i8 {}
impl Number for u16 {}
impl Number for i16 {}
impl Number for u32 {}
impl Number for i32 {}
impl Number for u64 {}
impl Number for i64 {}
impl Number for u128 {}
impl Number for i128 {}
/// A snowflake is an ID of an object in discord.
/// Things like Guilds, Channels, Users, Members, all have IDs.
#[derive(Clone)]
pub struct Snowflake(String);
impl Snowflake {
    pub fn from(id: String) -> Self {
        assert!(is_ok(&id.parse::<u128>()));
        Self(id)
    }
    pub fn as_str(&self) -> String {
        self.0.clone()
    }
    pub fn take(self) -> String {
        self.0
    }
    pub fn as_number<T>(&self) -> T
    where
        T: Number + FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0.parse().unwrap()
    }
}
impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
impl Debug for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Snowflake").field(&self.0).finish()
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.parse::<u128>();
        if is_ok(&res) {
            Ok(Self(s.to_string()))
        } else {
            Err(res.unwrap_err())
        }
    }
}
fn is_ok<T, E>(r: &Result<T, E>) -> bool {
    if let Ok(_) = r {
        true
    } else {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::Snowflake;

    #[test]
    #[should_panic]
    fn test_invalid_snowflake() {
        Snowflake::from("This is a invalid Snowflake".to_owned());
    }
    #[test]
    fn test_convert_to_number() {
        let snowflake = Snowflake::from("123456789123456789123456789".to_string());
        assert_eq!(snowflake.as_number::<u128>(), 123456789123456789123456789)
    }
    #[test]
    fn display_outputs_number() {
        let snowflake = Snowflake::from("123456789123456789123456789".to_string());
        assert_eq!(format!("{}", snowflake), "123456789123456789123456789");
    }
    #[test]
    fn debug_shows_its_a_snowflake() {
        let snowflake = Snowflake::from("123456789123456789123456789".to_string());
        assert_eq!(
            format!("{:?}", snowflake),
            r#"Snowflake("123456789123456789123456789")"#
        );
    }
}
