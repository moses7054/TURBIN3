use anchor_lang::prelude::*;

#[account]
pub struct ProjectCompletionDetails {
    project_pubkey: Pubkey, // Pubkey of project it is linked to.
    ngo_pubkey: Pubkey,     // Pubkey of ngo account completing it.
    merkel_root: [u8; 32],  // root of the tree which has leves made up of all the transactions
    completion_bump: u8,
}

impl Space for ProjectCompletionDetails {
    const INIT_SPACE: usize = 8  // Anchor's discriminator (always 8 bytes)
    + 32 // project_pubkey
    + 32 // ngo_pubkey
    + 32 // merkel_root
    + 1; // bump
}
