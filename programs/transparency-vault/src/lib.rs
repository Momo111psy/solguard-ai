use anchor_lang::prelude::*;

declare_id!("TrAnSpArEnCy1111111111111111111111111111111");

#[program]
pub mod transparency_vault {
    use super::*;

    /// Initialize the Transparency Vault
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.total_programs = 0;
        vault.verified_programs = 0;
        vault.last_update = Clock::get()?.unix_timestamp;
        
        msg!("Transparency Vault initialized");
        Ok(())
    }

    /// Register a program's IDL and metadata
    pub fn register_program(
        ctx: Context<RegisterProgram>,
        program_id: Pubkey,
        idl_hash: [u8; 32],
        idl_uri: String,
        source_code_uri: String,
        build_hash: [u8; 32],
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let program_record = &mut ctx.accounts.program_record;
        
        program_record.program_id = program_id;
        program_record.idl_hash = idl_hash;
        program_record.idl_uri = idl_uri;
        program_record.source_code_uri = source_code_uri;
        program_record.build_hash = build_hash;
        program_record.deployer = ctx.accounts.deployer.key();
        program_record.registration_time = Clock::get()?.unix_timestamp;
        program_record.is_verified = false;
        program_record.verification_count = 0;
        program_record.last_update = Clock::get()?.unix_timestamp;
        
        vault.total_programs += 1;
        
        msg!("Program registered: {}", program_id);
        emit!(ProgramRegistered {
            program_id,
            deployer: ctx.accounts.deployer.key(),
            timestamp: program_record.registration_time,
        });
        
        Ok(())
    }

    /// Verify a program's IDL (community verification)
    pub fn verify_program(
        ctx: Context<VerifyProgram>,
        verification_proof: [u8; 32],
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let program_record = &mut ctx.accounts.program_record;
        let verification = &mut ctx.accounts.verification;
        
        verification.program_id = program_record.program_id;
        verification.verifier = ctx.accounts.verifier.key();
        verification.verification_proof = verification_proof;
        verification.timestamp = Clock::get()?.unix_timestamp;
        verification.is_valid = true;
        
        program_record.verification_count += 1;
        
        // Auto-verify after 3 independent verifications
        if program_record.verification_count >= 3 && !program_record.is_verified {
            program_record.is_verified = true;
            vault.verified_programs += 1;
            
            msg!("Program verified: {}", program_record.program_id);
            emit!(ProgramVerified {
                program_id: program_record.program_id,
                verifications: program_record.verification_count,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }
        
        Ok(())
    }

    /// Update program metadata
    pub fn update_program(
        ctx: Context<UpdateProgram>,
        new_idl_hash: [u8; 32],
        new_idl_uri: String,
        new_build_hash: [u8; 32],
    ) -> Result<()> {
        let program_record = &mut ctx.accounts.program_record;
        
        require!(
            ctx.accounts.deployer.key() == program_record.deployer,
            ErrorCode::Unauthorized
        );
        
        program_record.idl_hash = new_idl_hash;
        program_record.idl_uri = new_idl_uri;
        program_record.build_hash = new_build_hash;
        program_record.is_verified = false; // Reset verification
        program_record.verification_count = 0;
        program_record.last_update = Clock::get()?.unix_timestamp;
        
        msg!("Program updated: {}", program_record.program_id);
        Ok(())
    }

    /// Add audit report
    pub fn add_audit_report(
        ctx: Context<AddAuditReport>,
        auditor: String,
        report_uri: String,
        findings_count: u16,
        severity_score: u8,
    ) -> Result<()> {
        let program_record = &mut ctx.accounts.program_record;
        let audit_report = &mut ctx.accounts.audit_report;
        
        audit_report.program_id = program_record.program_id;
        audit_report.auditor = auditor;
        audit_report.report_uri = report_uri;
        audit_report.findings_count = findings_count;
        audit_report.severity_score = severity_score;
        audit_report.audit_date = Clock::get()?.unix_timestamp;
        audit_report.submitted_by = ctx.accounts.submitter.key();
        
        msg!("Audit report added for: {}", program_record.program_id);
        emit!(AuditReportAdded {
            program_id: program_record.program_id,
            findings: findings_count,
            severity: severity_score,
            timestamp: audit_report.audit_date,
        });
        
        Ok(())
    }

    /// Query program transparency score
    pub fn get_transparency_score(ctx: Context<GetTransparencyScore>) -> Result<u8> {
        let program_record = &ctx.accounts.program_record;
        
        let mut score: u8 = 0;
        
        // IDL published: +30 points
        if !program_record.idl_uri.is_empty() {
            score += 30;
        }
        
        // Source code published: +30 points
        if !program_record.source_code_uri.is_empty() {
            score += 30;
        }
        
        // Verified: +20 points
        if program_record.is_verified {
            score += 20;
        }
        
        // Multiple verifications: +10 points
        if program_record.verification_count >= 3 {
            score += 10;
        }
        
        // Has audit: +10 points (would check audit_report existence)
        score += 10;
        
        msg!("Transparency score for {}: {}", program_record.program_id, score);
        Ok(score)
    }
}

// Account Structures

#[account]
pub struct TransparencyVault {
    pub authority: Pubkey,
    pub total_programs: u64,
    pub verified_programs: u64,
    pub last_update: i64,
}

#[account]
pub struct ProgramRecord {
    pub program_id: Pubkey,
    pub idl_hash: [u8; 32],
    pub idl_uri: String,
    pub source_code_uri: String,
    pub build_hash: [u8; 32],
    pub deployer: Pubkey,
    pub registration_time: i64,
    pub is_verified: bool,
    pub verification_count: u16,
    pub last_update: i64,
}

#[account]
pub struct ProgramVerification {
    pub program_id: Pubkey,
    pub verifier: Pubkey,
    pub verification_proof: [u8; 32],
    pub timestamp: i64,
    pub is_valid: bool,
}

#[account]
pub struct AuditReport {
    pub program_id: Pubkey,
    pub auditor: String,
    pub report_uri: String,
    pub findings_count: u16,
    pub severity_score: u8,
    pub audit_date: i64,
    pub submitted_by: Pubkey,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, TransparencyVault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterProgram<'info> {
    #[account(mut)]
    pub vault: Account<'info, TransparencyVault>,
    #[account(
        init,
        payer = deployer,
        space = 8 + 32 + 32 + 256 + 256 + 32 + 32 + 8 + 1 + 2 + 8,
    )]
    pub program_record: Account<'info, ProgramRecord>,
    #[account(mut)]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyProgram<'info> {
    #[account(mut)]
    pub vault: Account<'info, TransparencyVault>,
    #[account(mut)]
    pub program_record: Account<'info, ProgramRecord>,
    #[account(
        init,
        payer = verifier,
        space = 8 + 32 + 32 + 32 + 8 + 1,
    )]
    pub verification: Account<'info, ProgramVerification>,
    #[account(mut)]
    pub verifier: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProgram<'info> {
    #[account(mut)]
    pub program_record: Account<'info, ProgramRecord>,
    pub deployer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddAuditReport<'info> {
    pub program_record: Account<'info, ProgramRecord>,
    #[account(
        init,
        payer = submitter,
        space = 8 + 32 + 64 + 256 + 2 + 1 + 8 + 32,
    )]
    pub audit_report: Account<'info, AuditReport>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetTransparencyScore<'info> {
    pub program_record: Account<'info, ProgramRecord>,
}

// Events

#[event]
pub struct ProgramRegistered {
    pub program_id: Pubkey,
    pub deployer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProgramVerified {
    pub program_id: Pubkey,
    pub verifications: u16,
    pub timestamp: i64,
}

#[event]
pub struct AuditReportAdded {
    pub program_id: Pubkey,
    pub findings: u16,
    pub severity: u8,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
}
