use anchor_lang::prelude::*;

#[account]
pub struct EnterpriseData {
    pub authority: Pubkey, 
    pub name: String, //30
    pub bump_original: u8, 
    pub amount_per_month: u64,
    pub secure_check: i64,
    pub total_users: u16
}
#[account]
pub struct SubscriberData {
    pub authority: Pubkey,
    pub name: String, // 20
    pub lastname: String, // 20
    pub month_timestamp: i64,
    pub bump: u8,
    pub credits: u8
}
impl EnterpriseData {
    pub const LEN: usize = 32 + 4 + 30 + 1 + 8 + 8 + 2;
}
impl SubscriberData {
    pub const LEN: usize = 32 + 4 + 20 + 4 + 20 + 8 + 1 + 1;
}