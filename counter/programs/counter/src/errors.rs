use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorType {
    #[msg("User trying to modify counter is not authorized")]
    UnAuthorized,
    #[msg("Error while incrementing counter")]
    IncrementError,
    #[msg("Error while decrementing counter")]
    DecrementError
}