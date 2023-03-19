use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Overdue credits")]OverdueCredits, 
    #[msg("You have no credits")]YouHaveNoCredits, 
    #[msg("Recently changed less than 22 days ago")]RecentlyChanged, 
    #[msg("Only 20 characters")]TooLong, 
}