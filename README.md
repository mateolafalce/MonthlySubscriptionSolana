<div align="center">

![suscription](/suscription.png)


# Solana monthly subscription

### A monthly subscription validated by the blockchain

</div>

---

This program focuses on the massive integration of monthly services to the Solana blockchain ecosystem. The goal is to create services and allow for their respective administration to be decentralized across each provider. The program consists of 5 basic functions that enable autonomous performance, and it can be integrated into desktop and mobile projects. The program will use 8 credits per month, with the service being offered twice a week. However, this can be modified and scaled to meet the needs of the developer.

Please note that this project is for personal development and is not subject to any legislation in any country. Therefore, the responsibility of developing applications influenced by it falls on the developer or company in question.

Registering a service and using one of the credits paid each month implies the use of the computing power of the Solana blockchain. This makes it a great option for projects where security and transparency are vital, such as medical supplies, home rentals, retail specific, or multimedia playback of exclusive content, among many other options.

---

<h2>Register a service</h2>

<br>

```rust
pub fn create(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let (_pda, bump) = Pubkey::find_program_address(
        &[b"Enterprise", ctx.accounts.user.key().as_ref()],
        ctx.program_id,
    );
    //update state
    enterprise_data.set_authority(ctx.accounts.user.key());
    enterprise_data.set_bump(bump);
    enterprise_data.set_name(name);
    enterprise_data.set_total_users();
    enterprise_data.set_amount_per_month(share_amount);
    enterprise_data.set_secure_check();
    Ok(())
}
```
The create function takes three parameters: a Context, a u64 value named `share_amount`, and a string named name. The return type of the function is `Result<()>`, which means that the function can return an `Ok(())` value on success or an Err value on error.

Inside the function, you define a variable `enterprise_data` that is a reference to an Account<EnterpriseData> account. This account is initialized using the `#[account(init)]` macro and is stored in the variable enterprise_data.

The company account information is updated with the values provided in the function parameters. Finally, an Ok(()) value is returned if the operation was successful.


---

<h2>Suscribe to a service</h2>

<br>

```rust
pub fn suscribe(ctx: Context<Suscribe>, name: String, lastname: String) -> Result<()> {
    //validations
    check_size(name, 20).unwrap();
    check_size(lastname, 20).unwrap();

    // get bump from pda
    let (_pda, bump) = Pubkey::find_program_address(
        &[
            ctx.accounts.enterprise_data.key().as_ref(),
            ctx.accounts.user.key().as_ref(),
        ],
        ctx.program_id,
    );

    //transfer from -> to (lamport)
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.enterprise_data.authority,
            ctx.accounts.enterprise_data.amount_per_month,
        ),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.stake.to_account_info().clone(),
        ],
    )
    .expect("Error");

    // get &mut accounts
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;

    //update state
    enterprise_data.add_total_users();
    user_data.set_bump(bump);
    user_data.add_month_timestamp();
    user_data.add_credits();

    Ok(())
}
```

The function takes as input the user's information, including their first and last name, as well as the information of the company that provides the service. The function performs several operations, including validating the length of the first and last name, transferring a specified amount of money from the user's account to the company's account, updating company data, and creating a new user account.

In the function itself, various validations and account management operations are performed, including validating the length of the first and last name, transferring an amount of money, updating company data, and creating a new user account.

The feature also uses the blockchain clock to set the subscription expiration date and the number of `credits` available to the user.

---

<h2>Use your suscription  </h2>

<br>

```rust
pub fn use_sus(
    ctx: Context<UseSus>,
) -> Result<()> {
    //get &mut accounts
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data; 
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data; 

    //validations
    enterprise_data.have_credits(user_data.credits).unwrap();
    user_data.valid_time().unwrap();
                       
    //update state           
    user_data.sub_credits();

    Ok(()) 
}
```

Begins by declaring two variables that point to the user and company accounts. Both are mutable so they can be updated on the blockchain. Next, a `secure_check` variable is declared which is used to keep track of whether the transaction is secure or not.

Performs some checks on the user's data to ensure that they have enough credits to use the service and that they are not late in paying. If the user has available credits, the function uses them and increments the value of `secure_check`. If the user has no credits available, the total number of subscribed users is reduced and an error is returned indicating that the user has no credits.

Finally, if the transaction is secure, a credit is used and Ok(()) is returned to indicate that the transaction was successful. The function also uses some attributes from the #[derive(Accounts)] macro to define the accounts that are used in the transaction and ensure that only accounts that correspond to the company and user in question can be updated.

---

<h2>Renew your suscription</h2>

<br>

```rust
pub fn renew(ctx: Context<Renew>) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;

    //transfer from -> to (amount)
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            &ctx.accounts.from.key(),
            &enterprise_data.authority,
            enterprise_data.amount_per_month,
        ),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.stake.to_account_info().clone(),
        ],
    )
    .expect("Error");

    //update state
    enterprise_data.add_total_users();
    user_data.add_month_timestamp();
    user_data.add_credits();

    Ok(())
}
```


The function takes a ctx argument of type Context<Renew> that contains information about the relevant user and business accounts, as well as other parameters needed to execute the function.

Performs several operations. First, it gets a mutable reference to the user and business data accounts. It then invokes a transfer function from Solana's system program to transfer a specified number of tokens from the user's account to the company's authorized party. It then updates the business data to reflect that an additional user has been added. Lastly, it updates the user's data to reflect that their subscription has been renewed for another month, adding additional credits to their account.

Annotated with an Accounts attribute that specifies the accounts required for the execution of the function. In this case, the role requires a business data account, a user data account, a token transfer user account, a gambling account, and the ability to use the Solana system program.

---

<h2>Delete your suscription</h2>

<br>

```rust
pub fn delete(
    _ctx: Context<Delete>, 
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

The Delete structure uses the Accounts attribute to specify which accounts should be used in the function. The `enterprise_data` account is an enterprise account that stores company data, while `user_data` is an account that stores subscriber data. Both accounts are mutable and require a set of seeds for modification.

The user account is a signer that is used to authorize the delete operation. In addition, the function also uses the Solana system program to perform the operation.

</details>

---
