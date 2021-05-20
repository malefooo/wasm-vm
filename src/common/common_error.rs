use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommonError{
    Leb128Error(String),
}

impl Display for CommonError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::Leb128Error(str) => {f.write_str(str)}
        }
    }
}

// impl std::error::Error for CommonError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)>{
//         match self {
//             CommonError::Leb128Error(str) => {Some()}
//         }
//     }
// }
//
