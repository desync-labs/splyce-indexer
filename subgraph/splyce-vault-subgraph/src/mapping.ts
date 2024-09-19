import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import {Account, Deposit, Strategy, Token, Vault} from "../generated/schema";
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";
import { StrategyInitEvent } from "./pb/vault/events/v1/StrategyInitEvent";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        log.info("Initializing vault: {}", [vaultEvent.vaultInitialize!.vaultIndex.join("")]);
        getOrCreateVaultEntity(vaultEvent.vaultInitialize!)
    }else if(vaultEvent.strategyInitialize != null){
        log.info("Initializing strategy: {}", [vaultEvent.strategyInitialize!.accountKey.join("")]);
        getOrCreateStrategyEntity(vaultEvent.strategyInitialize!);
    }else if(vaultEvent.strategyAdd != null){
        log.info("Add strategy to vault {}",[vaultEvent.strategyAdd!.strategyKey.join("")]);
        let vault = Vault.load(vaultEvent.strategyAdd!.vaultIndex.join(""));
        if(vault != null){
           let strategy = Strategy.load(vaultEvent.strategyAdd!.strategyKey.join("")); 
           if(strategy != null){
                log.info("Strategy {} added to vault {1}",[strategy.id,vault.id]);
                strategy.vault = vault.id;
                strategy.save();
           }
        }
    }else if(vaultEvent.vaultDeposit != null){
        log.info("Deposit to vault {}",[vaultEvent.vaultDeposit!.vaultIndex.join("")]);
        createDepositEntity(vaultEvent);

        let vault = Vault.load(vaultEvent.vaultDeposit!.vaultIndex.join(""));
        if(vault != null){
            vault.totalIdle = vault.totalIdle.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.amount));
            vault.totalShare = vault.totalShare.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.share));
            vault.save();
        }

    }else if(vaultEvent.withdrwal != null){
        log.info("Withdrwal from vault {}",[vaultEvent.withdrwal!.vaultIndex.join("")]);
        let vault = Vault.load(vaultEvent.withdrwal!.vaultIndex.join(""));
        if(vault != null){
            vault.totalIdle = BigInt.fromU64(vaultEvent.withdrwal!.totalIdle);
            vault.totalShare =BigInt.fromU64(vaultEvent.withdrwal!.totalShare);
            vault.save();
        }
    }else if(vaultEvent.updateDepositLimit != null){
        log.info("Update deposit limit from vault {}",[vaultEvent.updateDepositLimit!.vaultIndex.join("")]);
        let vault = Vault.load(vaultEvent.updateDepositLimit!.vaultIndex.join(""));
        if(vault != null){
            vault.depositLimit = BigInt.fromU64(vaultEvent.updateDepositLimit!.newLimit);
            vault.save();
        }
    }else if(vaultEvent.strategyDeposit != null){
        log.info("Deposit to strategy {}",[vaultEvent.strategyDeposit!.accountKey.join("")]);
        let strategy = Strategy.load(vaultEvent.strategyDeposit!.accountKey.join(""));
        if(strategy != null){
            strategy.totalAssets = strategy.totalAssets.plus(BigInt.fromU64(vaultEvent.strategyDeposit!.totalAssets));
            strategy.save();
        }
    }else if(vaultEvent.strategyWithdraw != null){
        log.info("Withdraw from strategy {}",[vaultEvent.strategyWithdraw!.accountKey.join("")]);
        let strategy = Strategy.load(vaultEvent.strategyWithdraw!.accountKey.join(""));
        if(strategy != null){
            strategy.totalAssets = strategy.totalAssets.minus(BigInt.fromU64(vaultEvent.strategyWithdraw!.totalAssets));
            strategy.save();
        }
    }    
}

function createDepositEntity(vaultEvent: VaultEvent): Deposit {
    
    let account = Account.load(vaultEvent.vaultDeposit!.depositor.join(""));
    if (account == null) {
        account = new Account(vaultEvent.vaultDeposit!.depositor.join(""));
        account.save();
    }

    let deposit = new Deposit(vaultEvent.transactionHash);
    deposit.timestamp = BigInt.fromI64(vaultEvent.blockTimestamp);
    deposit.blockNumber = BigInt.fromI64(vaultEvent.blockHeight);
    deposit.account = vaultEvent.vaultDeposit!.depositor.join("");
    deposit.vault = vaultEvent.vaultDeposit!.vaultIndex.join("");
    deposit.tokenAmount = BigInt.fromU64(vaultEvent.vaultDeposit!.amount);
    deposit.sharesMinted = BigInt.fromU64(vaultEvent.vaultDeposit!.share);
    deposit.save();

    return deposit
}

function getOrCreateVaultEntity(vaultInitEvent: VaultInitEvent): Vault {
    let vault = Vault.load(vaultInitEvent.vaultIndex.join(""));
    if (vault == null) {
        vault = new Vault(vaultInitEvent.vaultIndex.join(""));
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
    let strategy = Strategy.load(strategyInitializeEvent.accountKey.join(""));
    if (strategy == null) {
        strategy = new Strategy(strategyInitializeEvent.accountKey.join(""));

        strategy.strategyType = strategyInitializeEvent.strategyType;
        strategy.amount =  BigInt.fromI64(0);
        strategy.totalAssets =  BigInt.fromI64(0);
        strategy.depositLimit =  BigInt.fromU64(strategyInitializeEvent.depositLimit);
        strategy.depositPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.depositPeriodEnds);
        strategy.lockPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.lockPeriodEnds);

        strategy.vault = strategyInitializeEvent.vault.join("");
        log.info("===========================",[]);
        log.info("Strategy added to vault {}",[strategyInitializeEvent.vault.join("")]);
        log.info("===========================",[]);

        strategy.save()
    }

    return strategy as Strategy;
}

function getOrCreateTokenEntity(vaultInitEvent: VaultInitEvent): Token {
    let token = Token.load(vaultInitEvent.underlyingMint.join(""));
    if (token == null) {
        token = new Token(vaultInitEvent.underlyingMint.join(""));
        token.decimals = vaultInitEvent.underlyingDecimals;
        token.symbol = ""; //TODO: Get symbol from mint
        token.name = "";   //TODO: Get name from mint 
        token.save();
    }

    return token as Token;
}