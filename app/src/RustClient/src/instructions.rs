use anchor_client::solana_client::rpc_client;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::{pubkey, signer::keypair};
use anchor_client::{Client, Cluster};
use multisigwallet::accounts as msw_accounts;
use multisigwallet::instruction as msw_instruction;
use multisigwallet::MultiSigWallet;
use std::rc::Rc;
use std::str::FromStr;
use transaction::accounts as t_accounts;
use transaction::instruction as t_instruction;
use transaction::Transaction;
// pubkey 8rrBaEqmiWbb9JzLHePuA9zX4ToHWoxC4U2KTbebFt4f
// pda 2 BD3icjcrS4otpiCcyRqxkL3mfZYWXewK6NJqrmavvyGS

const KEYPAIR_PATH: &str = "/home/anonymous/.config/solana/id.json";
const LAMPORTS_PER_SOL: u64 = 1000_000_000;
const DEVNET: &str = "https://api.devnet.solana.com";
//msw AMxPAL7BPWV7Uo4FACvBmytWLe1yGN9HdvqTagegJb7B
// pda : F115MqVZZq4yzzZCMMUJVCfd37MrFGSjTGb3nRe8kgm

pub mod instructions {
    use super::*;

    pub fn create_multisig_wallet(msw_keypair: keypair::Keypair) -> Result<(), String> {
        let pid = pubkey::Pubkey::from_str("J6XQ4Zn8jfB3JokXrYBfEADakhwHHmm8C2CdGCe4t6qo").unwrap();
        let sol_rpc = rpc_client::RpcClient::new(DEVNET);
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");

        let receiver_pubkey =
            pubkey::Pubkey::from_str("8rrBaEqmiWbb9JzLHePuA9zX4ToHWoxC4U2KTbebFt4f")
                .expect("err in reading pubkey");
        let payer_pubkey = payer_keypair.try_pubkey().unwrap();
        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );

        println!(
            "Payer balance :{}",
            sol_rpc.get_balance(&payer_pubkey).unwrap() as f64 / LAMPORTS_PER_SOL as f64
        );
        println!(
            "Receiver balance :{}",
            sol_rpc.get_balance(&receiver_pubkey).unwrap()
        );
        let s3 = pubkey::Pubkey::from_str("2ypRzhvkVtXJi3r17m7ULJyYUCkMDY94PAEJQf6rfpCE").unwrap();
        let s4 = pubkey::Pubkey::from_str("2Jpwh3rvtHe2X67TxpAGEB4x751FNMwWzDyQHhBjqfKg").unwrap();
        let program = rpc.program(pid);
        let signers = vec![payer_pubkey, receiver_pubkey, s3, s4];

        println!("Signers:{:?},{:?}", &msw_keypair.pubkey(), &payer_pubkey);
        let ix = program
            .request()
            .signer(&msw_keypair)
            .accounts(msw_accounts::CreateMultiSigWallet {
                wallet: msw_keypair.pubkey(),
                creater: payer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(msw_instruction::CreateMultisigWallet {
                signers: signers,
                min_signatures: 2,
                total_signers: 4,
            })
            .send()
            .unwrap();

        let msw_acc: MultiSigWallet = program.account(msw_keypair.pubkey()).unwrap();

        assert_eq!(msw_acc.min_signatures, 2);

        println!(
            "Multisigwallet account creation  success! with Signature:{:?}",
            ix
        );

        println!("multisigWallet_pubkey : {:?}", msw_keypair.pubkey());

        Ok(())
    }

    pub fn create_transaction() -> Result<(), String> {
        let msw = pubkey::Pubkey::from_str("9HZ6TFToeMzQZEtZ3cVCDktaFfwtpAvtjY3DGToNdKZW").unwrap();
        let sol_rpc = rpc_client::RpcClient::new(DEVNET);
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");

        let receiver_pubkey =
            pubkey::Pubkey::from_str("8FziqCe5oT8773hbRYMvDkpLYTXo2ka2zShiUSK3TUJE")
                .expect("err in reading pubkey");
        let payer_pubkey = payer_keypair.try_pubkey().unwrap();
        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );

        let slot = sol_rpc.get_slot().unwrap();
        let timestamp = sol_rpc.get_block_time(slot).unwrap();
        let ts = timestamp.to_string();

        println!(
            "Payer balance :{}",
            sol_rpc.get_balance(&payer_pubkey).unwrap() as f64 / LAMPORTS_PER_SOL as f64
        );
        println!(
            "Receiver balance :{}",
            sol_rpc.get_balance(&receiver_pubkey).unwrap()
        );
        let t_pid =
            pubkey::Pubkey::from_str("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F").expect("err");
        let tpubkey = pubkey::Pubkey::try_find_program_address(
            &[
                b"transaction_escrow",
                payer_pubkey.as_ref(),
                receiver_pubkey.as_ref(),
                ts.as_bytes(),
            ],
            &t_pid,
        )
        .expect("Err in finding pda");
        let program = rpc.program(t_pid);

        let ix = program
            .request()
            .accounts(t_accounts::CreateTransaction {
                transaction: tpubkey.0,
                author: payer_pubkey,
                receiver: receiver_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(t_instruction::CreateTransaction {
                memo: "pda creation test 8".as_bytes().to_owned(),
                sigrequired: 2,
                amount_in_sol: 1.0,
                timestamp: ts.as_bytes().to_owned(),
                msw_pubkey: msw,
            })
            .send()
            .unwrap();

        println!("Transaction Successful !");
        println!("PDA:{:?}", tpubkey.0);
        println!("Signature:{:?}", ix);

        Ok(())
    }

    pub fn sign_transaction() -> Result<(), String> {
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");
        let s1 = keypair::Keypair::new();
        let payer_pubkey = payer_keypair.pubkey();
        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );
        let pid = pubkey::Pubkey::from_str("J6XQ4Zn8jfB3JokXrYBfEADakhwHHmm8C2CdGCe4t6qo").unwrap();
        let tpid =
            pubkey::Pubkey::from_str("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F").expect("err");
        let pda = pubkey::Pubkey::from_str("GGi55nYiK3nfid2RzQHgke1jmXEUws32nDHpcbr3niVG").unwrap();
        let program = rpc.program(pid);
        let msw = pubkey::Pubkey::from_str("9HZ6TFToeMzQZEtZ3cVCDktaFfwtpAvtjY3DGToNdKZW").unwrap();
        let ix = program
            .request()
            .accounts(msw_accounts::SignTransactionCpi {
                msw: msw,
                transaction: pda,
                transaction_program: tpid,
                signer: payer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(msw_instruction::SignTransaction {})
            .send()
            .unwrap();

        println!("Sig:{:?}", ix);
        let t_acc: Transaction = program.account(pda).unwrap();
        println!("transaction_account:{:?}", t_acc);
        Ok(())
    }
    pub fn get_transaction_acc() -> Result<Transaction, String> {
        let sol_rpc = rpc_client::RpcClient::new(DEVNET);
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");

        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );

        let t_pda = sol_rpc
            .get_account(
                &pubkey::Pubkey::from_str("Gi5usTTgSS94trF1YNktcR9m6HALntGjHz1iiZfYm4h4")
                    .expect("err"),
            )
            .unwrap();
        println!("acc:{:?}", t_pda.owner);

        let program = rpc.program(
            pubkey::Pubkey::from_str("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F").expect("err"),
        );
        let pda = pubkey::Pubkey::from_str("Gi5usTTgSS94trF1YNktcR9m6HALntGjHz1iiZfYm4h4").unwrap();
        let acc: Transaction = program.account(pda).unwrap();
        println!("Acc:{:?}", acc);
        Ok(acc)
    }

    pub fn execute_transaction(msw_keypair: keypair::Keypair) -> Result<(), String> {
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");
        let s1 = keypair::Keypair::new();
        let payer_pubkey = payer_keypair.pubkey();

        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );
        let pid = pubkey::Pubkey::from_str("J6XQ4Zn8jfB3JokXrYBfEADakhwHHmm8C2CdGCe4t6qo").unwrap();
        let tpid =
            pubkey::Pubkey::from_str("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F").expect("err");
        let pda = pubkey::Pubkey::from_str("6voGcVxQEF5YG3KqaFPBnRhwhMxnbaiVdaQqMMUpRTZN").unwrap();
        let program = rpc.program(pid);
        let receiver_pubkey =
            pubkey::Pubkey::from_str("8FziqCe5oT8773hbRYMvDkpLYTXo2ka2zShiUSK3TUJE")
                .expect("err in reading pubkey");
        let msw = pubkey::Pubkey::from_str("JAzaZzayNjpGjgouQGxyfGw8Qz6PCZbDixC1WswvSVWD").unwrap();
        let ix = program
            .request()
            .accounts(msw_accounts::ExecuteTransactionCpi {
                msw: msw,
                transaction: pda,
                transaction_program: tpid,
                signer: payer_pubkey,
                receiver: receiver_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .signer(&msw_keypair)
            .args(msw_instruction::ExecuteTransaction {})
            .send()
            .unwrap();

        println!("Sig:{:?}", ix);
        let t_acc: Transaction = program.account(pda).unwrap();
        println!("transaction_account:{:?}", t_acc);
        Ok(())
    }

    pub fn fund_msw() -> Result<(), String> {
        let pid = pubkey::Pubkey::from_str("J6XQ4Zn8jfB3JokXrYBfEADakhwHHmm8C2CdGCe4t6qo").unwrap();
        let sol_rpc = rpc_client::RpcClient::new(DEVNET);
        let keypair_path = "/home/anonymous/.config/solana/id.json";
        let payer_keypair =
            keypair::read_keypair_file(keypair_path).expect("Error in reading Keypair Path");

        let msw_pubkey = pubkey::Pubkey::from_str("9HZ6TFToeMzQZEtZ3cVCDktaFfwtpAvtjY3DGToNdKZW")
            .expect("err in reading pubkey");
        let payer_pubkey = payer_keypair.try_pubkey().unwrap();
        let rpc = Client::new_with_options(
            Cluster::Devnet,
            Rc::new(payer_keypair),
            CommitmentConfig::processed(),
        );
        let amount_in_sol = 0.001;
        let program = rpc.program(pid);
        let ix = program
            .request()
            .accounts(msw_accounts::FundAccount {
                wallet: msw_pubkey,
                payer: payer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::id(),
            })
            .args(msw_instruction::FundAccount {
                amount_in_sol: amount_in_sol,
            })
            .send()
            .unwrap();

        println!("Msw account:{:?} funded with:{}", msw_pubkey, amount_in_sol);
        Ok(())
    }
}
