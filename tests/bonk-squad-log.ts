import * as anchor from "@coral-xyz/anchor";
import { assert, expect } from "chai"
import { BonkSquadLog } from "../target/types/bonk_squad_log"
import { BN } from "bn.js";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import initWallet from '../wba-wallet.json'
import playerWallet from '../taker-wallet.json'

describe("bonk-squad-log", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .BonkSquadLog as anchor.Program<BonkSquadLog>

  const test_player = {
    name: "Test player 2",
    squad: "The Thunderbirds",
    score: 5,
  }

  const initializer = Keypair.fromSecretKey(new Uint8Array(initWallet));
  //const player = Keypair.fromSecretKey(new Uint8Array(playerWallet));
  const player = PublicKey.findProgramAddressSync([Buffer.from(test_player.name), initializer.publicKey.toBuffer()], program.programId)[0];

  const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(test_player.name), initializer.publicKey.toBuffer()],
    program.programId
  )

  it("Player is added", async () => {
    // Add test here.
    const tx = await program.methods
    .addPlayer(test_player.name, test_player.squad, new anchor.BN(test_player.score))
    .accounts({
      player,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ initializer ])
    .rpc()

    const account = await program.account.playerAccountState.fetch(playerPda)
    expect(test_player.name === account.name)
    expect(test_player.squad === account.squad)
    expect(test_player.score === account.score.toNumber())
    expect(account.key === provider.wallet.publicKey)
  })

  it("Player is updated", async () => {
    // test goes here
    const newSquad = "new";
    const newScore = 4;

    const tx = await program.methods
    .updatePlayer(test_player.name, newSquad)
    .accounts({
      player,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ initializer ])
    .rpc()

    const account = await program.account.playerAccountState.fetch(playerPda)
    expect(test_player.name === account.name)
    expect(newSquad === account.squad)
    expect(newScore === account.score.toNumber())
    expect(account.key === provider.wallet.publicKey)
  })

  it("Delete player", async () => {
    const tx = await program.methods
    .deletePlayer(test_player.name)
    .accounts({
      player,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ initializer ])
    .rpc()
  })
})
