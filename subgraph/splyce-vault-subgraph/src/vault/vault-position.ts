import { BigInt, log } from "@graphprotocol/graph-ts";
import { Account, AccountVaultPosition, Token, Vault } from "../../generated/schema";
import { BIGINT_ZERO } from "../constants";

export function buildId(account: Account, vault: Vault): string {
    return account.id.concat('-').concat(vault.id);
}

export function deposit(
    // vaultContract: VaultPackage,
    account: Account,
    vault: Vault,
    txHash: string,
    depositedTokens: BigInt,
    receivedShares: BigInt,
    balanceShares: BigInt,
    total_debt: BigInt,
    total_idle: BigInt,
    total_shares: BigInt
  ): AccountVaultPosition {
    log.debug('[VaultPosition] Deposit', []);
    // TODO Use getOrCreate function
    let vaultPositionId = buildId(account, vault);
    //let txHash = transaction.hash.toHexString();
    let accountVaultPosition = AccountVaultPosition.load(vaultPositionId);
    // let accountVaultPositionUpdate: AccountVaultPositionUpdate;
    // TODO Use tokenLibrary.getOrCreate
    let token = Token.load(vault.token) as Token;
    let pricePerShare =  total_debt.plus(total_idle).div(total_shares) //(total_debt + total_idle) / total_shares
    let balancePosition = balanceShares.times(pricePerShare).div(BigInt.fromI32(token.decimals));//getBalancePosition(account, vaultContract);
    if (accountVaultPosition == null) {
      log.info('Tx: {} Account vault position {} not found. Creating it.', [
        txHash,
        vaultPositionId,
      ]);
      accountVaultPosition = new AccountVaultPosition(vaultPositionId);
      accountVaultPosition.vault = vault.id;
      accountVaultPosition.account = account.id;
      accountVaultPosition.token = vault.token;
      accountVaultPosition.shareToken = vault.shareToken;
    //   accountVaultPosition.transaction = transaction.id;
      accountVaultPosition.balanceTokens = depositedTokens;
      accountVaultPosition.balanceShares = balanceShares;
      accountVaultPosition.balanceProfit = BIGINT_ZERO;
    //   accountVaultPositionUpdate = vaultPositionUpdateLibrary.createFirst(
    //     account,
    //     vault,
    //     vaultPositionId,
    //     BIGINT_ZERO,
    //     transaction,
    //     depositedTokens,
    //     receivedShares,
    //     balanceShares,
    //     balancePosition
    //   );
  
      accountVaultPosition.balancePosition = balancePosition;
    //   accountVaultPosition.latestUpdate = accountVaultPositionUpdate.id;
      accountVaultPosition.save();
    } else {
      log.info('Tx: {} Account vault position {} found. Using it.', [
        txHash,
        vaultPositionId,
      ]);
  
      // Assuming accountVaultPosition.latestUpdate is a string of concatenated hashes separated by '-'
    //   let updatesParts = accountVaultPosition.latestUpdate.split('-');
    //   let thirdElementOfUpdate: string | null;
  
    //   // Check if the updatesParts array has more than two elements
    //   if (updatesParts.length > 2) {
    //     thirdElementOfUpdate = updatesParts[2]; // Get the third element
    //   } else {
    //     thirdElementOfUpdate = null; // Set to null if there aren't enough parts
    //   }
  
      // AssemblyScript does not handle 'null' the same way, so we check for null before proceeding
    //   if (thirdElementOfUpdate != null && thirdElementOfUpdate != txHash) {
        // If they are different, proceed with updating the accountVaultPosition
        accountVaultPosition.balanceTokens = accountVaultPosition.balanceTokens.plus(depositedTokens);
        accountVaultPosition.balanceShares = balanceShares;
        // accountVaultPositionUpdate = vaultPositionUpdateLibrary.deposit(
        //   account,
        //   vault,
        //   vaultPositionId,
        //   accountVaultPosition.latestUpdate,
        //   transaction,
        //   depositedTokens,
        //   receivedShares,
        //   balanceShares,
        //   balancePosition
        // );
  
        accountVaultPosition.balancePosition = balancePosition;
        // accountVaultPosition.latestUpdate = accountVaultPositionUpdate.id;
        accountVaultPosition.save();
  
    //   } else {
    //     // If they are the same, log the event and do not update
    //     log.info('Tx: {} has already been processed. Skipping.', [txHash]);
    //   }
    }
  
    return accountVaultPosition;
  }