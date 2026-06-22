use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = MyAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub user_account: Account<'info, MyAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MyAccount {
    pub seed: u64,
    #[max_len(20)]
    pub name: String,
}

pub fn handler(ctx: Context<Initialize>, seed: u64, name: String) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    ctx.accounts.user_account.name = name;
    ctx.accounts.user_account.seed = seed;
    Ok(())
}
