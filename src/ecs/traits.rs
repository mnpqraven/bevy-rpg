//! IMPL RULES
//! Description, OptionDescription
//! any component that can be shown on the skill context menu
//!
//! Stat
//! Any tuple component that represents a basic skill stat
use super::component::*;
use crate::impl_desc;
use crate::impl_stat;
use std::fmt::Debug;

pub trait Stat<T: Debug> {
    fn stat(&self) -> T;
}
pub trait Description {
    fn get_description(&self) -> String;
}
pub trait OptionDescription {
    fn unwrap_description(&self) -> String;
}

impl_stat!(i32, Health, Mana, MaxHealth, MaxMana);
impl_stat!(i32, Heal, Block, Damage);
impl_stat!(u32, Channel);

impl_desc!(Heal, "Heal the target for {}");
impl_desc!(Block, "Grant {} Block");
impl_desc!(Damage, "Deal {} Damage");
impl Description for Channel {
    fn get_description(&self) -> String {
        let s = match self.stat() {
            1 => "turn",
            _ => "turns",
        };
        format!("Channel for {} {}", self.stat(), s)
    }
}
impl OptionDescription for Option<&Channel> {
    fn unwrap_description(&self) -> String {
        if let Some(value) = self {
            let s = match value.0 {
                1 => "turn",
                _ => "turns",
            };
            format!("Channel for {} {}", value.stat(), s)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_stat_impl_macro() {
        let a = Block(3);
        assert_eq!(a.stat(), 3);
        assert_eq!(a.get_description(), "Grant 3 Block".to_string());
    }
    #[test]
    fn check_desc_impl_macro() {
        let a = Heal(2);
        assert_eq!(a.get_description(), "Heal the target for 2".to_string())
    }
    #[test]
    fn check_none_unwrap() {
        let a: Option<&Damage> = None;
        assert_eq!(a.unwrap_description(), String::new())
    }
    #[test]
    fn check_some_unwrap() {
        let a: Option<&Damage> = Some(&Damage(83));
        assert_eq!(a.unwrap_description(), "Deal 83 Damage".to_string())
    }
}
