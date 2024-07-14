import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { WhitelistGatedSale } from "../target/types/whitelist_gated_sale";
import fs from "fs";
import * as path from 'path';
import * as os from 'os';
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("whitelist_gated_sale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WhitelistGatedSale as Program<WhitelistGatedSale>;
  const connection = new web3.Connection("https://api.devnet.solana.com");


  // Get the path to the JSON file
  const filePath = path.join(os.homedir(), '.config', 'solana', 'id.json');

  const data = fs.readFileSync(filePath, 'utf-8');
  let key = JSON.parse(data);
  const pg = web3.Keypair.fromSecretKey(Uint8Array.from(key));

  let user = new web3.Keypair();

  // Generate keypair for the new account
  let uniqueId = 1234;
  const uniqueIdBuffer = Buffer.alloc(8);
  uniqueIdBuffer.writeUInt32LE(uniqueId, 0);

  var whiteListAccount = ""

  it("initialize", async () => {



    let whitelist = [
      new web3.PublicKey("HB7ygJr1dWGy2ViwcVbH7TVMYiTGu55hxoi9DHLkCcZu"),
      new web3.PublicKey("An11GaHJPkMHydDB8ENFKYn2KpdXZZxXohWXF5HVvC9d"),
      new web3.PublicKey("5ojwv9NJC4rDEaPLjgUupA5SrsNKh8QVxe3X6cqso89f"),
    ];

    const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("whitelist"),
        pg.publicKey.toBuffer(),
        uniqueIdBuffer,
      ],
      program.programId
    );

    whiteListAccount = statePda.toString()

    const txHash = await program.methods
      .initialize(new anchor.BN(uniqueId), whitelist)
      .accounts({
        state: statePda,
        user: pg.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([pg])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await connection.confirmTransaction(txHash);
    // Fetch the created account
    const newWhitelist = await program.account.state.fetch(statePda);
    console.log("On-chain data is:", newWhitelist.whitelist.toString());

  });
  it("add users", async () => {
    let statePda = new web3.PublicKey(whiteListAccount);
    let newAccount = new web3.PublicKey(
      "13dqNw1su2UTYPVvqP6ahV8oHtghvoe2k2czkrx9uWJZ"
    );
    // const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
    //   [Buffer.from("whitelist"), pg.wallets.programm.publicKey.toBuffer()],
    //   pg.PROGRAM_ID
    // );
    const txHash = await program.methods
      .addUserToWhiteList()
      .accounts({
        state: statePda,
        newAccount: newAccount,
        user: pg.publicKey,
      })
      .signers([pg])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await connection.confirmTransaction(txHash);
    // Fetch the created account
    const newWhitelist = await program.account.state.fetch(statePda);
    console.log("On-chain data is:", newWhitelist.whitelist.toString());
    // Check whether the data on-chain is equal to local 'data'
    // assert(newWhitelist.whitelist.indexOf());
  });
  it("removes users", async () => {
    let statePda = new web3.PublicKey(whiteListAccount);
    let newAccount = new web3.PublicKey(
      "13dqNw1su2UTYPVvqP6ahV8oHtghvoe2k2czkrx9uWJZ"
    );
    // const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
    //   [Buffer.from("whitelist"), pg.wallets.programm.publicKey.toBuffer()],
    //   pg.PROGRAM_ID
    // );
    const txHash = await program.methods
      .removeUserFromList()
      .accounts({
        state: statePda,
        newAccount: newAccount,
        user: pg.publicKey,
      })
      .signers([pg])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await connection.confirmTransaction(txHash);
    // Fetch the created account
    const newWhitelist = await program.account.state.fetch(statePda);
    console.log("On-chain data is:", newWhitelist.whitelist.toString());
    // Check whether the data on-chain is equal to local 'data'
    // assert(newWhitelist.whitelist.indexOf());
  });


  // it("buys token", async () => {

  //   let statePda = new web3.PublicKey(whiteListAccount);

  //   let buyer = new web3.Keypair();

  //   const tx = await connection.requestAirdrop(buyer.publicKey, 1e9)

  //   const [userPda, userBump] = await web3.PublicKey.findProgramAddress(
  //     [buyer.publicKey.toBuffer()],
  //     program.programId
  //   );
  //   const txHash = await program.methods
  //     .buyToken(new anchor.BN(10))
  //     .accounts({
  //       state: statePda,
  //       userInfo: userPda,
  //       buyer: buyer.publicKey,
  //       treasury: new web3.PublicKey(
  //         "13dqNw1su2UTYPVvqP6ahV8oHtghvoe2k2czkrx9uWJZ"
  //       ),
  //       mint: "",
  //       tokenAccount: "",
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       tokenAuthority: "",
  //       systemProgram: web3.SystemProgram.programId,
  //     })
  //     .signers([buyer])
  //     .rpc();
  //   console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  //   // Confirm transaction
  //   await connection.confirmTransaction(txHash);
  //   // Fetch the created account
  //   const newWhitelist = await program.account.state.fetch(statePda);
  //   console.log("On-chain data is:", newWhitelist.whitelist.toString());
  //   // Check whether the data on-chain is equal to local 'data'
  //   // assert(da.eq(newWhitelist.whitelist.toString()));
  // });
});
