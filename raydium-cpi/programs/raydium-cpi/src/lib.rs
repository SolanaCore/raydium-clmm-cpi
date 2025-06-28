pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("924xdevKoM2tQ2CELcCpMpBJHeXVVq4Nq8AuEs1X1vLq");

#[program]
pub mod raydium_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
