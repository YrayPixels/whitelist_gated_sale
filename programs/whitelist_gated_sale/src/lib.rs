use anchor_lang::prelude::*;
// use anchor_spl::token::{self, InitializeMint, Mint, Token, TokenAccount};
// use mpl_token_metadata::instruction::{create_metadata_accounts, CreateMetadataAccountsArgs};
// use mpl_token_metadata::state::Creator;

declare_id!("CRKtvQJeuqgASZzoSFnRh65ihHRVyMzidrw7sQmfYKi7");

#[program]
pub mod whitelist_gated_sale {
    use super::*;

    const TOKEN_PRICE: u64 = 1_000_000; // Price of the token (in lamports)
    const MAX_TOKENS_PER_WALLET: u64 = 100; // Maximum tokens a wallet can purchase

    pub fn initialize(ctx: Context<Initialize>, whitelist: Vec<Pubkey>) -> Result<()>{
        let state = &mut ctx.accounts.state;
        state.whitelist = whitelist;
        Ok(())
    }

    // pub fn create_token(
    //     ctx: Context<CreateToken>,
    //     decimals: u8,
    //     name: String,
    //     symbol: String,
    //     uri: String,
    // ) -> ProgramResult {
    //     let cpi_accounts = InitializeMint {
    //         mint: ctx.accounts.mint.to_account_info(),
    //         rent: ctx.accounts.rent.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     token::initialize_mint(cpi_ctx, decimals, ctx.accounts.mint_authority.key, None)?;

    //     let metadata_accounts = create_metadata_accounts(
    //         ctx.accounts.metadata.key(),
    //         ctx.accounts.mint.key(),
    //         ctx.accounts.mint_authority.key(),
    //         ctx.accounts.mint_authority.key(),
    //         ctx.accounts.mint_authority.key(),
    //         name,
    //         symbol,
    //         uri,
    //         None,
    //         0,
    //         true,
    //         false,
    //         None,
    //         None,
    //     );
    //     let ix = metadata_accounts.unwrap();
    //     anchor_lang::solana_program::program::invoke(
    //         &ix,
    //         &[
    //             ctx.accounts.metadata.to_account_info(),
    //             ctx.accounts.mint.to_account_info(),
    //             ctx.accounts.mint_authority.to_account_info(),
    //             ctx.accounts.rent.to_account_info(),
    //             ctx.accounts.system_program.to_account_info(),
    //             ctx.accounts.token_program.to_account_info(),
    //             ctx.accounts.token_metadata_program.to_account_info(),
    //         ],
    //     )?;

    //     Ok(())
    // }

    // pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
    //     let state = &ctx.accounts.state;
    //     let buyer = &ctx.accounts.buyer;

    //     if !state.whitelist.contains(buyer.key) {
    //         return Err(ErrorCode::NotWhitelisted.into());
    //     }

    //     let user_info = &mut ctx.accounts.user_info;
    //     if user_info.purchased_amount + amount > MAX_TOKENS_PER_WALLET {
    //         return Err(ErrorCode::PurchaseLimitExceeded.into());
    //     }

    //     let total_price = TOKEN_PRICE.checked_mul(amount).ok_or(Err(ErrorCode::Overflow.into()));

    //     // Transfer lamports to the treasury
    //     **ctx.accounts.buyer.try_borrow_mut_lamports()? -= total_price;
    //     **ctx.accounts.treasury.try_borrow_mut_lamports()? += total_price;

    //     // Mint tokens to the buyer
    //     let cpi_accounts = token::MintTo {
    //         mint: ctx.accounts.mint.to_account_info(),
    //         to: ctx.accounts.token_account.to_account_info(),
    //         authority: ctx.accounts.token_authority.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     token::mint_to(cpi_ctx, amount)?;

    //     user_info.purchased_amount += amount;

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (32 * 100))]
    // Space for a vector of 100 pubkeys
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct CreateToken<'info> {
//     #[account(init, payer = payer, mint::decimals = decimals, mint::authority = mint_authority)]
//     pub mint: Account<'info, Mint>,
//     #[account(signer)]
//     pub mint_authority: AccountInfo<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(mut)]
//     pub metadata: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>,
//     pub token_metadata_program: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct BuyToken<'info> {
//     #[account(mut)]
//     pub state: Account<'info, State>,
//     #[account(mut)]
//     pub buyer: Signer<'info>,
//     #[account(init_if_needed, payer = buyer, space = 8 + 32 + 8)]
//     pub user_info: Account<'info, UserInfo>,
//     #[account(mut)]
//     pub treasury: AccountInfo<'info>,
//     pub mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub token_account: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     pub token_authority: AccountInfo<'info>,
// }

#[account]
pub struct State {
    pub whitelist: Vec<Pubkey>,
}

#[account]
pub struct UserInfo {
    pub purchased_amount: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Buyer is not whitelisted.")]
    NotWhitelisted,
    #[msg("Purchase limit exceeded.")]
    PurchaseLimitExceeded,
    #[msg("Overflow occurred.")]
    Overflow,
}
