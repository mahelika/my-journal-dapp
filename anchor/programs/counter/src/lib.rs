#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod journal {
  use super::*;

  pub fn create_journal_entry(
    ctx: Context<CreateEntry>,
    title: String,
    message: String,
  ) -> Result<()> {
    msg!("Journal Entry Created");
    msg!("Title: {}", title);
    msg!("Message: {}", message);

    let journal_entry = &mut ctx.accounts.journal_entry;
    journal_entry.owner = ctx.accounts.owner.key();
    journal_entry.title = title;
    journal_entry.message = message;
    Ok(())
  }
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  pub owner: Pubkey,
  #[max_len(50)]
  pub title: String,
  #[max_len(1000)]
  pub message: String
}