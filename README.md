<div align="center">

![suscription](/suscription.png)


<h1>🌞Solana monthly subscription💵</h1>
<h4>A monthly subscription validated by the blockchain</h4>
</div>

---

<div>
  This program is focused on the massive integration of monthly services to the Solana blockchain ecosystem. Being the creation of services and their respective administration, decentralized in each provider.

  The program consists of 5 basic functions that allow the autonomous performance of this program. Development can be integrated into desktop projects as well as mobile devices. The program will use credit of 8 per month, understanding that the service offered is 2 times per week, this can be modified and scaled to the needs of the developer.

  This project is a personal development and is not subject to any legislation of any country, so the development of applications influenced by it is the responsibility of the developer / company in question.

  Both registering a service and using one of the credits paid each month implies a use of the computing power of the Solana blockchain, so the project would scale up in those projects where security and transparency are vital, such as medical supplies, home rentals, retail specific, or multimedia playback of exclusive content, among many other options.

<h3 align="center">Register a service🏪</h3>

```rust
pub fn create(
    ctx: Context<Create>,
    share_amount: u64,
    name: String
) -> Result<()> {
    // Get the enterprise_data account and PDA (program-derived address)
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Enterprise", ctx.accounts.user.key().as_ref()], ctx.program_id);
    // Set the authority, name, and other fields in the enterprise_data account
    enterprise_data.authority = ctx.accounts.user.key();
    enterprise_data.bump_original = bump;
    enterprise_data.name = name;
    enterprise_data.total_users = 0;
    enterprise_data.amount_per_month = share_amount;
    enterprise_data.secure_check = Clock::get().unwrap().unix_timestamp + 2332800;
    // Return Ok if successful
    Ok(())
}

// Define a struct for the create function's accounts
#[derive(Accounts)]
pub struct Create<'info> {
    // Initialize enterprise_data account with seeds, payer, and space
    #[account(init, seeds = [b"Enterprise", user.key().as_ref()], bump, payer = user, space = 8 + EnterpriseData::LEN)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut)]
    pub user: Signer<'info>, // mutable user account
    pub system_program: Program<'info, System>, // system program account
}
```
The create function takes as parameters a Context, a u64 value named share_amount, and a string named name. The return type of the function is Result<()>, which means that the function can return an Ok(()) value on success or an Err value on error.

Inside the function, you define a variable enterprise_data variable that is a reference to an Account<EnterpriseData> account. This account is initialized using the #[account(init)] macro and is stored in the variableenterprise_data.

The Pubkey::find_program_address function is also used to generate a unique public account address for the company account being created. The address is made up of a seed and a bump value that is generated from the string "Enterprise" and the public key of the user who is creating the account. This address is used as a seed to initialize the company account.

The company account information is updated with the values ​​provided in the function parameters. Finally, an Ok(()) value is returned if the operation was successful.

The function also uses the #[derive(Accounts)] macro to define a Create structure that represents the accounts needed to execute the function. This structure includes an enterprise_data account, a user account, and a system program account. These accounts are passed as arguments to the create function via the ctx parameter, which is of type Context<Create>.

<h3 align="center">Suscribe to a service</h3>

```rust
pub fn suscribe(
    ctx: Context<Suscribe>,
    name: String,
    lastname: String
) -> Result<()> {
    // Check if name or lastname are too long
    if name.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    if lastname.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    // Get mutable reference to user_data account
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    // Generate PDA and bump using the keys of the enterprise and user accounts
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.enterprise_data.key().as_ref(), ctx.accounts.user.key().as_ref()], ctx.program_id);
    // Transfer funds from 'from' account to the enterprise authority account
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise_data.authority, ctx.accounts.enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    // Get mutable reference to enterprise_data account
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    // Increment total_users count
    enterprise_data.total_users += 1;
    // Set user_data bump, month_timestamp, and credits
    user_data.bump = bump;
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    user_data.credits = 8;
    // Return Ok if everything is successful
    Ok(())
}

// Define accounts required for subscription
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

The function takes as input the user's information, including their first and last name, as well as the information of the company that provides the service. The function performs various operations, including validating the length of the first and last name, transferring a specified amount of money from the user's account to the company's account, updating company data, and creating a new user account.

En la función en sí, se realizan varias validaciones y operaciones de manejo de cuenta, incluyendo la validación de la longitud del nombre y apellido, la transferencia de una cantidad específica de dinero, la actualización de los datos de la empresa y la creación de una nueva cuenta de usuario. La función también utiliza el reloj de la blockchain para establecer la fecha de vencimiento de la suscripción y la cantidad de créditos disponibles para el usuario.

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

The function takes an argument of type Context<UseSus> and returns a Result<()>. The function is used to perform a transaction on the Solana blockchain that updates the data of a user subscribed to a company service.

The function begins by declaring two variables that point to the user and company accounts. Both are mutable so they can be updated on the blockchain. Next, a secure_check variable is declared which is used to keep track of whether the transaction is secure or not.

The function then performs some checks on the user's data to ensure that they have enough credits to use the service and that they are not late in paying. If the user has available credits, the function uses them and increments the value of secure_check. If the user has no credits available, the total number of subscribed users is reduced and an error is returned indicating that the user has no credits.

Finally, if the transaction is secure, a credit is used and Ok(()) is returned to indicate that the transaction was successful. The function also uses some attributes from the #[derive(Accounts)] macro to define the accounts that are used in the transaction and ensure that only accounts that correspond to the company and user in question can be updated.

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

The function is responsible for renewing a user's subscription to a business service.

The function takes a ctx argument of type Context<Renew> that contains information about the relevant user and business accounts, as well as other parameters needed to execute the function.

The function performs several operations. First, it gets a mutable reference to the user and business data accounts. It then invokes a transfer function from Solana's system program to transfer a specified number of tokens from the user's account to the company's authorized party. It then updates the business data to reflect that an additional user has been added. Lastly, it updates the user's data to reflect that their subscription has been renewed for another month, adding additional credits to their account.

The function is annotated with an Accounts attribute that specifies the accounts required for the execution of the function. In this case, the role requires a business data account, a user data account, a token transfer user account, a gambling account, and the ability to use the Solana system program.

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

The delete function is a delete function that deletes a subscriber's data in an enterprise data account.

The Delete structure uses the Accounts attribute to specify which accounts should be used in the function. The enterprise_data account is an enterprise account that stores company data, while user_data is an account that stores subscriber data. Both accounts are mutable and require a set of seeds for modification.

The user account is a signer that is used to authorize the delete operation. In addition, the function also uses the Solana system program to perform the operation.

</div>
