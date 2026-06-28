use anchor_lang::prelude::*;

use crate::{constants::COUNTER_SEED, state::Counter};

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [COUNTER_SEED], bump)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn handle_increment(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.counter.count += 1;
    msg!("Counter incremented to {}", ctx.accounts.counter.count);
    Ok(())
}
