<div align="center">

![suscription](/suscription.png)


<h1>Solana monthly subscription</h1>
<h4>A monthly subscription validated by the Solana blockchain</h4>
</div>

---

<div style="text-align: justify;">
  <p>
  This program is focused on the massive integration of monthly services to the Solana blockchain ecosystem. Being the creation of services and their respective administration, decentralized in each provider. The program consists of 5 basic functions that allow the autonomous performance of this program. Development can be integrated into desktop projects as well as mobile devices. The program will use credit of 8 per month, understanding that the service offered is 2 times per week, this can be modified and scaled to the needs of the developer.
  </p>
  <p>
  This project is a personal development and is not subject to any legislation of any country, so the development of applications influenced by it is the responsibility of the developer / company in question.
  </p>
  <p>
  Both registering a service and using one of the credits paid each month implies a use of the computing power of the Solana blockchain, so the project would scale up in those projects where security and transparency are vital, such as medical supplies, home rentals, retail specific, or multimedia playback of exclusive content, among many other options. 
  </p>

<h3 align="center">Register a service</h3>

  ```rust
pub fn create(
      ctx: Context<Create>,
      share_amount: u64,
      name: String
  ) -> Result<()> {
      let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
      let (_pda, bump) = Pubkey::find_program_address(&[b"Enterprise", ctx.accounts.user.key().as_ref()], ctx.program_id);
      enterprise_data.authority = ctx.accounts.user.key();
      enterprise_data.bump_original = bump;
      enterprise_data.name = name;
      enterprise_data.total_users = 0;
      enterprise_data.amount_per_month = share_amount;
      enterprise_data.secure_check = Clock::get().unwrap().unix_timestamp + 2332800;
      Ok(())
  }
#[derive(Accounts)]
pub struct Create<'info> {
  #[account(init, seeds = [b"Enterprise", user.key().as_ref()], bump, payer = user, space = 8 + EnterpriseData::LEN)]
  pub enterprise_data: Account<'info, EnterpriseData>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}
```

<h3 align="center">Suscribe to a service</h3>

```rust
pub fn suscribe(
    ctx: Context<Suscribe>,
    name: String,
    lastname: String
) -> Result<()> {
    if name.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    if lastname.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.enterprise_data.key().as_ref(), ctx.accounts.user.key().as_ref()], ctx.program_id);
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise_data.authority, ctx.accounts.enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    enterprise_data.total_users += 1;
    user_data.bump = bump;
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    user_data.credits = 8;
    Ok(())
}

#[derive(Accounts)]
pub struct Suscribe<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(init, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump, payer = from, space = 8 + SubscriberData::LEN)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Use your suscription</h3>

```rust
pub fn use_sus(
    ctx: Context<UseSus>
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    let mut secure_check: u8 = 0;
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp && user_data.credits > 0 { user_data.credits -= 1; secure_check += 1; }
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp { return Err(ErrorCode::OverdueCredits.into()); }
    if user_data.credits == 0 {
        enterprise_data.total_users -= 1;
        return Err(ErrorCode::YouHaveNoCredits.into());
    }
    if secure_check == 0 { user_data.credits -= 1; }
    Ok(())
}
#[derive(Accounts)]
pub struct UseSus<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Renew your suscription</h3>

```rust
pub fn renew(
    ctx: Context<Renew>
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &enterprise_data.authority, enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    enterprise_data.total_users += 1;
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    user_data.credits += 8;
    Ok(())
}
#[derive(Accounts)]
pub struct Renew<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Delete your suscription</h3>

```rust
pub fn delete(
    _ctx: Context<Delete>
) -> Result<()> {
    Ok(())
}
#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump, close = enterprise_data)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

</div>
