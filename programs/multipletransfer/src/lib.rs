use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

declare_id!("D6UguvhBasaawUG51egV9vfQVmyZ6UMW2ej6aPYkWtt");

#[program]
pub mod multipletransfer {
    use super::*;

    pub fn transfer_many(ctx: Context<TransferMany>, total: u64) -> Result<()> {
        let account_count = 2;
        let lamports = total * LAMPORTS_PER_SOL / account_count;

        let to_account_infos = vec![ctx.accounts.one.clone(), ctx.accounts.two.clone()];

        let ixs = anchor_lang::solana_program::system_instruction::transfer_many(
            &ctx.accounts.signer.key,
            &[
                (to_account_infos[0].key.to_owned(), lamports),
                (to_account_infos[1].key.to_owned(), lamports),
            ],
        );

        for (i, ix) in ixs.iter().enumerate() {
            invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    to_account_infos[i].to_account_info(),
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferMany<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)]
  pub one: UncheckedAccount<'info>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)]
  pub two: UncheckedAccount<'info>,
  pub system_program: Program<'info, System>,
}
