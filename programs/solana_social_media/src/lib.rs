use anchor_lang::prelude::*;

declare_id!("59chiAcJ99ttxaDLxcsJh8Be2tChirZfWsWxkd2SRGDj");

#[program]
pub mod solana_social_media {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// create pointer to content
// like content
// comment on content
// earn rewards on content

#[error_code]
pub enum ErrorCode {
    LinkAlreadySubmitted,
    OverflowUnderflow,
    NoTextSubmitted,
    CommentTooLarge,
}
