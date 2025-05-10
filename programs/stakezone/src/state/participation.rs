use anchor_lang::prelude::*;

#[account]
pub struct Participation {
    pub pool: Pubkey,
    pub participant: Pubkey,
    pub fpl_team_id: u32,
    pub join_time: i64,
    pub score: u16,
    pub bump: u8,
}

impl Participation {
    pub const MAX_SIZE: usize = 8 + // Discriminator
                                32 + // pool
                                32 + // participant
                                4 + // fpl_team_id
                                8 + // join_time
                                2 + // score
                                1;  // bump

    // pub fn did_participate(
    //     program_id: &Pubkey,
    //     pool: &Pubkey,
    //     participant: &Pubkey,
    // ) -> Result<bool> {
    //     let (pda, _) = Pubkey::find_program_address(
    //         &[b"participation", pool.as_ref(), participant.as_ref()],
    //         program_id,
    //     );
        
    //     // Check if the account exists by trying to convert to AccountInfo
    //     let account_info = AccountInfo::try_from(&pda as &dyn IntoAccountInfo).ok();
    //     Ok(account_info.is_some())
    // }
}