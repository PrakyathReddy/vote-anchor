use anchor_lang::prelude::*;

declare_id!("GAutBpYRWyKY6pARFU78JR2akq9GMBL2c1Goish5PzCc");

#[program]
pub mod vote_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
