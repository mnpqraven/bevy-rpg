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

// macro + normal impl
impl_stat!(i32, Heal, Block);
impl Stat<i32> for Damage {
    fn stat(&self) -> i32 {
        self.0
    }
}

// macro + normal impl
impl_desc!(Heal, "Heal the target for {}");
impl_desc!(Block, "Grant {} Block");
impl Description for Damage {
    fn get_description(&self) -> String {
        format!("Deal {} Damage", self.stat())
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
}
