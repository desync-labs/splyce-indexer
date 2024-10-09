import { BigInt, log } from "@graphprotocol/graph-ts";
import { Account, AccountVaultPosition, Token, Vault } from "../../generated/schema";
import { BIGINT_ZERO } from "../constants";

export function buildId(account: Account, vault: Vault): string {
    return account.id.concat('-').concat(vault.id);
}

export function deposit(
    account: Account,
    vault: Vault,
    txHash: string,
    depositedTokens: BigInt,
    receivedShares: BigInt,
    balanceShares: BigInt,
    total_debt: BigInt,
    total_idle: BigInt,
    total_shares: BigInt
  ): void {
    log.debug('[VaultPosition] Deposit', []);
    let vaultPositionId = buildId(account, vault);
    let accountVaultPosition = AccountVaultPosition.load(vaultPositionId);
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
      accountVaultPosition.balanceTokens = depositedTokens;
      accountVaultPosition.balanceShares = balanceShares;
      accountVaultPosition.balanceProfit = BIGINT_ZERO;
  
      accountVaultPosition.balancePosition = balancePosition;
      accountVaultPosition.save();
    } else {
      log.info('Tx: {} Account vault position {} found. Using it.', [
        txHash,
        vaultPositionId,
      ]);
    }

      accountVaultPosition.balanceTokens = accountVaultPosition.balanceTokens.plus(depositedTokens);
      accountVaultPosition.balanceShares = balanceShares;

      accountVaultPosition.balancePosition = balancePosition;
      accountVaultPosition.save();

  }

  export function withdraw(
    account: Account,
    vault: Vault,
    txHash: string,
    withdrawnAmount: BigInt,
    sharesBurnt: BigInt,
    balanceShares: BigInt,
    total_debt: BigInt,
    total_idle: BigInt,
    total_shares: BigInt
  ): void {
    let token = Token.load(vault.token) as Token;

    let pricePerShare = BIGINT_ZERO
    if(total_shares.gt(BIGINT_ZERO)) 
    {
        pricePerShare =  total_debt.plus(total_idle).div(total_shares); 
    }else{
        log.info('total_shares is zero!.', []);
        pricePerShare =  total_debt.plus(total_idle);
    }

    if(token.decimals != 0){
        log.info('Token decimals is zero!.', []);
        return;
    }

    let balancePosition = balanceShares.times(pricePerShare).div(BigInt.fromI32(token.decimals));

    let vaultPositionId = buildId(account, vault);
    let accountVaultPosition = AccountVaultPosition.load(vaultPositionId);

    if (accountVaultPosition != null) {
      accountVaultPosition.balanceShares = balanceShares;
      accountVaultPosition.balanceTokens = getBalanceTokens(
        accountVaultPosition.balanceTokens,
        withdrawnAmount
      );

      accountVaultPosition.balanceProfit = getBalanceProfit(
        accountVaultPosition.balanceShares,
        accountVaultPosition.balanceProfit,
        accountVaultPosition.balanceTokens,
        withdrawnAmount
      );
      accountVaultPosition.balancePosition = balancePosition;
      accountVaultPosition.save();
  
    } 
    
  }

  function getBalanceProfit(
    currentSharesBalance: BigInt,
    currentProfit: BigInt,
    currentAmount: BigInt,
    withdrawAmount: BigInt
  ): BigInt {
    if (currentSharesBalance.isZero()) {
      // User withdrawn all the shares, so we can calculate the profit or losses.
      if (withdrawAmount.gt(currentAmount)) {
        // User has profits.
        return currentProfit.plus(withdrawAmount.minus(currentAmount));
      } else if (withdrawAmount.lt(currentAmount)) {
        // User has losses.
        return currentProfit.minus(currentAmount.minus(withdrawAmount));
      } else {
        // User has no profits or losses.
        return currentProfit;
      }
    }
    // User still have shares, so we returns the current profit.
    return currentProfit;
  }

  function getBalanceTokens(current: BigInt, withdraw: BigInt): BigInt {
    return withdraw.gt(current) ? BIGINT_ZERO : current.minus(withdraw);
  }