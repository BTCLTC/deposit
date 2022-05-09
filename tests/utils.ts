import { web3, getProvider } from '@project-serum/anchor';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  Token,
} from '@solana/spl-token';

import { mint } from './constant';

const provider = getProvider();

export const getAta = async (owner: web3.PublicKey) => {
  return Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    new web3.PublicKey(mint),
    owner
  );
};

// get token amount
export const getTokenAmount = async (account: web3.PublicKey) => {
  const ata = await getAta(account);
  const data = await provider.connection.getTokenAccountBalance(ata);
  return data.value.uiAmountString;
}
