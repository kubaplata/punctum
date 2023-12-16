import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PunctumProtocol } from "../target/types/punctum_protocol";

describe("punctum_protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PunctumProtocol as Program<PunctumProtocol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
