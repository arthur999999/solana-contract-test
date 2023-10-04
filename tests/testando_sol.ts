import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestandoSol } from "../target/types/testando_sol";

describe("testando_sol", async () => {

  anchor.setProvider(anchor.AnchorProvider.env())
  const program = anchor.workspace.TestandoSol as Program<TestandoSol>

  it("send a new tweeitwer", async () => {

    const tweetKeyPair = anchor.web3.Keypair.generate();

    await program.methods.sendTweet("teet aler", "cervejaaaa" ).accounts({
      myTweet: tweetKeyPair.publicKey,
      senderOfTweet: program.provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([tweetKeyPair])
    .rpc()

    const tweetAcount = await program.account.tweetOnSolana.all();

    console.log(tweetAcount)

  })
});
