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
        //TODO: Check that initializer.key === player.key
        let player = &mut ctx.accounts.player;
        player.squad = squad;
        Ok(())
    }

    pub fn delete_player(_ctx: Context<DeletePlayer>, name: String) -> Result<()> {
        msg!("Player account for {} deleted", name);
        Ok(())
    }

    pub fn add_squad(
        ctx: Context<AddSquad>,
        name: String,
        logo: String,
        motto: String,
        players: Vec<Player>
    ) -> Result<()> {
        msg!("Squad Account Created");
        msg!("Name: {}", name);
        msg!("Logo: {}", logo);
        msg!("Motto: {}", motto);
        msg!("Players: {}", players);

        let squad = &mut ctx.accounts.squad;
        squad.creator_key = ctx.accounts.creator.key();
        squad.name = name;
        squad.logo = logo;
        squad.motto = motto;
        squad.players = players;
        Ok(())
    }

    pub fn update_squad(
        ctx: Context<UpdateSquad>,
        players: Vec<Player>
    ) -> Result<()> {
        msg!("Squad Account Updated");
        msg!("Players: {}", players);
        //TODO: Check that initializer.key === player.key
        let squad = &mut ctx.accounts.squad;
        squad.players = players;
        Ok(())
    }

    pub fn delete_squad(
        _ctx: Context<DeleteSquad>, 
        name: String
    ) -> Result<()> {
        msg!("Squad account for {} deleted", name);
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
    pub player: Account<'info, Player>,
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
    pub player: Account<'info, Player>,
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
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(name:String, logo:String, motto:String)]
pub struct AddSquad<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        // For space, let's say we're supporting up to 100 players per squad with max length of 15 characters each for player name length and squad name length 
        space = 8 + 32 + 4 + name.len() + 4 + logo.len() + 4 + motto.len() + 4 + 100 * (8 + 32 + 4 + 15 + 4 + 15 + 8)
    )]
    pub squad: Account<'info, Squad>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, logo:String, motto:String)]
pub struct UpdateSquad<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + logo.len() + 4 + motto.len() + 4 + 100 * (8 + 32 + 4 + 15 + 4 + 15 + 8),
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub squad: Account<'info, Squad>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteSquad<'info> {
    #[account(
        mut,
        seeds=[name.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[account]
pub struct Player {
    pub key: Pubkey,
    pub name: String,
    pub squad: String,
    pub score: u64
}

#[account]
pub struct Squad {
    pub creator_key: Pubkey,
    pub name: String, 
    pub logo: String, //URI of logo 
    pub motto: String, //max chars 30
    pub players: Vec<Player>
}

#[account]
pub struct SquadList {
    pub creator: Pubkey,
    pub squads: Vec<Squad>
}