// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import { Provider, Program, setProvider, workspace, web3 } from '@project-serum/anchor';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MintLayout,
  Token,
  u64,
} from '@solana/spl-token';
import toml from 'toml';
import fs from 'fs';

import { Deposit } from '../target/types/deposit';

import idl from '../target/idl/deposit.json';

const anchorToml = toml.parse(
  fs.readFileSync(__dirname + '/../Anchor.toml').toString()
);

const address = anchorToml.programs.mainnet.deposit;

const manager = web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(anchorToml.owner.manager, { encoding: 'utf8' }))));

module.exports = async function (provider) {
  // Configure client to use the provider.
  setProvider(provider);
  
  const program = new Program<Deposit>(idl as unknown as Deposit, address, provider);

  console.log("\nmanager initializing...");

  console.log("program", program.programId.toBase58());

  // 设置钱包为owner：provider.wallet.publicKey，如果需要设置其他owner，请自行更换
  const tx = await program.rpc.initManager(provider.wallet.publicKey, {
    accounts: {
      manager: manager.publicKey,
      payer: provider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    },
    signers: [manager],
  });

  console.log("transaction signature", tx);

  console.log("initialized!");
};
