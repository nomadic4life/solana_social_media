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

const DISCRIMATOR: usize = 8;
const PUBKEY: usize = 32;
const UNSIGNED_64: usize = 8;
const USIGNED_128: usize = 16;
const STRING_PREFIX: usize = 4;
const MAX_LINK_SIZE: usize = 96 * 4;
const MAX_COMMENT_SIZE: usize = 192 * 4;
const BUMP: usize = 1;

#[account]
pub struct Senddit {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub fee: u64,
    pub bump: u8,
}

impl Senddit {
    pub const LEN: usize = DISCRIMATOR + PUBKEY + PUBKEY + UNSIGNED_64 + BUMP;
}

#[account]
pub struct PostStore {
    pub authority: Pubkey,
    pub posts: u128,
    pub bump: u8,
}

impl PostStore {
    pub const LEN: usize = DISCRIMATOR + PUBKEY + USIGNED_128;
}

#[account]
pub struct Post {
    pub authority: Pubkey,
    pub link: String,
    pub upvotes: u64,
    pub comments: u64,
    pub bump: u8,
}

impl Post {
    pub const LEN: usize =
        DISCRIMATOR + PUBKEY + STRING_PREFIX + MAX_LINK_SIZE + UNSIGNED_64 + UNSIGNED_64 + BUMP;
}

#[account]
pub struct CommentStore {
    pub authority: Pubkey,
    pub posts: u128,
    pub bump: u8,
}

impl CommentStore {
    pub const LEN: usize = DISCRIMATOR + PUBKEY + USIGNED_128;
}

#[account]
pub struct Comment {
    pub authority: Pubkey,
    pub link: String,
    pub upvotes: u64,
    pub comments: u64,
    pub bump: u8,
}

impl Comment {
    pub const LEN: usize =
        DISCRIMATOR + PUBKEY + STRING_PREFIX + MAX_COMMENT_SIZE + UNSIGNED_64 + UNSIGNED_64 + BUMP;
}

#[error_code]
pub enum ErrorCode {
    LinkAlreadySubmitted,
    OverflowUnderflow,
    NoTextSubmitted,
    CommentTooLarge,
}
