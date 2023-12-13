use anchor_lang::prelude::*;

declare_id!("APozxuLZW4crR9nTPzQJEUpWMLubeC3bBUWs7ewCwtPE");

#[program]
pub mod bonk_squad_log {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
