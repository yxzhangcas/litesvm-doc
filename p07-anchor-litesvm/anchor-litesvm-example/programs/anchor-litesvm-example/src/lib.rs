pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4RLEF64sgjkSzpY6hTBHbpBJg9qbv4mbQfArJDKJwEvU");

#[program]
pub mod anchor_litesvm_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, name: String) -> Result<()> {
        initialize::handler(ctx, seed, name)
    }
}
