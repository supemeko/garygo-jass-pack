pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

// #[derive(Debug, From)]
// pub enum Error {
//     LexError {
//         raw_number: u32,
//         line_number: u32
//     }

// }
