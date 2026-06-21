use anchor_lang::prelude::*;

declare_id!("9CZ59UifjPTDkbbDQPJrwvU2x9Ku4v1zY12ufifDSeaL");

#[program]
mod crud {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        title: String,
        message: String,
    ) -> Result<()> {
        let data_store = &mut ctx.accounts.data_store;
        data_store.owner = ctx.accounts.owner.key();
        data_store.title = title;
        data_store.message = message;
        Ok(())
    }

    pub fn update(
        ctx: Context<Update>,
        _title: String,
        message: String,
    ) -> Result<()> {
       let data_store = &mut ctx.accounts.data_store;
        data_store.message = message;

        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>, _title: String) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct DataStore {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct Create<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        payer = owner, 
        space = 8 + DataStore::INIT_SPACE
    )]
    pub data_store: Account<'info, DataStore>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        realloc = 8 + DataStore::INIT_SPACE,
        realloc::payer = owner, 
        realloc::zero = true, 
    )]
    pub data_store: Account<'info, DataStore>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Delete<'info> {
    #[account( 
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        close= owner,
    )]
    pub data_store: Account<'info, DataStore>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}