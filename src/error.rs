use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum HammingError {
    #[error("unable to decode")]
    Decode,
}
