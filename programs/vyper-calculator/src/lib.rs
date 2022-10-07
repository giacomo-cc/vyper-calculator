use anchor_lang::prelude::*;
use rust_decimal::Decimal;

declare_id!("H13Jr5DwmYdSBS5XSFCFFPGbagFRNxN3EvgTg1W1uJ3E");

#[program]
pub mod vyper_calculator {
    use rust_decimal_macros::dec;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.authority = ctx.accounts.authority.key();
        ctx.accounts.state.value = dec!(0).serialize();
        Ok(())
    }

    pub fn sum(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        let a_dec = Decimal::from(a);
        let b_dec = Decimal::from(b);
        let res = a_dec + b_dec;
        ctx.accounts.state.value = res.serialize();
        Ok(())
    }

    pub fn sub(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        let a_dec = Decimal::from(a);
        let b_dec = Decimal::from(b);
        let res = a_dec - b_dec;
        ctx.accounts.state.value = res.serialize();
        Ok(())
    }

    pub fn mul(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        let a_dec = Decimal::from(a);
        let b_dec = Decimal::from(b);
        let res = a_dec * b_dec;
        ctx.accounts.state.value = res.serialize();
        Ok(())
    }

    pub fn div(ctx: Context<OpContext>, a: u64, b: u64) -> Result<()> {
        let a_dec = Decimal::from(a);
        let b_dec = Decimal::from(b);
        let res = a_dec / b_dec;
        ctx.accounts.state.value = res.serialize();
        Ok(()) 
    }

}

#[account]
pub struct State {
    pub authority: Pubkey,
    pub value: [u8; 16]
}

impl State {
    pub const LEN: usize = 8 + // discriminator
    32 + // pub authority: Pubkey,
    16;  // pub value: [u8; 16]
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
