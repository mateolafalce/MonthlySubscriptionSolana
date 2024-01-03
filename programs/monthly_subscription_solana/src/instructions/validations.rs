use anchor_lang::prelude::*;

pub fn check_size(string: String, size: usize) -> Result<()> {
    require_gt!(size, string.chars().count());
    Ok(())
}
