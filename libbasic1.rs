use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic_1 {
    use super::*;

    /* data argument passed into the program. This argument and any other valid Rust types
     can be passed to the instruction to define inputs to the program.*/

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account; //mutable account
        my_account.data = data; // the new data is stored in my account , data field
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /*When using init, one must also provide PAYER, which will fund the account creation,
     SPACE which defines how large the account should be, 
    system_program, which is required by the runtime for creating the account.*/
    #[account(init, payer = user, space = 8 + 8)] 
    /*my_account field is marked with the init attribute. 
    This will create a new account owned by the current program, zero initialized */
    pub my_account: Account<'info, MyAccount>, //deserialized data structure is MyAccount.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
