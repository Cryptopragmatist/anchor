

use anchor_lang::prelude::*; // using anchor library

#[program] // instruction handler, client entry point, rpc request handler
mod basic_0 {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult { //Context which is a simple container for the currently executing, option result 
        Ok(())
    }
}

#[derive(Accounts)] // all accounts specified for an instruction e.g addresses need for the program to execute
//marks a struct containing all the accounts that must be specified for a given instruction
pub struct Initialize {}