use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic_2 {
    use super::*;

    pub fn create(ctx: Context<Create>, authority: Pubkey) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    //mut: tells the program to persist all changes to the account
    //has_one: enforces the constraint that Increment.counter.authority == Increment.authority.key
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    //signer: enforces the constraint that the authority account signed the transaction
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

/*Accounts: derive macro implementing the Accounts trait ,
allowing a struct to transform from the untrusted &[AccountInfo] slice given to a Solana program into a validated struct of deserialized account types. */

/*#[account]: attribute macro implementing AccountSerialize and AccountDeserialize, automatically prepending a unique 8 byte discriminator to the account array. 
The discriminator is defined by the first 8 bytes of the Sha256 hash of the account's Rust identifier--i.e., the struct type name--and ensures no account can be substituted 
for another. */
/*Account : a wrapper type for a deserialized account implementing AccountDeserialize. 
Using this type within an Accounts struct will ensure the account is owned by the address defined by declare_id! where the inner account was defined. */



