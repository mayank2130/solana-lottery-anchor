use anchor_lang::prelude::error_code;

#[error_code]
pub enum LotteryError {
    #[msg("You're already won!")]
    WinnerAlreadyExists,
    #[msg("First, buy a ticket to pick a winner!")]
    NoTickets,
    #[msg("Pick a winner. To claim the prize!")]
    WinnerNotChosen,
    #[msg("Invalid Winner!")]
    InvalidWinner,
    #[msg("The prize has already been claimed!")]
    AlreadyClaimed,
}
