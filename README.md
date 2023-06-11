<div align="center">

![suscription](/suscription.png)


<h1>🌞Solana monthly subscription💵</h1>
<h4>A monthly subscription validated by the blockchain</h4>
</div>

---

This program focuses on the massive integration of monthly services to the Solana blockchain ecosystem. The goal is to create services and allow for their respective administration to be decentralized across each provider. The program consists of 5 basic functions that enable autonomous performance, and it can be integrated into desktop and mobile projects. The program will use 8 credits per month, with the service being offered twice a week. However, this can be modified and scaled to meet the needs of the developer.

Please note that this project is for personal development and is not subject to any legislation in any country. Therefore, the responsibility of developing applications influenced by it falls on the developer or company in question.

Registering a service and using one of the credits paid each month implies the use of the computing power of the Solana blockchain. This makes it a great option for projects where security and transparency are vital, such as medical supplies, home rentals, retail specific, or multimedia playback of exclusive content, among many other options.

---

<details>
<summary>Register a service🏙️</summary>

<br>

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
The create function takes three parameters: a Context, a u64 value named `share_amount`, and a string named name. The return type of the function is `Result<()>`, which means that the function can return an `Ok(())` value on success or an Err value on error.

Inside the function, you define a variable `enterprise_data` that is a reference to an Account<EnterpriseData> account. This account is initialized using the `#[account(init)]` macro and is stored in the variable enterprise_data.

The `find_program_address()` function is also used to generate a unique public account address for the company account being created. The address is made up of a seed and a bump value that is generated from the string "Enterprise" and the public key of the user who is creating the account. This address is used as a seed to initialize the company account.

The company account information is updated with the values provided in the function parameters. Finally, an Ok(()) value is returned if the operation was successful.

The function also uses the `#[derive(Accounts)]` macro to define a Create structure that represents the accounts needed to execute the function. This structure includes an enterprise_data account, a user account, and a system program account. These accounts are passed as arguments to the create function via the `ctx` parameter, which is of type Context<Create>.

</details>

---

<details>
<summary>Suscribe to a service ✒️ </summary>

<br>

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

The function takes as input the user's information, including their first and last name, as well as the information of the company that provides the service. The function performs several operations, including validating the length of the first and last name, transferring a specified amount of money from the user's account to the company's account, updating company data, and creating a new user account.

In the function itself, various validations and account management operations are performed, including validating the length of the first and last name, transferring an amount of money, updating company data, and creating a new user account.

The feature also uses the blockchain clock to set the subscription expiration date and the number of `credits` available to the user.

</details>

---

<details>
<summary>Use your suscription 🏋️ </summary>

<br>

```rust
pub fn use_sus(
    ctx: Context<UseSus> // The function takes a `Context` object as a parameter
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data; // Get a mutable reference to the enterprise data account
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data; // Get a mutable reference to the user data account
    let mut secure_check: u8 = 0; // Initialize a variable to keep track of whether a secure check has been performed
    // If the user's credits are greater than zero and their subscription has not expired, decrement their credits and perform a secure check
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp && user_data.credits > 0 {
        user_data.credits -= 1;
        secure_check += 1;
    }
    // If the user's subscription has expired, return an error
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp {
        return Err(ErrorCode::OverdueCredits.into());
    }
    // If the user has no credits left, decrement the total number of users and return an error
    if user_data.credits == 0 {
        enterprise_data.total_users -= 1;
        return Err(ErrorCode::YouHaveNoCredits.into());
    }
    // If a secure check has not been performed, decrement the user's credits and return Ok(())
    if secure_check == 0 {
        user_data.credits -= 1;
    }
    Ok(()) // Return Ok(()) if the function completes successfully
}

// Define a `UseSus` struct that represents the accounts needed to execute the `use_sus` function
#[derive(Accounts)]
pub struct UseSus<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>, // The enterprise data account, which is mutable and requires a seed
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>, // The user data account, which is mutable and requires two seeds
    pub user: Signer<'info>, // The user's signature, which is required to execute the function
    pub system_program: Program<'info, System>, // The system program account, which is required to execute the function
}
```

The function takes an argument of type `Context<UseSus>` and returns a Result<()>. The function is used to perform a transaction on the Solana blockchain that updates the data of a user subscribed to a company service.

The function begins by declaring two variables that point to the user and company accounts. Both are mutable so they can be updated on the blockchain. Next, a `secure_check` variable is declared which is used to keep track of whether the transaction is secure or not.

The function then performs some checks on the user's data to ensure that they have enough credits to use the service and that they are not late in paying. If the user has available credits, the function uses them and increments the value of `secure_check`. If the user has no credits available, the total number of subscribed users is reduced and an error is returned indicating that the user has no credits.

Finally, if the transaction is secure, a credit is used and Ok(()) is returned to indicate that the transaction was successful. The function also uses some attributes from the #[derive(Accounts)] macro to define the accounts that are used in the transaction and ensure that only accounts that correspond to the company and user in question can be updated.

</details>

---

<details>
<summary>Renew your suscription 🗒️ </summary>

<br>

```rust
pub fn renew(
    ctx: Context<Renew>
) -> Result<()> {
    // Declare mutable references to the enterprise and user accounts
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    // Transfer funds from the user's account to the enterprise's account
    // This is done using the Solana System Program's transfer instruction
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &enterprise_data.authority, enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    // Increment the total number of subscribed users for the enterprise
    enterprise_data.total_users += 1;
    // Set the user's renewal timestamp to one month from the current timestamp
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    // Add 8 credits to the user's account
    user_data.credits += 8;
    // Return Ok(()) to indicate that the transaction was successful
    Ok(())
}

// Define the accounts required for the Renew function using the #[derive(Accounts)] macro
#[derive(Accounts)]
pub struct Renew<'info> {
    // The enterprise account that is being subscribed to
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    // The user account that is subscribing
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    // The account from which funds are transferred
    #[account(mut)]
    pub from: AccountInfo<'info>,
    // The account where stake is added to
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    // The signer for the user account
    pub user: Signer<'info>,
    // The Solana System Program
    pub system_program: Program<'info, System>,
}
```

The function is responsible for renewing a `user's subscription` to a business service.

The function takes a ctx argument of type Context<Renew> that contains information about the relevant user and business accounts, as well as other parameters needed to execute the function.

The function performs several operations. First, it gets a mutable reference to the user and business data accounts. It then invokes a transfer function from Solana's system program to transfer a specified number of tokens from the user's account to the company's authorized party. It then updates the business data to reflect that an additional user has been added. Lastly, it updates the user's data to reflect that their subscription has been renewed for another month, adding additional credits to their account.

The function is annotated with an Accounts attribute that specifies the accounts required for the execution of the function. In this case, the role requires a business data account, a user data account, a token transfer user account, a gambling account, and the ability to use the Solana system program.

</details>

---

<details>
<summary>Delete your suscription 🗑️</summary>

<br>

```rust
pub fn delete(
    _ctx: Context<Delete> // The function takes a context object of type Delete
) -> Result<()> {
    Ok(()) // The function simply returns a successful result
}

// Define a struct to represent the accounts needed for the delete operation
#[derive(Accounts)]
pub struct Delete<'info> {
    // The enterprise_data account must be mutable and is derived from a seed
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    // The user_data account must be mutable, is derived from two seeds, and will be closed upon deletion
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump, close = enterprise_data)]
    pub user_data: Account<'info, SubscriberData>,
    // The user account must be a signer (i.e. it must be authorized to perform the delete operation)
    pub user: Signer<'info>,
    // The system_program account is a standard Solana program account used for system-level operations
    pub system_program: Program<'info, System>,
}
```

The delete function is a delete function that deletes a subscriber's data in an enterprise data account.

The Delete structure uses the Accounts attribute to specify which accounts should be used in the function. The `enterprise_data` account is an enterprise account that stores company data, while `user_data` is an account that stores subscriber data. Both accounts are mutable and require a set of seeds for modification.

The user account is a signer that is used to authorize the delete operation. In addition, the function also uses the Solana system program to perform the operation.

</details>

---
