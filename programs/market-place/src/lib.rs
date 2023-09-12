use anchor_lang::prelude::*;

declare_id!("DApTMHgStwAesfm92JNo9JQkMg7rVS83jwa2rxiegFix");

#[program]
pub mod market_place {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
