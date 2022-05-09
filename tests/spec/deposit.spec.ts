import { workspace, web3, Program, BN } from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import bs58 from 'bs58';

import { privateKey, mint } from '../constant';
import { getAta, getTokenAmount } from '../utils';
import { Deposit } from '../../target/types/deposit';

// account: 用户的钱包，即存款人
const secretKey = Uint8Array.from(bs58.decode(privateKey));
const account = web3.Keypair.fromSecretKey(secretKey);

// 管理员地址
const managerPubkey = new web3.PublicKey('GameKkXYTeW7RV7AiCj5M1WEVuCH7m9WXgMaLErscnKZ');

// 存一个代币
const depositAmount = new BN(10).pow(new BN(9));

describe('deposit', () => {
  it('should deposit successfully', async () => {

    const program = workspace.Gamefi as Program<Deposit>;

    // wallet，因为初始化管理员的时候，使用wallet来做为owner
    const owner = new web3.PublicKey('Game1CfEuicdVbM2YHBHz8Z3JVF4YAkMz9wf2XRpZ8gW');

    // owner的ATA账户
    const ownerAccount = await getAta(owner);

    // 存款人的ATA账户
    const depositAccount = await getAta(account.publicKey);

    const beforeTokenAmount = await getTokenAmount(account.publicKey);

    console.log('before deposit token amount: ', beforeTokenAmount);

    const tx = await program.rpc.deposit(depositAmount, {
      accounts: {
        manager: managerPubkey,
        owner,
        ownerAccount,
        depositor: account.publicKey,
        tokenMint: new web3.PublicKey(mint),
        depositAccount,
        rent: web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [account]
    });

    console.log('transaction signature', tx);

    const afterTokenAmount = await getTokenAmount(account.publicKey);

    console.log('after deposit token amount: ', afterTokenAmount);

    console.log('deposited!');
  });
});
