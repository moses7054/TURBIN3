use anchor_lang::prelude::*;

/// Temporary account for tracking project-related transactions.
#[account]
pub struct TempTransactionAccount {
    pub project_account_pubkey: Pubkey, // Project's public key (32 bytes)
    pub ngo_signer_pubkey: Pubkey,
    pub ngo_account_pubkey: Pubkey, // NGO's public key (32 bytes)
    pub submitted_requirements_hash: [u8; 32], // Hash of submitted requirements (32 bytes)
    pub status: TempTransactionAccountStatus, // Transaction status (1 byte)
    pub temp_bump: u8,
}

/// Defines the fixed storage space for on-chain allocation.
impl Space for TempTransactionAccount {
    const INIT_SPACE: usize = 8  // Discriminator
    + 32  // Project account
    + 32  // NGO signer pubkey
    + 32  // ngo account pubkey
    + 32  // Requirements hash
    + 1 // Status (enum, 1 byte)
    + 1; // bump
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Clone, Debug)]
pub enum TempTransactionAccountStatus {
    Processing,
    Accepted,
    Rejected,
}
