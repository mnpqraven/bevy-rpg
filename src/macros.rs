/// for now struct needs to be a valid tuple struct Struct(i32)
/// Implements the Stat<T> trait to basic game stat, T is the data type,
/// usually i32
#[macro_export]
macro_rules! impl_stat {
    ($t:ty, $($s:ty),+) => {
        $(
            impl Stat<$t> for $s {
                fn stat(&self) -> $t {
                    // Implementation code here
                    self.0
                }
            }
        )+
    };
}
