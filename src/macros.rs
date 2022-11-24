/// Implements the Stat<T> trait to basic game stat
/// T a component tuple struct, e.g Block(i32)
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

/// Implements the Description trait
#[macro_export]
macro_rules! impl_desc {
    ($a:ty, $s:expr) => {
        impl Description for $a {
            fn get_description(&self) -> String {
                format!($s, self.stat())
            }
        }
        impl OptionDescription for Option<&$a> {
            fn unwrap_description(&self) -> String {
                match self {
                    Some(value) => format!($s, value.stat()),
                    None => String::new()
                }
            }
        }
    };
}
