use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use serde::Deserialize;
use transaction::cpi::accounts::{ExecuteTransaction, SignTransaction};
use transaction::program::Transaction as T;
use transaction::{self, Transaction};
const LAMPORTS_PER_SOL: u64 = 1000_000_000;
declare_id!("J6XQ4Zn8jfB3JokXrYBfEADakhwHHmm8C2CdGCe4t6qo");

#[program]
pub mod multisigwallet {

    use super::*;

    pub fn create_multisig_wallet(
        ctx: Context<CreateMultiSigWallet>,
        signers: Vec<Pubkey>,
        min_signatures: u8,
        total_signers: u8,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        wallet.min_signatures = min_signatures;
        wallet.max_signatures = total_signers;

        if signers.len() as u8 != total_signers {
            return Err(error::Error::AnchorError(AnchorError {
                error_name: "MissingFewSignersInVector".to_owned(),
                error_code_number: 102,
                error_msg: "total signers is not matching with Signer Vector length".to_owned(),
                error_origin: Some(ErrorOrigin::AccountName("MutltiSigWallet".to_owned())),
                compared_values: None,
            }));
        }
        for i in signers.iter() {
            wallet.signers.push(*i);
        }

        return Ok(());
    }

    pub fn sign_transaction(ctx: Context<SignTransactionCpi>) -> Result<()> {
        let wallet = &ctx.accounts.msw;
        let signer = ctx.accounts.signer.key();
        if !wallet.signers.contains(&signer) {
            return Err(error::Error::AnchorError(AnchorError {
                error_name: "InvalidSigner".to_owned(),
                error_code_number: 101,
                error_msg: "Signer not authorized to sign this transaction".to_owned(),
                error_origin: Some(ErrorOrigin::AccountName("Transaction".to_owned())),
                compared_values: None,
            }));
        }
        let cpi_program = ctx.accounts.transaction_program.to_account_info();
        let cpi_accounts = SignTransaction {
            transaction: ctx.accounts.transaction.to_account_info(),
            signer: ctx.accounts.signer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transaction::cpi::sign_transaction(cpi_ctx).expect("err in signing transaction!");
        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransactionCpi>) -> Result<()> {
        let wallet = &ctx
            .accounts
            .msw
            .deserialize_data::<MultiSigWallet>()
            .unwrap();

        let signer = ctx.accounts.signer.key();
        if !wallet.signers.contains(&signer) {
            return Err(error::Error::AnchorError(AnchorError {
                error_name: "InvalidSigner".to_owned(),
                error_code_number: 101,
                error_msg: "Signer not authorized to execute this transaction".to_owned(),
                error_origin: Some(ErrorOrigin::AccountName("Transaction".to_owned())),
                compared_values: None,
            }));
        }
        let cpi_program = ctx.accounts.transaction_program.to_account_info();
        let cpi_accounts = ExecuteTransaction {
            transaction: ctx.accounts.transaction.to_account_info(),
            wallet: ctx.accounts.msw.to_account_info(),
            receiver: ctx.accounts.receiver.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transaction::cpi::execute_transaction(cpi_ctx).expect("err in transaction execution!");
        Ok(())
    }

    pub fn fund_account(ctx: Context<FundAccount>, amount_in_sol: f64) -> Result<()> {
        let payer = ctx.accounts.payer.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        let wallet = ctx.accounts.wallet.to_account_info();
        if payer.lamports() as f64 >= amount_in_sol * LAMPORTS_PER_SOL as f64 && payer.is_signer {
            let ix = system_instruction::transfer(
                &payer.key(),
                &wallet.key(),
                (LAMPORTS_PER_SOL as f64 * amount_in_sol) as u64,
            );
            invoke(&ix, &[payer, wallet, system_program]).expect("err in instruction");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SignTransactionCpi<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub msw: Account<'info, MultiSigWallet>,
    pub transaction_program: Program<'info, T>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransactionCpi<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    /// CHECK:Account is safe
    pub receiver: AccountInfo<'info>,
    pub msw: Signer<'info>,
    pub transaction_program: Program<'info, T>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMultiSigWallet<'info> {
    #[account(init,payer = creater,space = SPACE as usize)]
    pub wallet: Account<'info, MultiSigWallet>,
    #[account(mut)]
    pub creater: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug, Deserialize)]
pub struct MultiSigWallet {
    pub signers: Vec<Pubkey>, //imp
    pub min_signatures: u8,   //imp
    pub max_signatures: u8,   // imp
}

#[derive(Accounts)]
pub struct FundAccount<'info> {
    #[account(mut)]
    pub wallet: Account<'info, MultiSigWallet>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

const SPACE: usize = 8 + MAX_SIGNERS + MIN_SIGNERS + SIGNERS; //512 bytes
const MAX_SIGNERS: usize = 1;
const SIGNERS: usize = 16 * PUBKEY;
const PUBKEY: usize = 32;
const MIN_SIGNERS: usize = 1;
