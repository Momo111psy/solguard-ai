use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("HQqeQGBF1uQSEuhvBfnomn2jLidndFad9AhTt5siamNp");

#[program]
pub mod security_oracle {
    use super::*;

    /// Initialize the Security Oracle with AI model parameters
    pub fn initialize(
        ctx: Context<Initialize>,
        model_version: String,
        threshold_score: u8,
        update_authority: Pubkey,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.authority = ctx.accounts.authority.key();
        oracle.model_version = model_version;
        oracle.threshold_score = threshold_score;
        oracle.update_authority = update_authority;
        oracle.total_scans = 0;
        oracle.threats_detected = 0;
        oracle.last_update = Clock::get()?.unix_timestamp;
        oracle.is_active = true;

        msg!(
            "Security Oracle initialized with model version: {}",
            oracle.model_version
        );
        Ok(())
    }

    /// Submit a program for security analysis
    pub fn analyze_program(
        ctx: Context<AnalyzeProgram>,
        program_address: Pubkey,
        code_hash: [u8; 32],
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        let analysis = &mut ctx.accounts.analysis;

        require!(oracle.is_active, ErrorCode::OracleInactive);

        analysis.program_address = program_address;
        analysis.code_hash = code_hash;
        analysis.timestamp = Clock::get()?.unix_timestamp;
        analysis.status = AnalysisStatus::Pending;
        analysis.security_score = 0;
        analysis.vulnerability_count = 0;
        analysis.analyzer = ctx.accounts.analyzer.key();

        oracle.total_scans += 1;

        msg!("Program analysis initiated for: {}", program_address);
        emit!(AnalysisRequested {
            program_address,
            analyzer: ctx.accounts.analyzer.key(),
            timestamp: analysis.timestamp,
        });

        Ok(())
    }

    /// Update analysis results from AI model (called by authorized oracle)
    pub fn update_analysis(
        ctx: Context<UpdateAnalysis>,
        security_score: u8,
        vulnerability_count: u16,
        _vulnerabilities: Vec<Vulnerability>,
        _recommendations: String,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        let analysis = &mut ctx.accounts.analysis;

        require!(
            ctx.accounts.update_authority.key() == oracle.update_authority,
            ErrorCode::UnauthorizedUpdate
        );
        require!(
            analysis.status == AnalysisStatus::Pending,
            ErrorCode::AnalysisAlreadyCompleted
        );

        analysis.security_score = security_score;
        analysis.vulnerability_count = vulnerability_count;
        analysis.status = if security_score >= oracle.threshold_score {
            AnalysisStatus::Safe
        } else {
            AnalysisStatus::Unsafe
        };
        analysis.completed_at = Clock::get()?.unix_timestamp;

        if analysis.status == AnalysisStatus::Unsafe {
            oracle.threats_detected += 1;
        }

        oracle.last_update = Clock::get()?.unix_timestamp;

        let final_status = analysis.status.clone();

        msg!(
            "Analysis completed - Score: {}, Vulnerabilities: {}",
            security_score,
            vulnerability_count
        );
        emit!(AnalysisCompleted {
            program_address: analysis.program_address,
            security_score,
            status: final_status,
            timestamp: analysis.completed_at,
        });

        Ok(())
    }

    /// Report a security incident (community-driven)
    pub fn report_incident(
        ctx: Context<ReportIncident>,
        program_address: Pubkey,
        incident_type: IncidentType,
        description: String,
        severity: u8,
    ) -> Result<()> {
        let incident = &mut ctx.accounts.incident;
        let reporter_stake = &ctx.accounts.reporter_stake;

        require!(reporter_stake.amount >= 1000, ErrorCode::InsufficientStake);

        incident.program_address = program_address;
        incident.reporter = ctx.accounts.reporter.key();
        incident.incident_type = incident_type;
        incident.description = description;
        incident.severity = severity;
        incident.timestamp = Clock::get()?.unix_timestamp;
        incident.verified = false;
        incident.votes_for = 0;
        incident.votes_against = 0;

        msg!("Security incident reported for: {}", program_address);
        emit!(IncidentReported {
            program_address,
            reporter: ctx.accounts.reporter.key(),
            severity,
            timestamp: incident.timestamp,
        });

        Ok(())
    }

    /// Vote on reported incident (stake-weighted)
    pub fn vote_incident(ctx: Context<VoteIncident>, support: bool) -> Result<()> {
        let incident = &mut ctx.accounts.incident;
        let voter_stake = &ctx.accounts.voter_stake;

        require!(!incident.verified, ErrorCode::IncidentAlreadyVerified);

        let vote_weight = voter_stake.amount / 100; // 1 vote per 100 tokens

        if support {
            incident.votes_for += vote_weight;
        } else {
            incident.votes_against += vote_weight;
        }

        // Auto-verify if threshold reached
        if incident.votes_for >= 10000 {
            incident.verified = true;
            msg!("Incident verified by community consensus");
        }

        Ok(())
    }

    /// Update AI model parameters (governance)
    pub fn update_model(
        ctx: Context<UpdateModel>,
        new_model_version: String,
        new_threshold: u8,
    ) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;

        require!(
            ctx.accounts.authority.key() == oracle.authority,
            ErrorCode::Unauthorized
        );

        oracle.model_version = new_model_version.clone();
        oracle.threshold_score = new_threshold;
        oracle.last_update = Clock::get()?.unix_timestamp;

        msg!("Model updated to version: {}", new_model_version);
        Ok(())
    }

    /// Emergency pause (security measure)
    pub fn pause_oracle(ctx: Context<PauseOracle>) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;

        require!(
            ctx.accounts.authority.key() == oracle.authority,
            ErrorCode::Unauthorized
        );

        oracle.is_active = false;
        msg!("Security Oracle paused");
        Ok(())
    }

    /// Resume operations
    pub fn resume_oracle(ctx: Context<ResumeOracle>) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;

        require!(
            ctx.accounts.authority.key() == oracle.authority,
            ErrorCode::Unauthorized
        );

        oracle.is_active = true;
        msg!("Security Oracle resumed");
        Ok(())
    }
}

// Account Structures

#[account]
pub struct SecurityOracle {
    pub authority: Pubkey,
    pub model_version: String,
    pub threshold_score: u8,
    pub update_authority: Pubkey,
    pub total_scans: u64,
    pub threats_detected: u64,
    pub last_update: i64,
    pub is_active: bool,
}

#[account]
pub struct ProgramAnalysis {
    pub program_address: Pubkey,
    pub code_hash: [u8; 32],
    pub timestamp: i64,
    pub completed_at: i64,
    pub status: AnalysisStatus,
    pub security_score: u8,
    pub vulnerability_count: u16,
    pub analyzer: Pubkey,
}

#[account]
pub struct SecurityIncident {
    pub program_address: Pubkey,
    pub reporter: Pubkey,
    pub incident_type: IncidentType,
    pub description: String,
    pub severity: u8,
    pub timestamp: i64,
    pub verified: bool,
    pub votes_for: u64,
    pub votes_against: u64,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 64 + 1 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle: Account<'info, SecurityOracle>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnalyzeProgram<'info> {
    #[account(mut)]
    pub oracle: Account<'info, SecurityOracle>,
    #[account(
        init,
        payer = analyzer,
        space = 8 + 32 + 32 + 8 + 8 + 1 + 1 + 2 + 32,
        seeds = [b"analysis", oracle.key().as_ref(), analyzer.key().as_ref()],
        bump
    )]
    pub analysis: Account<'info, ProgramAnalysis>,
    #[account(mut)]
    pub analyzer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAnalysis<'info> {
    #[account(mut)]
    pub oracle: Account<'info, SecurityOracle>,
    #[account(mut)]
    pub analysis: Account<'info, ProgramAnalysis>,
    pub update_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReportIncident<'info> {
    #[account(
        init,
        payer = reporter,
        space = 8 + 32 + 32 + 1 + 256 + 1 + 8 + 1 + 8 + 8,
    )]
    pub incident: Account<'info, SecurityIncident>,
    #[account(mut)]
    pub reporter: Signer<'info>,
    pub reporter_stake: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteIncident<'info> {
    #[account(mut)]
    pub incident: Account<'info, SecurityIncident>,
    pub voter: Signer<'info>,
    pub voter_stake: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct UpdateModel<'info> {
    #[account(mut)]
    pub oracle: Account<'info, SecurityOracle>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct PauseOracle<'info> {
    #[account(mut)]
    pub oracle: Account<'info, SecurityOracle>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResumeOracle<'info> {
    #[account(mut)]
    pub oracle: Account<'info, SecurityOracle>,
    pub authority: Signer<'info>,
}

// Enums and Structs

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AnalysisStatus {
    Pending,
    Safe,
    Unsafe,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum IncidentType {
    Reentrancy,
    IntegerOverflow,
    UnauthorizedAccess,
    LogicError,
    FrontRunning,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Vulnerability {
    pub vuln_type: String,
    pub severity: u8,
    pub location: String,
    pub description: String,
}

// Events

#[event]
pub struct AnalysisRequested {
    pub program_address: Pubkey,
    pub analyzer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AnalysisCompleted {
    pub program_address: Pubkey,
    pub security_score: u8,
    pub status: AnalysisStatus,
    pub timestamp: i64,
}

#[event]
pub struct IncidentReported {
    pub program_address: Pubkey,
    pub reporter: Pubkey,
    pub severity: u8,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Oracle is currently inactive")]
    OracleInactive,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Unauthorized to update analysis")]
    UnauthorizedUpdate,
    #[msg("Analysis already completed")]
    AnalysisAlreadyCompleted,
    #[msg("Insufficient stake to report incident")]
    InsufficientStake,
    #[msg("Incident already verified")]
    IncidentAlreadyVerified,
}
