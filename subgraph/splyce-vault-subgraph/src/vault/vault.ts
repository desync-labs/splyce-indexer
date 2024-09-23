import { BigInt, log } from "@graphprotocol/graph-ts";
import { Deposit, Token, Vault, Withdrawal } from "../../generated/schema";
import { VaultEvent } from "../pb/vault/events/v1/VaultEvent";

import * as accountLibrary from '../account/account';
import * as vaultPosition from './vault-position';
import { VaultInitEvent } from "../pb/vault/events/v1/VaultInitEvent";
import { BIGDECIMAL_ZERO, BIGINT_ZERO } from "../constants";

export function createVaultEntity(vaultInitEvent: VaultInitEvent): void {
    let vault = Vault.load(vaultInitEvent.vaultIndex);
    if (vault == null) {
        vault = new Vault(vaultInitEvent.vaultIndex);
        vault.depositLimit = BigInt.fromU64(vaultInitEvent.depositLimit);
        vault.shutdown = false;
        vault.totalDebt =    BIGINT_ZERO;
        vault.totalIdle = BIGINT_ZERO;
        vault.totalShare = BIGINT_ZERO;

        //TODO: Will be updated with report processing logic
        vault.apr  =   BIGDECIMAL_ZERO;
        // vault.strategyIds = [];
        
        //Create token entity
        vault.token  =  getOrCreateTokenEntity(vaultInitEvent).id;

        vault.balanceTokens = BIGINT_ZERO;
        vault.balanceTokensIdle = BIGINT_ZERO;
        vault.sharesSupply = BIGINT_ZERO;
    
        vault.save();
    }
}

//TODO: Move this to token library
function getOrCreateTokenEntity(vaultInitEvent: VaultInitEvent): Token {
    let token = Token.load(vaultInitEvent.underlyingMint);
    if (token == null) {
        token = new Token(vaultInitEvent.underlyingMint);
        token.decimals = vaultInitEvent.underlyingDecimals;
        token.symbol = ""; //TODO: Get symbol from mint
        token.name = "";   //TODO: Get name from mint 
        token.save();
    }

    return token as Token;
}

export function deposit(vaultEvent: VaultEvent): void{

    log.info(
        '[Vault] Deposit vault: {} receiver: {} depositAmount: {} sharesMinted: {}',
        [
          vaultEvent.vaultDeposit!.vaultIndex,
          vaultEvent.vaultDeposit!.authority,
          vaultEvent.vaultDeposit!.amount.toString(),
          vaultEvent.vaultDeposit!.share.toString(),
        ]
      );

    let account = accountLibrary.updateAccountEntity(vaultEvent.vaultDeposit!.authority,
                        vaultEvent.vaultDeposit!.tokenAccount,
                        vaultEvent.vaultDeposit!.shareAccount);

    let deposit = new Deposit(vaultEvent.transactionHash);
    deposit.timestamp = BigInt.fromI64(vaultEvent.blockTimestamp);
    deposit.blockNumber = BigInt.fromI64(vaultEvent.blockHeight);
    deposit.account = vaultEvent.vaultDeposit!.authority;
    deposit.vault = vaultEvent.vaultDeposit!.vaultIndex;
    deposit.tokenAmount = BigInt.fromU64(vaultEvent.vaultDeposit!.amount);
    deposit.sharesMinted = BigInt.fromU64(vaultEvent.vaultDeposit!.share);
    deposit.save();

    let vault = Vault.load(vaultEvent.vaultDeposit!.vaultIndex);
    if(vault != null){
        vault.totalDebt = BigInt.fromU64(vaultEvent.vaultDeposit!.totalDebt);
        vault.totalIdle = BigInt.fromU64(vaultEvent.vaultDeposit!.amount);
        vault.totalShare = BigInt.fromU64(vaultEvent.vaultDeposit!.share);

        //TODO: This is as per fathom logic.. 
        //will be break out into another common function when vaultUpdate entity is ready
        vault.balanceTokensIdle = vault.totalIdle;
        //TODO: Rcheck this logic, we dont have share token on vault entity/vault on-chain program
        //logic from fathom vault subgraph 
        //let balanceTokens: BigInt = getTotalAssets(Address.fromString(vault.shareToken));
        vault.balanceTokens = vault.totalDebt.plus(vault.totalIdle);

        vault.save();

        vaultPosition.deposit(
            account,
            vault,
            vaultEvent.transactionHash,
            deposit.tokenAmount,
            deposit.sharesMinted,
            vault.totalShare,
            vault.totalDebt,
            vault.totalIdle,
            vault.totalShare
        )
    }
}

export function withdraw(vaultEvent: VaultEvent): void {
    createWithdrawEntity(vaultEvent);

    let vault = Vault.load(vaultEvent.withdrwal!.vaultIndex);
    if(vault != null){
        vault.totalIdle = BigInt.fromU64(vaultEvent.withdrwal!.totalIdle);
        vault.totalShare =BigInt.fromU64(vaultEvent.withdrwal!.totalShare);

        //TODO: This is as per fathom logic.. 
        //will be break out into another common function when vaultUpdate entity is ready
        vault.balanceTokensIdle = vault.totalIdle;
        vault.save();
    }
}

function createWithdrawEntity(vaultEvent: VaultEvent): Withdrawal {
    
    accountLibrary.updateAccountEntity(vaultEvent.withdrwal!.authority,
                        vaultEvent.withdrwal!.tokenAccount,
                        vaultEvent.withdrwal!.shareAccount);

    let withdrwal = new Withdrawal(vaultEvent.transactionHash);
    withdrwal.timestamp = BigInt.fromI64(vaultEvent.blockTimestamp);
    withdrwal.blockNumber = BigInt.fromI64(vaultEvent.blockHeight);
    withdrwal.account = vaultEvent.withdrwal!.authority;
    withdrwal.vault = vaultEvent.withdrwal!.vaultIndex;
    withdrwal.tokenAmount = BigInt.fromU64(vaultEvent.withdrwal!.assetsToTransfer);
    withdrwal.sharesBurnt = BigInt.fromU64(vaultEvent.withdrwal!.sharesToBurn);

    withdrwal.save();

    return withdrwal
}

export function updateDebt(vaultId: string,
                            totalDebt: number,
                            totalIdle: number,
): void {
    let vault = Vault.load(vaultId);
    if(vault != null){
        vault.totalDebt = BigInt.fromU64(totalDebt);
        vault.totalIdle = BigInt.fromU64(totalIdle);
        vault.save();
    }
}




