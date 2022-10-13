use anchor_lang::prelude::ProgramError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint_deprecated::ProgramResult;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use serde::Deserialize;
use std::str::from_utf8;
const LAMPORTS_PER_SOL: u64 = 1000_000_000;
declare_id!("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F");

#[program]
pub mod transaction {

    use super::*;

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        timestamp: Vec<u8>,
        memo: Vec<u8>,
        sigrequired: u16,
        amount_in_sol: f64,
        msw_pubkey: Pubkey,
    ) -> Result<()> {
        let transaction: &mut Account<Transaction> = &mut ctx.accounts.transaction;

        let clock: Clock = Clock::get().unwrap();
        let memo1 = from_utf8(&memo)
            .map_err(|err| {
                msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
                ProgramError::InvalidInstructionData
            })
            .unwrap();
        msg!("The memo of tranaction is {}", memo1);

        transaction.multisig_wallet = msw_pubkey;
        transaction.timestamp = clock.unix_timestamp as u32;
        transaction.memo = memo;
        transaction.sigreceived = 1;
        transaction.receiver = ctx.accounts.receiver.key();

        // This needs to fetched from wallet account property via CPI
        transaction.sigrequired = sigrequired;
        transaction.amount_in_sol = amount_in_sol;

        transaction.ispending = true;

        if transaction.sigreceived >= transaction.sigrequired {
            transaction.ispending = false
        };

        Ok(())
    }

    pub fn sign_transaction(ctx: Context<SignTransaction>) -> Result<()> {
        let transaction: &mut Account<Transaction> = &mut ctx.accounts.transaction;
        let signer = ctx.accounts.signer.key();

        // Before doing this, add some constraint that the current signer belong to
        // the signer list in wallet account
        if transaction.ispending {
            transaction.sigreceived = transaction.sigreceived + 1;
            if transaction.sigreceived >= transaction.sigrequired {
                transaction.ispending = false;
            }
        }

        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        let transaction: &mut Account<Transaction> = &mut ctx.accounts.transaction;
        let multisig_wallet = ctx.accounts.wallet.key();
        let receiver = ctx.accounts.receiver.key();
        let amount = transaction.amount_in_sol;
        let system_program = ctx.accounts.system_program.to_account_info();
        let wallet_acc = ctx.accounts.wallet.to_account_info();
        let receiver_acc = ctx.accounts.receiver.to_account_info();
        if transaction.ispending == false {
            let ix = system_instruction::transfer(
                &multisig_wallet,
                &receiver,
                (LAMPORTS_PER_SOL as f64 * amount) as u64,
            );
            invoke(&ix, &[wallet_acc, receiver_acc, system_program]).expect("err in instruction");
        };
        // Forward the transaction to submit

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(timestamp:Vec<u8>)]
pub struct CreateTransaction<'info> {
    #[account(init,seeds = [b"transaction_escrow",author.key().as_ref(),receiver.key().as_ref(),timestamp.as_ref()],bump, payer = author, space = Transaction::LEN)]
    pub transaction: Account<'info, Transaction>,

    #[account(mut)]
    pub author: Signer<'info>,
    /// CHECK: receiver receiving tokens
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    /// CHECK:safe account
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub multisig_wallet: Pubkey, //imp
    pub timestamp: u32,          //imp
    pub ispending: bool,         //imp
    pub sigreceived: u16,        //imp
    pub sigrequired: u16,        //imp
    pub memo: Vec<u8>,           //imp
    pub amount_in_sol: f64,      //imp
    pub receiver: Pubkey,        //imp
}

const DISCRIMINATOR_LENGTH: usize = 8;
const MULTISIG_WALLET_PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 4;
const PENDING_STATUS_LENGTH: usize = 1; // 1 byte to store the bool
const SIG_RECIEVED_LENGTH: usize = 2; // Max 2^16 members multi sig wallet
const SIG_REQUIRED_LENGTH: usize = 2; // Max 2^16 members multi sig wallet
const RECEIVER_PUBKEY: usize = 32; // Stores the size of the string.
const MEMO_LENGTH: usize = 25; // 25 chars max
const AMOUNT_IN_SOL: usize = 8;

impl Transaction {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + MULTISIG_WALLET_PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + PENDING_STATUS_LENGTH
        + SIG_RECIEVED_LENGTH
        + SIG_REQUIRED_LENGTH
        + RECEIVER_PUBKEY+AMOUNT_IN_SOL+ MEMO_LENGTH; // Transaction memo.
}
