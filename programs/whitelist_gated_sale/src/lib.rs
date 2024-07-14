use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
// use mpl_token_metadata::instruction::{create_metadata_accounts, CreateMetadataAccountsArgs};
// use mpl_token_metadata::state::Creator;

declare_id!("9Gbwi5Et3btCXaa2u1XgYmfP7myDN6yp7xSbyMU86tmt");

#[program]
pub mod whitelist_gated_sale {
    use super::*;

    const TOKEN_PRICE: u64 = 250_000_000; // Price of the token (in lamports)
    const MAX_TOKENS_PER_WALLET: u64 = 100; // Maximum tokens a wallet can purchase

    pub fn initialize(
        ctx: Context<Initialize>,
        unique_id: u64,
        whitelist: Vec<Pubkey>,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = *ctx.accounts.user.key;
        if whitelist.is_empty() {
            state.whitelist = Vec::new();
        } else {
            state.whitelist = whitelist;
        }
        Ok(())
    }

    pub fn add_user_to_white_list(ctx: Context<AddUser>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let new_account = &mut ctx.accounts.new_account;
        let user = &ctx.accounts.user;

        if state.owner != *user.key {
            return Err(ErrorCode::NoPermissionToUpdateList.into());
        }

        if !state.whitelist.contains(new_account.key) {
            state.whitelist.push(*new_account.key);
        }

        Ok(())
    }

    pub fn remove_user_from_list(ctx: Context<RemoveUser>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let new_account = &mut ctx.accounts.new_account;
        let user = &ctx.accounts.user;

        if state.owner != *user.key {
            return Err(ErrorCode::NoPermissionToUpdateList.into());
        }

        if state.whitelist.contains(new_account.key) {
            state.whitelist.retain(|&x| x != *new_account.key);
        }

        Ok(())
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;
        let buyer = &ctx.accounts.buyer;

        if !state.whitelist.contains(buyer.key) {
            return Err(ErrorCode::NotWhitelisted.into());
        }

        let user_info = &mut ctx.accounts.user_info;
        if user_info.purchased_amount + amount > MAX_TOKENS_PER_WALLET {
            return Err(ErrorCode::PurchaseLimitExceeded.into());
        }

        let total_price = TOKEN_PRICE.checked_mul(amount).ok_or(ErrorCode::Overflow)?;

        // // Transfer lamports to the treasury
        **ctx.accounts.buyer.try_borrow_mut_lamports()? -= total_price;
        **ctx.accounts.treasury.try_borrow_mut_lamports()? += total_price;

        // Mint tokens to the buyer
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.token_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;

        user_info.account = buyer.key().to_string();
        user_info.purchased_amount += amount;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(unique_id: u64)]
pub struct Initialize<'info> {
    #[account(init, payer = user, seeds = ["whitelist".as_ref(), user.key().as_ref(), &unique_id.to_le_bytes()], space = 8 + 32 + 4 + 32 + (32 * 100), bump)]
    // Space for a vector of 100 pubkeys
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub new_account: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveUser<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub new_account: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(init_if_needed, payer = buyer, space = 8 + 32 + 8 + 32, seeds=[buyer.key().as_ref()],  bump)]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub whitelist: Vec<Pubkey>,
    pub owner: Pubkey,
}

#[account]
pub struct UserInfo {
    pub account: String,
    pub purchased_amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Buyer is not whitelisted.")]
    NotWhitelisted,
    #[msg("Purchase limit exceeded.")]
    PurchaseLimitExceeded,
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Underflow occurred.")]
    Underflow,
    #[msg("This list was not created by you")]
    NoPermissionToUpdateList,
}
