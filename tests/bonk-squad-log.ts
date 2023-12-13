import * as anchor from "@coral-xyz/anchor";
import { assert, expect } from "chai"
import { BonkSquadLog } from "../target/types/bonk_squad_log"
import { BN } from "bn.js";

describe("bonk-squad-log", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .BonkSquadLog as anchor.Program<BonkSquadLog>

  const player = {
    name: "Just a test player",
    squad: "The Thunderbirds",
    score: 5,
  }

  const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(player.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("Player is added", async () => {
    // Add test here.
    const tx = await program.methods
    .addPlayer(player.name, player.squad, new anchor.BN(player.score))
    .rpc()

    const account = await program.account.playerAccountState.fetch(playerPda)
    expect(player.name === account.name)
    expect(player.squad === account.squad)
    expect(player.score === account.score.toNumber())
    expect(account.key === provider.wallet.publicKey)  
  })

  xit("Player is updated`", async () => {
    // test goes here
    const newSquad = "new";
    const newScore = 4;

    const tx = await program.methods
      .updatePlayer(player.name, newSquad)
      .rpc()

    const account = await program.account.playerAccountState.fetch(playerPda)
    expect(player.name === account.name)
    expect(newSquad === account.squad)
    expect(newScore === account.score.toNumber())
    expect(account.key === provider.wallet.publicKey)
  })

  xit("Deletes a player", async () => {
    const tx = await program.methods
    .deletePlayer(player.name)
    .rpc()
  })
})
