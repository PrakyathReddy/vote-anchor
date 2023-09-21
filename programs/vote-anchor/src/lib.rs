use anchor_lang::prelude::*;

declare_id!("GAutBpYRWyKY6pARFU78JR2akq9GMBL2c1Goish5PzCc");

#[program]
pub mod vote_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes = 0;
        ctx.accounts.vote.bump = *ctx.bumps.get("vote").unwrap(); // unwrap because 'get' returns an option
        Ok(())
    }

    pub fn upvote(ctx: Context<VoteInteraction>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes += 1;
        Ok(())
    }

    pub fn downvote(ctx: Context<VoteInteraction>, _hash: Vec<u8>) -> Result<()> {
        ctx.accounts.vote.votes -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_hash: Vec<u8>)] // by using instruction, we are exposing the hash into our accounts atruct, inside the context
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>, // we need someone to pau for the account
    #[account(
        init,
        payer = signer,
        space = Vote::LEN,
        seeds = [b"vote", _hash.as_slice().as_ref()], // seeds are going to be set to seeds ("vote") + this hash
        bump // here, it is going to calculate the bump for us. Save it using the initialize instruction
    )]
    vote: Account<'info, Vote>, // going to hold the number of votes
    system_program: Program<'info, System>, // since we are creating an account, we need the system program
}

#[derive(Accounts)]
#[instruction(_hash: Vec<u8>)]
pub struct VoteInteraction<'info> {
    #[account(mut)]
    signer: Signer<'info>, // we need someone to pau for the account
    #[account(
        mut,
        seeds = [b"vote", _hash.as_slice().as_ref()], // seeds are going to be set to seeds ("vote") + this hash
        bump = vote.bump, // reusing the saved bump
    )]
    vote: Account<'info, Vote>, // going to hold the number of votes
}

#[account]
pub struct Vote {
    votes: i64,
    bump: u8,
}

impl Vote {
    const LEN: usize = 8 + 8 + 1; // 8 bytes for the anchor discriminator, 8 bytes for the votes, 1 byte for the bump
}
