use super::component::*;
use crate::impl_stat;
use std::fmt::Debug;
use std::any::type_name;

pub trait Stat<T>
where
    T: Debug,
{
    fn stat(&self) -> T;
}
pub trait Description {
    fn get_description(&self) -> String;
}

impl_stat!(i32, Heal, Block, Damage, Speed);

// TODO: generic actually working
impl<T> Description for dyn Stat<T>
where
    T: Debug,
{
    fn get_description(&self) -> String {
        format!("{:?} {}", self.stat(), type_name::<T>())
    }
}

impl Description for Block {
    fn get_description(&self) -> String {
        format!("Grant {} Block", self.stat())
    }
}
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
        let b = Heal(3);
        let c = Damage(3);
        assert_eq!(a.stat(), 3);
        assert_eq!(b.stat(), 3);
        assert_eq!(c.stat(), 3);
        assert_eq!(c.get_description(), "Deal 3 Damage".to_string());
    }

    #[test]
    fn check_generic_desc_impl() {
        let va = Heal(10);
    }
}
