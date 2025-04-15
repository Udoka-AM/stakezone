use anchor_lang::prelude::*;

declare_id!("5m6vknYePNRhEecTTnkW7L9NhZxrgK9A2axnVpyDPkFF");

#[program]
pub mod stakezone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
