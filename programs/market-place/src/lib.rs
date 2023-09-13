use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("DApTMHgStwAesfm92JNo9JQkMg7rVS83jwa2rxiegFix");

#[program]
pub mod market_place {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        require!(name.len() < 33, MarkteplaceError::NameTooLong);
        require!(name.len() > 3, MarkteplaceError::NameTooShort);
        ctx.accounts.marketplace.admin = *ctx.accounts.admin.key;
        ctx.accounts.marketplace.fee = fee;
        ctx.accounts.marketplace.name = name;
        Ok(())
    }
    pub fn add_collection(ctx: Context<AddCollection>) -> Result<()> {
        ctx.accounts.allowed_collection.collection = ctx.accounts.collection.key();
        Ok(())
    }
    // pub fn list(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn de_list(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn purchase(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn offer(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace".as_ref(), name.as_str().as_bytes()],
        bump,
        space = Marketplace::LEN,
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = admin,
    )]
    rewards: Account<'info, Mint>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddCollection<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        has_one = admin,
        seeds = [b"marketplace".as_ref(), marketplace.name.as_str().as_bytes()],
        bump,
    )]
    /// CHECK: check is not required  
    marketplace: Account<'info, Marketplace>,
    collection: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [marketplace.key().as_ref(), collection.key().as_ref()],
        bump,
        space = 8 + 32,
    )]
    allowed_collection: Account<'info, AllowedCollection>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct AddCollection<'info> {
//     #[account(mut)]
//     admin: Signer<'info>,
//     #[account(
//         seeds = [b"marketplace".as_ref(), marketplace.name.as_str().as_bytes()],
//         bump,
//     )]
//     marketplace: Account<'info, Marketplace>,
//     #[account()]
//     collection: Account<'info, Mint>,
//     #[account(
//         init,
//         payer = admin,
//         seeds = [marketplace.key().as_ref(), collection.key().as_ref()],
//         bump,
//         space = 8 + 32,
//     )]
//     allowed_collection: Account<'info, AllowedCollection>,
//     token_program: Program<'info, Token>,
//     system_program: Program<'info, System>,
// }

#[account]
pub struct Marketplace {
    admin: Pubkey,
    fee: u16,
    name: String,
    bump: u8,
    treasury_bump: u8,
}

impl Marketplace {
    const LEN: usize = 8 + 32 + 2 + 4 + 32;
}

#[account]
pub struct Collection {
    name: String,
    collection: Pubkey,
}

#[account]
pub struct Listing {
    owner: Pubkey,
    mint: Pubkey,
    price: u64,
    expires_at: i64,
}

#[account]
pub struct AllowedCollection {
    collection: Pubkey,
}

#[error_code]
pub enum MarkteplaceError {
    #[msg("Name must be between 3 and 32 characters")]
    NameTooLong,
    #[msg("Name must be between 3 and 32 characters")]
    NameTooShort,
}
