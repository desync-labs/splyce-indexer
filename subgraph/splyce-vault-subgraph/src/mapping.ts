import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import {Account, Deposit, ShareAccount, Strategy, Token, TokenAccount, Vault, Withdrawal} from "../generated/schema";
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";
import { StrategyInitEvent } from "./pb/vault/events/v1/StrategyInitEvent";

import * as vaultLibrary from './vault/vault';
import { updateAccountEntity } from "./account/account";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        log.info("Initializing vault: {}", [vaultEvent.vaultInitialize!.vaultIndex]);
        getOrCreateVaultEntity(vaultEvent.vaultInitialize!)
    }else if(vaultEvent.strategyInitialize != null){
        log.info("Initializing strategy: {}", [vaultEvent.strategyInitialize!.accountKey]);
        getOrCreateStrategyEntity(vaultEvent.strategyInitialize!);
    }else if(vaultEvent.strategyAdd != null){
        log.info("Add strategy to vault {}",[vaultEvent.strategyAdd!.strategyKey]);
        let vault = Vault.load(vaultEvent.strategyAdd!.vaultIndex);
        if(vault != null){
           let strategy = Strategy.load(vaultEvent.strategyAdd!.strategyKey); 
           if(strategy != null){
                log.info("Strategy {} added to vault {1}",[strategy.id,vault.id]);
                strategy.vault = vault.id;
                strategy.save();
           }
        }
    }else if(vaultEvent.vaultDeposit != null){
        handleDeposit(vaultEvent);
    }else if(vaultEvent.withdrwal != null){
        log.info("Withdrwal from vault {}",[vaultEvent.withdrwal!.vaultIndex]);
        createWithdrawEntity(vaultEvent);

        let vault = Vault.load(vaultEvent.withdrwal!.vaultIndex);
        if(vault != null){
            vault.totalIdle = BigInt.fromU64(vaultEvent.withdrwal!.totalIdle);
            vault.totalShare =BigInt.fromU64(vaultEvent.withdrwal!.totalShare);
            vault.save();
        }
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

function createWithdrawEntity(vaultEvent: VaultEvent): Withdrawal {
    
    updateAccountEntity(vaultEvent.withdrwal!.authority,
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

function getOrCreateVaultEntity(vaultInitEvent: VaultInitEvent): Vault {
    let vault = Vault.load(vaultInitEvent.vaultIndex);
    if (vault == null) {
        vault = new Vault(vaultInitEvent.vaultIndex);
        vault.depositLimit = BigInt.fromU64(vaultInitEvent.depositLimit);
        vault.shutdown = false;
        vault.totalDebt =    BigInt.fromI32(0);
        vault.totalIdle = BigInt.fromI32(0);
        vault.totalShare = BigInt.fromI32(0);
        vault.apr  =   BigDecimal.fromString("0");
        // vault.strategyIds = [];
        
        //Create token entity
        vault.token  =  getOrCreateTokenEntity(vaultInitEvent).id;
    
        vault.save();
    }
    return vault as Vault;
}

function getOrCreateStrategyEntity(strategyInitializeEvent: StrategyInitEvent): Strategy {
    let strategy = Strategy.load(strategyInitializeEvent.accountKey);
    if (strategy == null) {
        strategy = new Strategy(strategyInitializeEvent.accountKey);

        strategy.strategyType = strategyInitializeEvent.strategyType;
        strategy.amount =  BigInt.fromI64(0);
        strategy.totalAssets =  BigInt.fromI64(0);
        strategy.depositLimit =  BigInt.fromU64(strategyInitializeEvent.depositLimit);
        strategy.depositPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.depositPeriodEnds);
        strategy.lockPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.lockPeriodEnds);

        strategy.vault = strategyInitializeEvent.vault;

        strategy.save()
    }

    return strategy as Strategy;
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