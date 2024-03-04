use anchor_lang::{prelude::*, solana_program::syscalls};

declare_id!("59chiAcJ99ttxaDLxcsJh8Be2tChirZfWsWxkd2SRGDj");

#[program]
pub mod solana_social_media {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority,
        space = Senddit::LEN,
        seeds = [
            b"senddit"
        ],
        bump
    )]
    pub senddit: Account<'info, Senddit>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPostStore<'info> {
    #[account(
        mut,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK: Account must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = PostStore::LEN,
        seeds = [
            (((Clock::get().unwrap().unix_timestamp.abs() as f64) / (60.0 * 60.0 * 24.0)) as u128).to_string().as_bytes().as_ref(),
        ], 
        bump
    )]
    pub post_store: Account<'info, PostStore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(link: String)]
pub struct PostLink<'info> {
    #[account(
        mut,
        seeds = [b"senddit"],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK: Acocunt must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,


    /// CHEKC: Account must match the creator of the store
    #[account(
        mut,
        address = post_store.authority
    )]
    pub poster_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            (((Clock::get().unwrap().unix_timestamp.abs() as f64) / (60.0 * 60.0 * 24.0)) as u128).to_string().as_bytes().as_ref(),
        ],
        bump = post_store.bump
    )]
    pub post_store: Account<'info, PostStore>,

    #[account(
        init,
        payer = authority,
        space = Post::LEN,
        seeds = [
            post_store.key().as_ref(),
            (post_store.posts + 1).to_string().as_bytes().as_ref()
        ],
        bump
    )]
    pub post: Account<'info, Post>,

    /// CHECK: It's okay not to deserialize this, its just to prvent duplicate links 
    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [
            link.as_bytes().as_ref()
        ],
         bump
    )]
    pub post_pda: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(number: String)]
pub struct UpvotePost<'info> {
    #[account(
        mut,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK:: Account must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Account must match the maker of the post
    #[account(
        mut,
        address = post.authority
    )]
    pub poster_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            // incorrect seed. use time stamp
            post.key().as_ref()
        ], 
        bump = post_store.bump
    )]
    pub post_store: Account<'info, PostStore>,

    #[account(
        mut,
        seeds = [
            post_store.key().as_ref(),
            number.as_bytes().as_ref(),
        ],
        bump = post.bump
    )]
    post: Account<'info, Post>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitCommentStore<'info> {
    #[account(
        mut,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK: Account must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CommentStore::LEN,
        seeds = [
            post.key().as_ref()
        ],
        bump
    )]
    pub comment_store: Account<'info, CommentStore>,

    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct PostComment<'info> {
    #[account(
        mut,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK: Account must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Account must match the creator of the store
    #[account(
        mut, 
        address = comment_store.authority
    )]
    pub commenter_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            post.key().as_ref()
        ],
        bump =comment_store.bump
    )]
    pub comment_store: Account<'info, CommentStore>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(
        init,
        payer = authority,
        space = Comment::LEN,
        seeds = [
            comment_store.key().as_ref(),
            (comment_store.comments + 1).to_string().as_bytes().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(number: String)]
pub struct UpvoteComment<'info> {
    #[account(
        mut,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    /// CHECK: Account must match our config
    #[account(
        mut,
        address = senddit.treasury
    )]
    pub treasury: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Account must match the maker of the comments
    #[account(
        mut,
        address = comment.authority
    )]
    pub commenter_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            post.key().as_ref()
        ],
        bump = comment_store.bump
    )]
    pub comment_store: Account<'info, CommentStore>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(
        mut,
        seeds = [
            comment_store.key().as_ref(),
            number.as_bytes().as_ref()
        ],
        bump = comment.bump
    )]
    pub comment: Account<'info, Comment>,

    pub system_program: Program<'info, System>,
}


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
    pub comments: u128,
    pub bump: u8,
}

impl CommentStore {
    pub const LEN: usize = DISCRIMATOR + PUBKEY + USIGNED_128;
}

#[account]
pub struct Comment {
    pub authority: Pubkey,
    pub comment: String,
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
