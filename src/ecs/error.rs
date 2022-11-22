use std::fmt::Display;

#[derive(Debug)]
pub enum DataError {
    EmptyList,
}
impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DataError::EmptyList => write!(
                f,
                "empty List, check if the generate function was ran before"
            ),
        }
    }
}
