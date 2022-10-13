import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Transaction } from '../../target/types/transaction';
import { Multisigwallet } from '../../target/types/multisigwallet';
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import * as fs from 'fs'

import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

async function Test1() {
    const author = anchor.web3.Keypair.generate();
    const receiver = anchor.web3.Keypair.generate();
    const tpid = new anchor.web3.PublicKey("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F");
    const program = anchor.workspace.Multisigwallet as Program<Multisigwallet>;
  // Configure the client to use the local cluster.
  const idl = JSON.parse(fs.readFileSync("./target/idl/multisigwallet.json","utf8"));
  
  const wallet = new anchor.Wallet(author);
  const con = new anchor.web3.Connection(" http://localhost:8899");
  
  console.log(con.getBalance(wallet.publicKey));
  
  const provider = new anchor.AnchorProvider(con,wallet,anchor.AnchorProvider.defaultOptions());
  
  const Program = new anchor.Program(idl,tpid,provider);

  const transaction = await anchor.web3.PublicKey.findProgramAddress([
    utf8.encode("transaction"),author.publicKey.toBuffer(),receiver.publicKey.toBuffer()
  ],tpid);
  const transactionpub = transaction[0];
  let memo = "ujwal";
  const transactionprogram = new anchor.web3.PublicKey("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F");
  it("Is created!", async () => {
    // Add your test here.

    // 
    const tx2 = await Program.methods.createTransaction(memo,2)
    .accounts(
      {
      author: author.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId,
      transaction:transactionpub,
      transactionProgram:transactionprogram
      }

    )
    .signers([author])
    .rpc();
    
    console.log("Your transaction signature", tx2);
    // console.log(t.)
  });
}


Test1()