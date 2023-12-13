use anchor_lang::prelude::*;

declare_id!("APozxuLZW4crR9nTPzQJEUpWMLubeC3bBUWs7ewCwtPE");

#[program]
pub mod bonk_squad_log {
    use super::*;

    pub fn add_player(
        ctx: Context<AddPlayer>,
        name: String,
        squad: String,
        score: u64,
    ) -> Result<()> {
        msg!("Player Account Created");
        msg!("Name: {}", name);
        msg!("Squad: {}", squad);
        msg!("Score: {}", score);

        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.initializer.key();
        player.name = name;
        player.squad = squad;
        player.score = score;
        Ok(())
    }

    pub fn update_player(
        ctx: Context<UpdatePlayer>,
        name: String,
        squad: String,
    ) -> Result<()> {
        msg!("Player Account Updated");
        msg!("Name: {}", name);
        msg!("Squad: {}", squad);

        let player = &mut ctx.accounts.player;
        player.squad = squad;
        Ok(())
    }

    pub fn delete_player(_ctx: Context<DeletePlayer>, name: String) -> Result<()> {
        msg!("Player account for {} deleted", name);
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name:String, squad:String)]
pub struct AddPlayer<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4 + name.len() + 4 + squad.len() + 8
    )]
    pub player: Account<'info, PlayerAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, squad:String)]
pub struct UpdatePlayer<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + squad.len() + 8,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub player: Account<'info, PlayerAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeletePlayer<'info> {
    #[account(
        mut,
        seeds=[name.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub player: Account<'info, PlayerAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}
#[account]

pub struct PlayerAccountState {
    pub key: Pubkey,
    pub name: String,
    pub squad: String,
    pub score: u64
}