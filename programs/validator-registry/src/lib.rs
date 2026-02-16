use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("FyDxHtfbHxm61nV2DMGXdxCrmkNHt6GacuGbXJDbr8Ap");

#[program]
pub mod validator_registry {
    use super::*;

    /// Initialize the Validator Registry
    pub fn initialize(
        ctx: Context<Initialize>,
        min_stake_requirement: u64,
        reward_rate: u64,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.total_validators = 0;
        registry.active_validators = 0;
        registry.min_stake_requirement = min_stake_requirement;
        registry.reward_rate = reward_rate;
        registry.total_rewards_distributed = 0;
        registry.nakamoto_coefficient = 0;
        registry.last_update = Clock::get()?.unix_timestamp;

        msg!("Validator Registry initialized");
        Ok(())
    }

    /// Register a new validator
    pub fn register_validator(
        ctx: Context<RegisterValidator>,
        validator_pubkey: Pubkey,
        commission_rate: u8,
        metadata_uri: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let validator = &mut ctx.accounts.validator;
        let stake_account = &ctx.accounts.stake_account;

        require!(
            stake_account.amount >= registry.min_stake_requirement,
            ErrorCode::InsufficientStake
        );
        require!(commission_rate <= 100, ErrorCode::InvalidCommissionRate);

        validator.validator_pubkey = validator_pubkey;
        validator.operator = ctx.accounts.operator.key();
        validator.commission_rate = commission_rate;
        validator.total_stake = stake_account.amount;
        validator.performance_score = 100; // Start at 100%
        validator.uptime_percentage = 0;
        validator.blocks_produced = 0;
        validator.last_reward_claim = Clock::get()?.unix_timestamp;
        validator.registration_time = Clock::get()?.unix_timestamp;
        validator.is_active = true;
        validator.metadata_uri = metadata_uri;
        validator.health_score = 100;

        registry.total_validators += 1;
        registry.active_validators += 1;

        msg!("Validator registered: {}", validator_pubkey);
        emit!(ValidatorRegistered {
            validator_pubkey,
            operator: ctx.accounts.operator.key(),
            stake: stake_account.amount,
            timestamp: validator.registration_time,
        });

        Ok(())
    }

    /// Update validator performance metrics (called by monitoring system)
    pub fn update_performance(
        ctx: Context<UpdatePerformance>,
        blocks_produced: u64,
        uptime_percentage: u8,
    ) -> Result<()> {
        let validator = &mut ctx.accounts.validator;

        require!(validator.is_active, ErrorCode::ValidatorInactive);
        require!(uptime_percentage <= 100, ErrorCode::InvalidUptime);

        validator.blocks_produced += blocks_produced;
        validator.uptime_percentage = uptime_percentage;

        // Calculate performance score (weighted average)
        let uptime_score = uptime_percentage as u16;
        let production_score = if blocks_produced > 0 { 100 } else { 50 };
        validator.performance_score = ((uptime_score + production_score) / 2) as u8;

        // Calculate health score
        validator.health_score = calculate_health_score(
            validator.performance_score,
            validator.total_stake,
            validator.uptime_percentage,
        );

        msg!(
            "Performance updated - Score: {}, Health: {}",
            validator.performance_score,
            validator.health_score
        );

        Ok(())
    }

    /// Claim validator rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let clock = Clock::get()?;

        let registry_info = ctx.accounts.registry.to_account_info();
        let validator = &mut ctx.accounts.validator;

        require!(validator.is_active, ErrorCode::ValidatorInactive);

        let time_elapsed = clock.unix_timestamp - validator.last_reward_claim;
        require!(time_elapsed >= 86400, ErrorCode::ClaimTooEarly); // 24 hours

        // Calculate rewards based on stake, performance, and time
        let base_reward = (validator.total_stake * ctx.accounts.registry.reward_rate) / 10000;
        let performance_multiplier = validator.performance_score as u64;
        let health_multiplier = validator.health_score as u64;
        let reward = (base_reward * performance_multiplier * health_multiplier) / 10000;

        // Transfer rewards
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.validator_rewards.to_account_info(),
            authority: registry_info,
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, reward)?;

        let registry = &mut ctx.accounts.registry;
        validator.last_reward_claim = clock.unix_timestamp;
        registry.total_rewards_distributed += reward;

        msg!("Rewards claimed: {} tokens", reward);
        emit!(RewardsClaimed {
            validator: validator.validator_pubkey,
            amount: reward,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Delegate stake to validator
    pub fn delegate_stake(ctx: Context<DelegateStake>, amount: u64) -> Result<()> {
        let validator = &mut ctx.accounts.validator;
        let delegation = &mut ctx.accounts.delegation;

        require!(validator.is_active, ErrorCode::ValidatorInactive);
        require!(amount > 0, ErrorCode::InvalidAmount);

        // Transfer tokens to stake account
        let cpi_accounts = Transfer {
            from: ctx.accounts.delegator_token.to_account_info(),
            to: ctx.accounts.validator_stake.to_account_info(),
            authority: ctx.accounts.delegator.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        delegation.delegator = ctx.accounts.delegator.key();
        delegation.validator = validator.validator_pubkey;
        delegation.amount = amount;
        delegation.delegation_time = Clock::get()?.unix_timestamp;
        delegation.is_active = true;

        validator.total_stake += amount;

        msg!(
            "Stake delegated: {} tokens to {}",
            amount,
            validator.validator_pubkey
        );
        Ok(())
    }

    /// Undelegate stake from validator
    pub fn undelegate_stake(ctx: Context<UndelegateStake>) -> Result<()> {
        let clock = Clock::get()?;

        let validator_info = ctx.accounts.validator.to_account_info();
        let validator = &mut ctx.accounts.validator;
        let delegation = &mut ctx.accounts.delegation;

        require!(delegation.is_active, ErrorCode::DelegationNotActive);
        require!(
            clock.unix_timestamp - delegation.delegation_time >= 604800, // 7 days
            ErrorCode::UndelegationTooEarly
        );

        let amount = delegation.amount;

        // Transfer tokens back
        let cpi_accounts = Transfer {
            from: ctx.accounts.validator_stake.to_account_info(),
            to: ctx.accounts.delegator_token.to_account_info(),
            authority: validator_info,
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        validator.total_stake -= amount;
        delegation.is_active = false;

        msg!("Stake undelegated: {} tokens", amount);
        Ok(())
    }

    /// Calculate and update Nakamoto Coefficient
    pub fn update_nakamoto_coefficient(ctx: Context<UpdateNakamoto>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;

        // This would be called periodically by a crank/keeper
        // Simplified calculation for demonstration
        let coefficient = calculate_nakamoto_coefficient(registry.active_validators);
        registry.nakamoto_coefficient = coefficient;
        registry.last_update = Clock::get()?.unix_timestamp;

        msg!("Nakamoto Coefficient updated: {}", coefficient);
        emit!(NakamotoUpdated {
            coefficient,
            active_validators: registry.active_validators,
            timestamp: registry.last_update,
        });

        Ok(())
    }

    /// Deactivate underperforming validator
    pub fn deactivate_validator(ctx: Context<DeactivateValidator>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let validator = &mut ctx.accounts.validator;

        require!(
            ctx.accounts.authority.key() == registry.authority
                || ctx.accounts.authority.key() == validator.operator,
            ErrorCode::Unauthorized
        );

        validator.is_active = false;
        registry.active_validators -= 1;

        msg!("Validator deactivated: {}", validator.validator_pubkey);
        Ok(())
    }

    /// Reactivate validator
    pub fn reactivate_validator(ctx: Context<ReactivateValidator>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let validator = &mut ctx.accounts.validator;

        require!(
            ctx.accounts.authority.key() == validator.operator,
            ErrorCode::Unauthorized
        );
        require!(!validator.is_active, ErrorCode::ValidatorAlreadyActive);

        validator.is_active = true;
        validator.health_score = 100; // Reset health score
        registry.active_validators += 1;

        msg!("Validator reactivated: {}", validator.validator_pubkey);
        Ok(())
    }
}

// Helper Functions

fn calculate_health_score(performance: u8, stake: u64, uptime: u8) -> u8 {
    let performance_weight = (performance as u32 * 40) / 100;
    let stake_weight = if stake > 100_000_000_000 {
        30
    } else {
        (stake / 3_333_333_333) as u32
    };
    let uptime_weight = (uptime as u32 * 30) / 100;

    ((performance_weight + stake_weight + uptime_weight) as u8).min(100)
}

fn calculate_nakamoto_coefficient(active_validators: u64) -> u16 {
    // Simplified: In reality, this would analyze stake distribution
    // Higher is better for decentralization
    if active_validators < 10 {
        1
    } else if active_validators < 50 {
        (active_validators / 10) as u16
    } else {
        ((active_validators as f64).sqrt() as u16).max(7)
    }
}

// Account Structures

#[account]
pub struct ValidatorRegistry {
    pub authority: Pubkey,
    pub total_validators: u64,
    pub active_validators: u64,
    pub min_stake_requirement: u64,
    pub reward_rate: u64,
    pub total_rewards_distributed: u64,
    pub nakamoto_coefficient: u16,
    pub last_update: i64,
}

#[account]
pub struct ValidatorInfo {
    pub validator_pubkey: Pubkey,
    pub operator: Pubkey,
    pub commission_rate: u8,
    pub total_stake: u64,
    pub performance_score: u8,
    pub uptime_percentage: u8,
    pub blocks_produced: u64,
    pub last_reward_claim: i64,
    pub registration_time: i64,
    pub is_active: bool,
    pub metadata_uri: String,
    pub health_score: u8,
}

#[account]
pub struct StakeDelegation {
    pub delegator: Pubkey,
    pub validator: Pubkey,
    pub amount: u64,
    pub delegation_time: i64,
    pub is_active: bool,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 2 + 8,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, ValidatorRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterValidator<'info> {
    #[account(mut)]
    pub registry: Account<'info, ValidatorRegistry>,
    #[account(
        init,
        payer = operator,
        space = 8 + 32 + 32 + 1 + 8 + 1 + 1 + 8 + 8 + 8 + 1 + 128 + 1,
    )]
    pub validator: Account<'info, ValidatorInfo>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub stake_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePerformance<'info> {
    #[account(mut, has_one = operator)]
    pub validator: Account<'info, ValidatorInfo>,
    pub operator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub registry: Account<'info, ValidatorRegistry>,
    #[account(mut)]
    pub validator: Account<'info, ValidatorInfo>,
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub validator_rewards: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DelegateStake<'info> {
    #[account(mut)]
    pub validator: Account<'info, ValidatorInfo>,
    #[account(
        init,
        payer = delegator,
        space = 8 + 32 + 32 + 8 + 8 + 1,
    )]
    pub delegation: Account<'info, StakeDelegation>,
    #[account(mut)]
    pub delegator: Signer<'info>,
    #[account(mut)]
    pub delegator_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub validator_stake: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UndelegateStake<'info> {
    #[account(mut)]
    pub validator: Account<'info, ValidatorInfo>,
    #[account(mut)]
    pub delegation: Account<'info, StakeDelegation>,
    pub delegator: Signer<'info>,
    #[account(mut)]
    pub delegator_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub validator_stake: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateNakamoto<'info> {
    #[account(mut)]
    pub registry: Account<'info, ValidatorRegistry>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivateValidator<'info> {
    #[account(mut)]
    pub registry: Account<'info, ValidatorRegistry>,
    #[account(mut)]
    pub validator: Account<'info, ValidatorInfo>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReactivateValidator<'info> {
    #[account(mut)]
    pub registry: Account<'info, ValidatorRegistry>,
    #[account(mut)]
    pub validator: Account<'info, ValidatorInfo>,
    pub authority: Signer<'info>,
}

// Events

#[event]
pub struct ValidatorRegistered {
    pub validator_pubkey: Pubkey,
    pub operator: Pubkey,
    pub stake: u64,
    pub timestamp: i64,
}

#[event]
pub struct RewardsClaimed {
    pub validator: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct NakamotoUpdated {
    pub coefficient: u16,
    pub active_validators: u64,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient stake to register validator")]
    InsufficientStake,
    #[msg("Invalid commission rate (must be 0-100)")]
    InvalidCommissionRate,
    #[msg("Validator is not active")]
    ValidatorInactive,
    #[msg("Claim too early, must wait 24 hours")]
    ClaimTooEarly,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Delegation is not active")]
    DelegationNotActive,
    #[msg("Undelegation too early, must wait 7 days")]
    UndelegationTooEarly,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Validator is already active")]
    ValidatorAlreadyActive,
    #[msg("Invalid uptime percentage (must be 0-100)")]
    InvalidUptime,
}
