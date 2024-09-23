import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import {Strategy, Token, Vault, Withdrawal} from "../generated/schema";
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";
import { StrategyInitEvent } from "./pb/vault/events/v1/StrategyInitEvent";

import * as vaultLibrary from './vault/vault';
import { BIGINT_ZERO } from "./constants";
import * as strategyLiberary from './strategy/strategy';

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        getOrCreateVaultEntity(vaultEvent.vaultInitialize!)
    }else if(vaultEvent.strategyInitialize != null){
        handleStrategyInit(vaultEvent);
    }else if(vaultEvent.strategyAdd != null){
        handleStrategyAdd(vaultEvent);
    }else if(vaultEvent.vaultDeposit != null){
        handleDeposit(vaultEvent);
    }else if(vaultEvent.withdrwal != null){
        log.info("Withdrwal from vault {}",[vaultEvent.withdrwal!.vaultIndex]);
        handleWithdraw(vaultEvent);
    }else if(vaultEvent.updateDepositLimit != null){
        log.info("Update deposit limit from vault {}",[vaultEvent.updateDepositLimit!.vaultIndex]);
        let vault = Vault.load(vaultEvent.updateDepositLimit!.vaultIndex);
        if(vault != null){
            vault.depositLimit = BigInt.fromU64(vaultEvent.updateDepositLimit!.newLimit);
            vault.save();
        }
    }else if(vaultEvent.strategyDeposit != null){
        log.info("Deposit to strategy {}",[vaultEvent.strategyDeposit!.accountKey]);
        let strategy = Strategy.load(vaultEvent.strategyDeposit!.accountKey);
        if(strategy != null){
            strategy.totalAssets = strategy.totalAssets.plus(BigInt.fromU64(vaultEvent.strategyDeposit!.totalAssets));
            strategy.save();
        }
    }else if(vaultEvent.strategyWithdraw != null){
        log.info("Withdraw from strategy {}",[vaultEvent.strategyWithdraw!.accountKey]);
        let strategy = Strategy.load(vaultEvent.strategyWithdraw!.accountKey);
        if(strategy != null){
            strategy.totalAssets = strategy.totalAssets.minus(BigInt.fromU64(vaultEvent.strategyWithdraw!.totalAssets));
            strategy.save();
        }
    }    
}

function getOrCreateVaultEntity(vaultInitEvent: VaultInitEvent): Vault {
    log.info("Initializing vault: {}", [vaultInitEvent.vaultIndex]);
    let vault = Vault.load(vaultInitEvent.vaultIndex);
    if (vault == null) {
        vault = new Vault(vaultInitEvent.vaultIndex);
        vault.depositLimit = BigInt.fromU64(vaultInitEvent.depositLimit);
        vault.shutdown = false;
        vault.totalDebt =    BIGINT_ZERO;
        vault.totalIdle = BIGINT_ZERO;
        vault.totalShare = BIGINT_ZERO;
        vault.apr  =   BigDecimal.fromString("0");
        // vault.strategyIds = [];
        
        //Create token entity
        vault.token  =  getOrCreateTokenEntity(vaultInitEvent).id;

        vault.balanceTokens = BIGINT_ZERO;
        vault.balanceTokensIdle = BIGINT_ZERO;
        vault.sharesSupply = BIGINT_ZERO;
    
        vault.save();
    }
    return vault as Vault;
}

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

function handleDeposit(vaultEvent: VaultEvent): void {
    log.info("Deposit to vault {}",[vaultEvent.vaultDeposit!.vaultIndex]);
    vaultLibrary.deposit(vaultEvent);
}

function handleWithdraw(vaultEvent: VaultEvent): void {
    log.info("Handle withdrawal {}",[vaultEvent.vaultDeposit!.vaultIndex]);
    vaultLibrary.withdraw(vaultEvent);
}

function handleStrategyInit(vaultEvent: VaultEvent): void {
    log.info("Initializing strategy: {}", [vaultEvent.strategyInitialize!.accountKey]);
    strategyLiberary.getOrCreateStrategyEntity(vaultEvent.strategyInitialize!);
}

function handleStrategyAdd(vaultEvent: VaultEvent): void {
    log.info("Add strategy to vault {}",[vaultEvent.strategyAdd!.strategyKey]);
    strategyLiberary.addStrategyToVault(vaultEvent.strategyAdd!, vaultEvent.strategyAdd!.vaultIndex);
}
