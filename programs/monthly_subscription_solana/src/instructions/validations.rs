use crate::errors::errors::ErrorCode;
use anchor_lang::prelude::*;

pub fn check_size(a: String, size: usize) -> Result<()> {
    require!(a.chars().count() < size, ErrorCode::TooLong);
    Ok(())
}
