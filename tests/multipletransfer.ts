import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Multipletransfer } from "../target/types/multipletransfer";

describe("multipletransfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Multipletransfer as Program<Multipletransfer>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    const from = program.provider.wallet.payer;
      console.log("from",from.publicKey.toString());
      const one = anchor.web3.Keypair.generate();
      console.log("First To Address",one.publicKey.toString());
      const two = anchor.web3.Keypair.generate();
      console.log("Second To Address",two.publicKey.toString());
      var  total:number = 2;
    let tx =  await program.rpc.transferMany(new anchor.BN(total),{
        accounts: {
          signer: from.publicKey,
          one: one.publicKey,
          two: two.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [from]
      
      });
    console.log("Transaction Hash",tx);
  });
});
