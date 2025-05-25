use anchor_lang::prelude::*;

declare_id!("5Y61S87J7uvrcqCaHt5Svii3pLA3XyYUZpXzM3Qkuyyb");

#[program]
pub mod verify {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
