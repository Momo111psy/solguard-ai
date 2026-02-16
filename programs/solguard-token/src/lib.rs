use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("3WB548f6DytYL9mGizZ55orA3ExAiU5shxNSTQ9Cgo9m");

#[program]
pub mod solguard_token {
    use super::*;

    /// Initialize the SOLGUARD token with fair launch parameters
    /// NO pre-mine, NO team allocation, NO liquidity control
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let token_state = &mut ctx.accounts.token_state;
        token_state.authority = ctx.accounts.authority.key();
        token_state.total_supply = 0;
        token_state.total_burned = 0;
        token_state.bonding_curve_k = 1_000_000; // Curve steepness
        token_state.base_price = 1; // Starts at 0.000000001 SOL
        token_state.launch_timestamp = Clock::get()?.unix_timestamp;
        token_state.is_active = true;

        msg!("SOLGUARD Token initialized - Fair Launch begins!");
        msg!("Starting price: 0.000000001 SOL");
        msg!("No pre-mine | No team tokens | No control");

        Ok(())
    }

    /// Buy tokens using bonding curve
    /// Price increases with supply: Price = k * (supply)^2
    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        // Limit constants (in lamports - 0.001 SOL to 100 SOL)
        const MIN_BUY_LAMPORTS: u64 = 1_000_000;
        const MAX_BUY_LAMPORTS: u64 = 100_000_000_000;

        let token_state_info = ctx.accounts.token_state.to_account_info();

        let token_state = &mut ctx.accounts.token_state;

        require!(token_state.is_active, ErrorCode::TokenNotActive);
        require!(sol_amount >= MIN_BUY_LAMPORTS, ErrorCode::BuyTooSmall);
        require!(sol_amount <= MAX_BUY_LAMPORTS, ErrorCode::BuyTooLarge);

        // Calculate tokens to mint based on bonding curve
        let tokens_to_mint = calculate_tokens_for_sol(
            sol_amount,
            token_state.total_supply,
            token_state.bonding_curve_k,
        );

        // Transfer SOL to treasury (burned or used for protocol)
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, sol_amount)?;

        // Mint tokens to buyer
        let seeds = &[b"token_state".as_ref(), &[ctx.bumps.token_state]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: token_state_info,
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, tokens_to_mint)?;

        token_state.total_supply += tokens_to_mint;

        let current_price =
            calculate_current_price(token_state.total_supply, token_state.bonding_curve_k);

        msg!("Bought {} SOLGUARD for {} SOL", tokens_to_mint, sol_amount);
        emit!(TokensPurchased {
            buyer: ctx.accounts.buyer.key(),
            sol_amount,
            tokens_received: tokens_to_mint,
            new_price: current_price,
        });

        Ok(())
    }

    /// Sell tokens back to bonding curve
    /// Price decreases with supply reduction
    pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
        // Limit constants (in tokens - 100 to 1B tokens)
        const MIN_SELL_TOKENS: u64 = 100;
        const MAX_SELL_TOKENS: u64 = 1_000_000_000;

        let token_state = &mut ctx.accounts.token_state;

        require!(token_state.is_active, ErrorCode::TokenNotActive);
        require!(token_amount >= MIN_SELL_TOKENS, ErrorCode::SellTooSmall);
        require!(token_amount <= MAX_SELL_TOKENS, ErrorCode::SellTooLarge);
        require!(
            token_amount <= token_state.total_supply / 10,
            ErrorCode::SellExceedsLimit
        );

        // Calculate SOL to return based on bonding curve
        let sol_to_return = calculate_sol_for_tokens(
            token_amount,
            token_state.total_supply,
            token_state.bonding_curve_k,
        );

        // Burn tokens
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, token_amount)?;

        // Return SOL to seller
        **ctx
            .accounts
            .treasury
            .to_account_info()
            .try_borrow_mut_lamports()? -= sol_to_return;
        **ctx
            .accounts
            .seller
            .to_account_info()
            .try_borrow_mut_lamports()? += sol_to_return;

        token_state.total_supply -= token_amount;
        token_state.total_burned += token_amount;

        msg!("Sold {} SOLGUARD for {} SOL", token_amount, sol_to_return);
        emit!(TokensSold {
            seller: ctx.accounts.seller.key(),
            tokens_sold: token_amount,
            sol_received: sol_to_return,
            new_price: calculate_current_price(
                token_state.total_supply,
                token_state.bonding_curve_k
            ),
        });

        Ok(())
    }

    /// Stake tokens for governance voting power
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;

        require!(amount > 0, ErrorCode::InvalidAmount);

        // Transfer tokens to stake account
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.stake_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        stake_account.user = ctx.accounts.user.key();
        stake_account.staked_amount += amount;
        stake_account.stake_timestamp = Clock::get()?.unix_timestamp;
        stake_account.voting_power = calculate_voting_power(amount);

        msg!("Staked {} SOLGUARD tokens", amount);
        Ok(())
    }

    /// Unstake tokens (with time lock)
    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            amount <= stake_account.staked_amount,
            ErrorCode::InsufficientStake
        );

        // 7-day time lock
        let time_locked = clock.unix_timestamp - stake_account.stake_timestamp;
        require!(time_locked >= 604800, ErrorCode::StillLocked);

        // Transfer tokens back
        let seeds = &[b"stake_vault".as_ref(), &[ctx.bumps.stake_vault]];
        let signer = &[&seeds[..]];

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.stake_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.stake_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount)?;

        stake_account.staked_amount -= amount;
        stake_account.voting_power = calculate_voting_power(stake_account.staked_amount);

        msg!("Unstaked {} SOLGUARD tokens", amount);
        Ok(())
    }

    /// Burn tokens to reduce supply (deflationary mechanism)
    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let token_state = &mut ctx.accounts.token_state;

        require!(amount > 0, ErrorCode::InvalidAmount);

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        token_state.total_supply -= amount;
        token_state.total_burned += amount;

        msg!("Burned {} SOLGUARD tokens - Deflationary!", amount);
        emit!(TokensBurned {
            burner: ctx.accounts.user.key(),
            amount,
            new_supply: token_state.total_supply,
        });

        Ok(())
    }
}

// Helper Functions

fn calculate_tokens_for_sol(sol_amount: u64, current_supply: u64, k: u64) -> u64 {
    // Simplified bonding curve: tokens = sqrt(sol * k + supply^2) - supply
    // Use checked arithmetic to prevent overflow
    let sol_scaled = (sol_amount as u128).saturating_mul(k as u128);
    let supply_squared = (current_supply as u128).saturating_mul(current_supply as u128);
    let new_supply_squared = sol_scaled.saturating_add(supply_squared);
    let new_supply = integer_sqrt(new_supply_squared);
    new_supply.saturating_sub(current_supply)
}

fn calculate_sol_for_tokens(token_amount: u64, current_supply: u64, k: u64) -> u64 {
    // Inverse bonding curve
    let new_supply = current_supply.saturating_sub(token_amount);
    let current_squared = (current_supply as u128).saturating_mul(current_supply as u128);
    let new_squared = (new_supply as u128).saturating_mul(new_supply as u128);
    let diff = current_squared.saturating_sub(new_squared);
    let sol_amount = diff.saturating_div(k as u128);
    sol_amount as u64
}

fn calculate_current_price(supply: u64, k: u64) -> u64 {
    // Price = k * supply
    (supply as u128)
        .saturating_mul(k as u128)
        .saturating_div(1_000_000_000) as u64
}

fn integer_sqrt(n: u128) -> u64 {
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x as u64
}

fn calculate_voting_power(staked_amount: u64) -> u64 {
    // 1 token = 1 vote
    staked_amount
}

// Account Structures

#[account]
pub struct TokenState {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub total_burned: u64,
    pub bonding_curve_k: u64,
    pub base_price: u64,
    pub launch_timestamp: i64,
    pub is_active: bool,
}

#[account]
pub struct StakeAccount {
    pub user: Pubkey,
    pub staked_amount: u64,
    pub stake_timestamp: i64,
    pub voting_power: u64,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"token_state"],
        bump
    )]
    pub token_state: Account<'info, TokenState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut, seeds = [b"token_state"], bump)]
    pub token_state: Account<'info, TokenState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Treasury account for SOL
    pub treasury: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellTokens<'info> {
    #[account(mut, seeds = [b"token_state"], bump)]
    pub token_state: Account<'info, TokenState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Treasury account
    pub treasury: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"stake_vault"], bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(mut, seeds = [b"stake", user.key().as_ref()], bump)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"stake_vault"], bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut, seeds = [b"token_state"], bump)]
    pub token_state: Account<'info, TokenState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Events

#[event]
pub struct TokensPurchased {
    pub buyer: Pubkey,
    pub sol_amount: u64,
    pub tokens_received: u64,
    pub new_price: u64,
}

#[event]
pub struct TokensSold {
    pub seller: Pubkey,
    pub tokens_sold: u64,
    pub sol_received: u64,
    pub new_price: u64,
}

#[event]
pub struct TokensBurned {
    pub burner: Pubkey,
    pub amount: u64,
    pub new_supply: u64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Token is not active")]
    TokenNotActive,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient stake")]
    InsufficientStake,
    #[msg("Tokens still locked")]
    StillLocked,
    #[msg("Buy amount too small (min 0.001 SOL)")]
    BuyTooSmall,
    #[msg("Buy amount too large (max 100 SOL)")]
    BuyTooLarge,
    #[msg("Sell amount too small")]
    SellTooSmall,
    #[msg("Sell amount too large")]
    SellTooLarge,
    #[msg("Sell exceeds 10% of total supply")]
    SellExceedsLimit,
}
