use crate::error::ErrorCode;
use crate::{validate, Size, Vault, VaultDepositor};
use anchor_lang::prelude::*;

pub fn initialize_vault_depositor(ctx: Context<InitializeVaultDepositor>) -> Result<()> {
    let mut vault_depositor = ctx.accounts.vault_depositor.load_init()?;
    vault_depositor.vault = ctx.accounts.vault.key();
    vault_depositor.pubkey = ctx.accounts.vault_depositor.key();
    vault_depositor.authority = *ctx.accounts.authority.key;

    let vault = ctx.accounts.vault.load()?;
    if vault.permissioned {
        validate!(
            vault.authority == *ctx.accounts.payer.key,
            ErrorCode::PermissionedVault,
            "Vault depositor can only be created by vault authority"
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVaultDepositor<'info> {
    pub vault: AccountLoader<'info, Vault>,
    #[account(
        init,
        seeds = [b"vault_depositor", vault.key().as_ref()],
        space = Vault::SIZE,
        bump,
        payer = payer
    )]
    pub vault_depositor: AccountLoader<'info, VaultDepositor>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
