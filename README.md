#Whitelist Gated Token Sale
This repository contains a Solana-based program for managing a whitelist-gated token sale written in Rust using the Anchor framework. The program allows users to initialize a sale, manage a whitelist, and buy tokens if they are on the whitelist.


## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Program Architecture](#program-architecture)
- [Tests](#tests)

Initialize Sale: Allows an administrator to initialize a token sale with a whitelist of allowed buyers.
Manage Whitelist: Allows the addition and removal of users from the whitelist.
Buy Tokens: Enables whitelisted users to buy tokens, enforcing purchase limits.

# Installation
 To get started, clone this repository and install the necessary dependencies.

```bash

git clone https://github.com/yourusername/whitelist-gated-sale.git
cd whitelist-gated-sale
```

Ensure you have the following installed:
Rust
Solana CLI
Anchor

# Usage

Building the Program
To build the program, run:

```bash
anchor build
```
# Deploying the Program
To deploy the program to the Solana devnet, run:

```bash

anchor deploy
```

# Running Tests
To run the provided test cases, use:

```bash
anchor test
```
Program Architecture
Modules and Functions
initialize
Initializes the token sale with a unique ID and a whitelist of allowed buyers.

```rust

pub fn initialize(
    ctx: Context<Initialize>,
    _unique_id: u64,
    whitelist: Vec<Pubkey>,
) -> Result<()>
```

# add_user_to_white_list
Adds a new user to the whitelist.

```rust

pub fn add_user_to_white_list(ctx: Context<AddUser>) -> Result<()>

```
# remove_user_from_list
Removes a user from the whitelist.

```rust

pub fn remove_user_from_list(ctx: Context<RemoveUser>) -> Result<()>
```

# buy_token
Allows a whitelisted user to buy tokens, enforcing a purchase limit.

```rust

pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()>
```
# Accounts

## State
Stores the details of the sale state, including the whitelist and the owner.

```rust

#[account]
pub struct State {
    pub whitelist: Vec<Pubkey>,
    pub owner: Pubkey,
}
```
 ## UserInfo
Stores the details of a user's token purchases.

```rust

#[account]
pub struct UserInfo {
    pub account: String,
    pub purchased_amount: u64,
}
```
## Error Codes
```rust

#[error_code]
pub enum ErrorCode {
    #[msg("Buyer is not whitelisted.")]
    NotWhitelisted,
    #[msg("Purchase limit exceeded.")]
    PurchaseLimitExceeded,
    #[msg("Overflow occurred.")]
    Overflow,
    #[msg("Underflow occurred.")]
    Underflow,
    #[msg("This list was not created by you")]
    NoPermissionToUpdateList,
}
```

# Tests
The tests for the program are written in JavaScript using the @coral-xyz/anchor library.

### Example Test Case

```javascript

import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get the path to the JSON file
  const filePath = path.join(os.homedir(), '.config', 'solana', 'id.json');

  const data = fs.readFileSync(filePath, 'utf-8');
  let key = JSON.parse(data);

  const pg = web3.Keypair.fromSecretKey(Uint8Array.from(key));

  const program = anchor.workspace.WhitelistGatedSale;
  const connection = new web3.Connection("https://api.devnet.solana.com");

  let uniqueId = Math.floor(Date.now() / 1000);
  const uniqueIdBuffer = Buffer.alloc(8);
  uniqueIdBuffer.writeUInt32LE(uniqueId, 0);

  it("initialize", async () => {
    const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("whitelist"),
        pg.publicKey.toBuffer(),
        uniqueIdBuffer,
      ],
      program.programId
    );

    const txHash = await program.methods
      .initialize(new anchor.BN(uniqueId), [])
      .accounts({
        state: statePda,
        user: pg.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    console.log(txHash);
  });

  it("add user to whitelist", async () => {
    const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("whitelist"),
        pg.publicKey.toBuffer(),
        uniqueIdBuffer,
      ],
      program.programId
    );

    const newAccount = web3.Keypair.generate();
    const txHash = await program.methods
      .addUserToWhiteList()
      .accounts({
        state: statePda,
        newAccount: newAccount.publicKey,
        user: pg.publicKey,
      })
      .signers([])
      .rpc();

    console.log(txHash);
  });

  it("buy token", async () => {
    const [statePda, stateBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("whitelist"),
        pg.publicKey.toBuffer(),
        uniqueIdBuffer,
      ],
      program.programId
    );

    const buyer = new web3.Keypair();
    const tx = await connection.requestAirdrop(buyer.publicKey, 1e9);

    const [userInfoPda, userInfoBump] = await web3.PublicKey.findProgramAddress(
      [buyer.publicKey.toBuffer()],
      program.programId
    );

    const txHash = await program.methods
      .buyToken(1)
      .accounts({
        state: statePda,
        userInfo: userInfoPda,
        buyer: buyer.publicKey,
        treasury: pg.publicKey,
        mint: tokenMint.publicKey,
        tokenAccount: tokenAccount.publicKey,
        tokenProgram: token.TOKEN_PROGRAM_ID,
        tokenAuthority: pg.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("Transaction hash:", txHash);
  });
});
```