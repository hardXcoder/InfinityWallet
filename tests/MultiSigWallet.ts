import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Transaction } from '../target/types/transaction';
import { Multisigwallet } from '../target/types/multisigwallet';
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";

import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { assert } from "chai";

describe("MultisigWallet",async () => {

  // Configure the client to use the local cluster.
  
  it("Is created!", async () => {
    // Add your test here.
  anchor.setProvider(anchor.AnchorProvider.env());
  const author = anchor.web3.Keypair.generate();
  const receiver = anchor.web3.Keypair.generate();
  const tpid = new anchor.web3.PublicKey("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F");
  const program = anchor.workspace.Multisigwallet as Program<Multisigwallet>;
  
  const multisigwallet = anchor.web3.Keypair.generate();
  const signers = [new anchor.web3.PublicKey("AsM97N16ejpKcVJTwEWtnLsDMz7jFPGr6SU1vzJD9xZt"),
  new anchor.web3.PublicKey("8rrBaEqmiWbb9JzLHePuA9zX4ToHWoxC4U2KTbebFt4f")];
  let minsig = new anchor.BN(1);
  let total = new anchor.BN(2);
  const tx1 = await program.methods.createMultisigWallet(signers,2,2)
  .accounts({
    wallet:multisigwallet.publicKey,
    creater:author.publicKey,
    systemProgram:anchor.web3.SystemProgram.programId

})
.signers([multisigwallet,author])
.rpc();

  const transaction = await anchor.web3.PublicKey.findProgramAddress([
    utf8.encode("transaction"),author.publicKey.toBuffer(),receiver.publicKey.toBuffer()
  ],tpid);
  const transactionpub = transaction[0];
  let memo = "ujwal";
  const transactionprogram = new anchor.web3.PublicKey("Ad7FUpQyYDLFnsFo4bYMCYwAfZuBhhaUE89uiwVXQ94F");

    // 
    const tx2 = await program.methods.createTransaction(memo,2)
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
    const t = await  program.account.multiSigWallet.fetch(multisigwallet.publicKey);

    console.log(t.minSignatures);
    assert.equal(t.minSignatures,1);
    console.log("Your transaction signature", tx2);
    // console.log(t.)
  });
});
