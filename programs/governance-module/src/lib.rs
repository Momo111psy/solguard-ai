use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("GoVeRnAnCe1111111111111111111111111111111111");

#[program]
pub mod governance_module {
    use super::*;

    /// Initialize governance
    pub fn initialize(
        ctx: Context<Initialize>,
        voting_period: i64,
        quorum_percentage: u8,
    ) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        governance.authority = ctx.accounts.authority.key();
        governance.voting_period = voting_period;
        governance.quorum_percentage = quorum_percentage;
        governance.total_proposals = 0;
        governance.executed_proposals = 0;
        
        msg!("Governance initialized");
        Ok(())
    }

    /// Create a proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
        execution_data: Vec<u8>,
    ) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let proposal = &mut ctx.accounts.proposal;
        let proposer_stake = &ctx.accounts.proposer_stake;
        
        require!(proposer_stake.amount >= 10000, ErrorCode::InsufficientStake);
        
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.execution_data = execution_data;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.voting_ends_at = proposal.created_at + governance.voting_period;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.status = ProposalStatus::Active;
        
        governance.total_proposals += 1;
        
        msg!("Proposal created: {}", proposal.title);
        emit!(ProposalCreated {
            proposal_id: proposal.key(),
            proposer: ctx.accounts.proposer.key(),
            timestamp: proposal.created_at,
        });
        
        Ok(())
    }

    /// Vote on a proposal
    pub fn vote(
        ctx: Context<Vote>,
        support: bool,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let voter_stake = &ctx.accounts.voter_stake;
        let vote_record = &mut ctx.accounts.vote_record;
        let clock = Clock::get()?;
        
        require!(
            clock.unix_timestamp <= proposal.voting_ends_at,
            ErrorCode::VotingPeriodEnded
        );
        require!(
            proposal.status == ProposalStatus::Active,
            ErrorCode::ProposalNotActive
        );
        
        let vote_weight = voter_stake.amount / 1000; // 1 vote per 1000 tokens
        
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.proposal = proposal.key();
        vote_record.support = support;
        vote_record.weight = vote_weight;
        vote_record.timestamp = clock.unix_timestamp;
        
        if support {
            proposal.votes_for += vote_weight;
        } else {
            proposal.votes_against += vote_weight;
        }
        
        msg!("Vote cast: {} with weight {}", if support { "For" } else { "Against" }, vote_weight);
        Ok(())
    }

    /// Execute a passed proposal
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;
        
        require!(
            clock.unix_timestamp > proposal.voting_ends_at,
            ErrorCode::VotingPeriodNotEnded
        );
        require!(
            proposal.status == ProposalStatus::Active,
            ErrorCode::ProposalNotActive
        );
        
        let total_votes = proposal.votes_for + proposal.votes_against;
        let quorum_met = total_votes >= 100000; // Simplified quorum check
        let passed = proposal.votes_for > proposal.votes_against;
        
        if quorum_met && passed {
            proposal.status = ProposalStatus::Executed;
            governance.executed_proposals += 1;
            
            // Execute based on proposal type
            match proposal.proposal_type {
                ProposalType::ParameterChange => {
                    msg!("Executing parameter change");
                    // Would update relevant parameters
                }
                ProposalType::TreasurySpend => {
                    msg!("Executing treasury spend");
                    // Would transfer funds
                }
                ProposalType::UpgradeContract => {
                    msg!("Executing contract upgrade");
                    // Would trigger upgrade process
                }
                ProposalType::EmergencyAction => {
                    msg!("Executing emergency action");
                    // Would execute emergency measures
                }
            }
            
            emit!(ProposalExecuted {
                proposal_id: proposal.key(),
                votes_for: proposal.votes_for,
                votes_against: proposal.votes_against,
                timestamp: clock.unix_timestamp,
            });
        } else {
            proposal.status = ProposalStatus::Rejected;
            msg!("Proposal rejected - Quorum: {}, Passed: {}", quorum_met, passed);
        }
        
        Ok(())
    }

    /// Cancel a proposal (only by proposer before voting ends)
    pub fn cancel_proposal(ctx: Context<CancelProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;
        
        require!(
            ctx.accounts.proposer.key() == proposal.proposer,
            ErrorCode::Unauthorized
        );
        require!(
            clock.unix_timestamp <= proposal.voting_ends_at,
            ErrorCode::VotingPeriodEnded
        );
        
        proposal.status = ProposalStatus::Cancelled;
        
        msg!("Proposal cancelled");
        Ok(())
    }
}

// Account Structures

#[account]
pub struct Governance {
    pub authority: Pubkey,
    pub voting_period: i64,
    pub quorum_percentage: u8,
    pub total_proposals: u64,
    pub executed_proposals: u64,
}

#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub execution_data: Vec<u8>,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
}

#[account]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub support: bool,
    pub weight: u64,
    pub timestamp: i64,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 1 + 8 + 8,
        seeds = [b"governance"],
        bump
    )]
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 128 + 512 + 1 + 1024 + 8 + 8 + 8 + 8 + 1,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub proposer_stake: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 32 + 1 + 8 + 8,
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub voter_stake: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub proposer: Signer<'info>,
}

// Enums

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Executed,
    Rejected,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ProposalType {
    ParameterChange,
    TreasurySpend,
    UpgradeContract,
    EmergencyAction,
}

// Events

#[event]
pub struct ProposalCreated {
    pub proposal_id: Pubkey,
    pub proposer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalExecuted {
    pub proposal_id: Pubkey,
    pub votes_for: u64,
    pub votes_against: u64,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient stake to create proposal")]
    InsufficientStake,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Proposal is not active")]
    ProposalNotActive,
    #[msg("Voting period has not ended yet")]
    VotingPeriodNotEnded,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
}
