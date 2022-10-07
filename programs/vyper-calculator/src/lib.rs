use anchor_lang::prelude::*;

declare_id!("H13Jr5DwmYdSBS5XSFCFFPGbagFRNxN3EvgTg1W1uJ3E");

#[program]
pub mod vyper_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.authority = ctx.accounts.authority.key();
        ctx.accounts.state.value = 0;
        Ok(())
    }

    pub fn sum(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.state.value = a + b;
        Ok(())
    }

    pub fn sub(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.state.value = a - b;
        Ok(())
    }

    pub fn mul(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.state.value = a * b;
        Ok(())
    }

    pub fn div(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.state.value = a / b;
        Ok(()) 
    }

}

#[account]
pub struct State {
    pub authority: Pubkey,
    pub value: u64
}

impl State {
    pub const LEN: usize = 8 + // discriminator
    32 + // pub authority: Pubkey,
    8;  // pub value: u64
}

#[derive(Accounts)]
pub struct OpContext<'info> {
    /// Calculator state account
    #[account(mut, has_one = authority)]
    pub state: Account<'info, State>,

    /// Signer account
    #[account()]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Calculator state account
    #[account(init, payer = authority, space = State::LEN)]
    pub state: Account<'info, State>,

    /// Signer account
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
