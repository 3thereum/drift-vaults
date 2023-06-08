use crate::constraints::{
    is_authority_for_vault_depositor, is_user_for_vault, is_user_stats_for_vault,
};
use crate::{Vault, VaultDepositor};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use drift::cpi::accounts::Withdraw as DriftWithdraw;
use drift::instructions::optional_accounts::{load_maps, AccountMaps};
use drift::math::margin::{
    calculate_margin_requirement_and_total_collateral, MarginRequirementType,
};
use drift::program::Drift;
use drift::state::perp_market_map::{get_writable_perp_market_set, MarketSet};
use drift::state::user::User;

pub fn withdraw<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    amount: u64,
) -> Result<()> {
    let vault = ctx.accounts.vault.load()?;
    let name = vault.name;
    let bump = vault.bump;
    let spot_market_index = vault.spot_market_index;
    drop(vault);

    let signature_seeds = Vault::get_vault_signer_seeds(&name, &bump);
    let signers = &[&signature_seeds[..]];

    let cpi_program = ctx.accounts.drift_program.to_account_info().clone();
    let cpi_accounts = DriftWithdraw {
        state: ctx.accounts.drift_state.clone(),
        user: ctx.accounts.drift_user.to_account_info().clone(),
        user_stats: ctx.accounts.drift_user_stats.clone(),
        authority: ctx.accounts.vault.to_account_info().clone(),
        spot_market_vault: ctx
            .accounts
            .drift_spot_market_vault
            .to_account_info()
            .clone(),
        drift_signer: ctx.accounts.drift_signer.clone(),
        user_token_account: ctx.accounts.vault_token_account.to_account_info().clone(),
        token_program: ctx.accounts.token_program.to_account_info().clone(),
    };
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers)
        .with_remaining_accounts(ctx.remaining_accounts.into());
    drift::cpi::withdraw(cpi_context, spot_market_index, amount, false)?;

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info().clone(),
        to: ctx.accounts.user_token_account.to_account_info().clone(),
        authority: ctx.accounts.vault.to_account_info().clone(),
    };
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers);
    token::transfer(cpi_context, amount)?;

    let clock = &Clock::get()?;

    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    let AccountMaps {
        perp_market_map,
        spot_market_map,
        mut oracle_map,
    } = load_maps(
        remaining_accounts_iter,
        &MarketSet::new(),
        &get_writable_perp_market_set(spot_market_index),
        clock.slot,
        None,
    )?;

    let user = ctx.accounts.drift_user.load()?;

    let (margin_requirement, total_collateral, _, _) =
        calculate_margin_requirement_and_total_collateral(
            &user,
            &perp_market_map,
            MarginRequirementType::Initial,
            &spot_market_map,
            &mut oracle_map,
            None,
        )?;

    msg!("total collateral: {}", total_collateral);
    msg!("margin requirement: {}", margin_requirement);

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub vault: AccountLoader<'info, Vault>,
    #[account(
        mut,
        seeds = [b"vault_depositor", vault.key().as_ref()],
        bump,
        constraint = is_authority_for_vault_depositor(&vault_depositor, &authority)?,
    )]
    pub vault_depositor: AccountLoader<'info, VaultDepositor>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault_token_account".as_ref(), vault.key().as_ref()],
        bump,
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = is_user_stats_for_vault(&vault, &drift_user_stats)?
    )]
    /// CHECK: checked in drift cpi
    pub drift_user_stats: AccountInfo<'info>,
    #[account(
        mut,
        constraint = is_user_for_vault(&vault, &drift_user.key())?
    )]
    /// CHECK: checked in drift cpi
    pub drift_user: AccountLoader<'info, User>,
    /// CHECK: checked in drift cpi
    pub drift_state: AccountInfo<'info>,
    #[account(
        mut,
        token::mint = vault_token_account.mint
    )]
    pub drift_spot_market_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: checked in drift cpi
    pub drift_signer: AccountInfo<'info>,
    #[account(
        mut,
        token::authority = authority,
        token::mint = vault_token_account.mint
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    pub drift_program: Program<'info, Drift>,
    pub token_program: Program<'info, Token>,
}
