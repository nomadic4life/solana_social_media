use anchor_lang::{prelude::*, solana_program::system_instruction::transfer, solana_program::program::invoke};

declare_id!("59chiAcJ99ttxaDLxcsJh8Be2tChirZfWsWxkd2SRGDj");

#[program]
pub mod solana_social_media {
    use super::*;

    // LOGIC
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let Initialize {
            senddit,
            authority,
            ..
        } = ctx.accounts;

        senddit.authority = authority.key();
        senddit.treasury = authority.key();
        senddit.fee = (0.001 * 1e9) as u64; // 0.001 SOL
        senddit.bump = ctx.bumps.senddit;

        return Ok(());
    }

    pub fn update_fees(ctx: Context<UpdateFees>, amount: f64) -> Result<()> {
        let UpdateFees {
            senddit,
            ..
        } = ctx.accounts;

        senddit.fee = (amount * 1e9) as u64;

        return Ok(());
    }

    pub fn init_post_store(ctx: Context<InitPostStore>) -> Result<()> {
        let InitPostStore {
            senddit,
            treasury,
            post_store,
            authority,
            ..
        } = ctx.accounts;

        payout_fees(treasury, authority, senddit, None);

        post_store.authority = authority.key();
        post_store.posts = 0;
        post_store.bump = ctx.bumps.post_store;

        return Ok(());
    }

    pub fn post_link(ctx: Context<PostLink>, link: String) -> Result<()> {
        let PostLink {
            senddit,
            treasury,
            post_store,
            post,
            authority,
            poster_wallet,
            ..
        } = ctx.accounts;



        payout_fees(poster_wallet, authority, senddit, Some(treasury));

        post_store.posts = post_store
            .posts.checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        post.authority = authority.key();
        post.link = link;
        post.upvotes = 1;
        post.comments  = 0;
        post.bump = ctx.bumps.post;

        return Ok(());
    }

    pub fn upvote_post(ctx: Context<UpvotePost>, _number: String) -> Result<()>  {

        let UpvotePost {
            senddit,
            treasury,
            post,
            authority, 
            poster_wallet,
            ..
        } = ctx.accounts;

        payout_fees(poster_wallet, authority, senddit, Some(treasury));

        post.upvotes = post
            .upvotes
            .checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        return Ok(());
    }

    pub fn init_comment_store(ctx: Context<InitCommentStore>) -> Result<()> {
        let InitCommentStore {
            senddit,
            treasury,
            comment_store,
            authority,
            ..
        } = ctx.accounts;

        payout_fees(treasury, authority ,senddit, None);

        comment_store.authority = authority.key();
        comment_store.comments = 0;
        comment_store.bump = ctx.bumps.comment_store;

        return Ok(());
    }

    pub fn post_comment(ctx: Context<PostComment>, input: String, reply: Option<Pubkey>) -> Result<()> {
        let PostComment {
            senddit,
            treasury,
            comment_store,
            authority,
            commenter_wallet,
            comment,
            post,
            ..
        } = ctx.accounts;

        payout_fees(commenter_wallet, authority, senddit, Some(treasury));

        post.comments = post.comments
            .checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        comment_store.comments = comment_store.comments
            .checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        comment.authority = authority.key();
        comment.comment = input;
        comment.upvotes = 1;
        comment.comments  = 0;
        comment.bump = ctx.bumps.comment;
        comment.reply_to = reply;


        return Ok(());
    }

    pub fn upvote_comment(ctx: Context<UpvoteComment>, _number: String) -> Result<()>  {
        let UpvoteComment {
            senddit,
            treasury,
            comment,
            authority, 
            commenter_wallet,
            ..
        } = ctx.accounts;

        payout_fees(commenter_wallet, authority, senddit, Some(treasury));

        comment.upvotes = comment
            .upvotes
            .checked_add(1)
            .ok_or(ErrorCode::OverflowUnderflow)?;

        return Ok(());
    }

}



// UTILS

pub fn payout_fees<'info>(
    to: &mut UncheckedAccount<'info>,
    from: &mut Signer<'info>,
    senddit: &mut Account<'info, Senddit>,
    treasury: Option<&mut UncheckedAccount<'info>>,
) {
    if let Some(treasury) = treasury {
        invoke(
            &transfer(
                from.key,
                treasury.key,
                senddit.fee
            ),
            &[
                from.to_account_info(),
                treasury.to_account_info(),
            ]
        ).unwrap();
    }

    invoke(
        &transfer(from.key, to.key, senddit.fee),
        &[
            from.to_account_info(),
            to.to_account_info(),
        ]
    ).unwrap(); 
}

// DATA VALIDATORS

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
pub struct UpdateFees<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [
            b"senddit"
        ],
        bump = senddit.bump
    )]
    pub senddit: Account<'info, Senddit>,

    #[account(mut)]
    pub authority: Signer<'info>,
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
            (((Clock::get()
                .unwrap()
                .unix_timestamp.abs() as f64) / (60.0 * 60.0 * 24.0)) as u64 * 1000)
                    .to_string()
                    .as_bytes()
                    .as_ref(),
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


    /// CHECK: Account must match the creator of the store
    #[account(
        mut,
        address = post_store.authority
    )]
    pub poster_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            (((Clock::get()
                .unwrap()
                .unix_timestamp.abs() as f64) / (60.0 * 60.0 * 24.0)) as u64 * 1000)
                    .to_string()
                    .as_bytes()
                    .as_ref()
        ], 
        bump = post_store.bump
    )]
    pub post_store: Account<'info, PostStore>,

    #[account(
        constraint = Post::is_valid_post_size(&link) @ ErrorCode::LinkInvalidSize,
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
            (((Clock::get()
                .unwrap()
                .unix_timestamp.abs() as f64) / (60.0 * 60.0 * 24.0)) as u64 * 1000)
                    .to_string()
                    .as_bytes()
                    .as_ref(),
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
#[instruction(input: String)]
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
        constraint = Comment::is_valid_comment_size(&input) @ ErrorCode::CommentInvalid,
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



const EMPTY: usize = 0;
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
    pub const LEN: usize = DISCRIMATOR + PUBKEY + USIGNED_128 + BUMP;
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

    pub fn is_valid_post_size(link: &String) -> bool {

        if link.len() >= MAX_LINK_SIZE || link.len() == EMPTY  {
            return false
        }

        return true;
    }
}

#[account]
pub struct CommentStore {
    pub authority: Pubkey,
    pub comments: u128,
    pub bump: u8,
}

impl CommentStore {
    pub const LEN: usize = DISCRIMATOR + PUBKEY + USIGNED_128 + BUMP;
}

#[account]
pub struct Comment {
    pub authority: Pubkey,
    pub comment: String,
    pub upvotes: u64,
    pub comments: u64,
    pub reply_to: Option<Pubkey>,
    pub bump: u8,
}

impl Comment {
    pub const LEN: usize =
        DISCRIMATOR + PUBKEY + STRING_PREFIX + MAX_COMMENT_SIZE + UNSIGNED_64 + UNSIGNED_64 + BUMP;

    pub fn is_valid_comment_size(comment: &String) -> bool {

        if comment.len() >= MAX_COMMENT_SIZE || comment.len() == EMPTY  {
            return false
        }

        return true;
    }
}

#[error_code]
pub enum ErrorCode {
    LinkAlreadySubmitted,
    OverflowUnderflow,
    NoTextSubmitted,
    CommentTooLarge,
    LinkTooLarge,
    LinkInvalidSize,
    CommentInvalid,
}
