use crate::utils::util::{ANCHOR_BUFFER, MONTH};
use anchor_lang::prelude::*;

#[account]
pub struct EnterpriseData {
    pub authority: Pubkey,     //32
    pub name: String,          //4 + 30
    pub bump_original: u8,     //1
    pub amount_per_month: u64, //8
    pub secure_check: i64,     //8
    pub total_users: u16,      //2
}

#[account]
pub struct SubscriberData {
    pub authority: Pubkey,    //32
    pub name: String,         //4 + 20
    pub lastname: String,     //4 + 20
    pub month_timestamp: i64, //8
    pub bump: u8,             //1
    pub credits: u8,          //1
}

impl EnterpriseData {
    pub const LEN: usize = 32 + 4 + 30 + 1 + 8 + 8 + 2 + ANCHOR_BUFFER;

    pub fn add_total_users(&mut self) {
        self.total_users += 1;
    }

    pub fn set_total_users(&mut self) {
        self.total_users = 0;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_amount_per_month(&mut self, amount: u64) {
        self.amount_per_month = amount;
    }

    pub fn set_secure_check(&mut self) {
        let current_time: i64 = Clock::get().unwrap().unix_timestamp;
        self.secure_check = current_time + MONTH;
    }

    pub fn have_credits(&mut self, credits: u8) -> Result<()> {
        let active: bool = credits > 0;
        if !active {
            self.total_users -= 1;
        }
        require_gt!(credits, 0);
        Ok(())
    }
}

impl SubscriberData {
    pub const LEN: usize = 32 + 4 + 20 + 4 + 20 + 8 + 1 + 1 + ANCHOR_BUFFER;

    pub fn add_month_timestamp(&mut self) {
        let current_time: i64 = Clock::get().unwrap().unix_timestamp;
        self.month_timestamp = current_time + MONTH;
    }

    pub fn add_credits(&mut self) {
        self.credits += 8;
    }

    pub fn sub_credits(&mut self) {
        self.credits -= 1;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn valid_time(&self) -> Result<()> {
        let current_time: i64 = Clock::get().unwrap().unix_timestamp;
        require_gte!(self.month_timestamp, current_time);
        Ok(())
    }
}
