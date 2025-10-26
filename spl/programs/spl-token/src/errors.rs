use anchor_lang::error_code;

#[error_code]
pub enum ErrorType {
    #[msg("Unauthorized Access")]
    Unauthorized,

    #[msg("Insufficient Tokens to transfer")]
    InsufficientTokens,

    #[msg("Invalid Mint")]
    InvalidMint,

    #[msg("Can't send money to frozen account")]
    FrozenAccount,

    #[msg("Can't freeze already frozen account")]
    AlreadyFrozen,

    #[msg("Account is already in unfreezed state")]
    AlreadyUnFrozen,

    #[msg("Token Supply Overflowed")]
    TokenSupplyOverflowed,

    #[msg("Value Overflowed")]
    ValueOverflowed
}