use anchor_lang::prelude::*;

declare_id!("DevUcESSMdzU8BKHBa7br43MqoeXFHa4nDNZmZDDiFXK");

pub const   ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod lab8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies:
    Vec<String>) -> Result<()>{
        ctx.accounts.favorites.set_inner(Favorites{
            number,
            color,
            hobbies,
        });
    }
}

#[derive(Accounts)]

pub struct SetFavorites{
    #[account(mut)]
    pub user: Signer<'info>,


#[account(
    init,
    payer = user,
    space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
    seeds= [b"favorites", user.key().as_ref()],
    bump
)]
pub favorites: Account<'info, Favorites>,

pub system_program: Program<'info, System>,
}
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
