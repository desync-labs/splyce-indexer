import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import {Strategy, Token, Vault} from "../generated/schema";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";
import { StrategyInitEvent } from "./pb/vault/events/v1/StrategyInitEvent";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        log.info("Initializing vault: {0}", [vaultEvent.vaultInitialize!.vaultIndex.join("")]);

        let vault = new Vault(vaultEvent.vaultInitialize!.vaultIndex.join(""));
        vault.depositLimit = BigInt.fromU64(vaultEvent.vaultInitialize!.depositLimit);
        vault.shutdown = false;
        vault.totalDebt =    BigInt.fromI32(0);
        vault.totalIdle = BigInt.fromI32(0);
        vault.totalShare = BigInt.fromI32(0);
        vault.apr  =   BigDecimal.fromString("0");
        vault.strategyIds = [];
        
        //Create token entity
        vault.token  =  getOrCreateTokenEntity(vaultEvent.vaultInitialize!).id;

        vault.save();
    }else if(vaultEvent.strategyInitialize != null){
        log.info("Initializing strategy: {0}", [vaultEvent.strategyInitialize!.accountKey.join("")]);
        getOrCreateStrategyEntity(vaultEvent.strategyInitialize!);
    }else if(vaultEvent.strategyAdd != null){
        log.info("Add strategy to vault {0}",[vaultEvent.strategyAdd!.strategyKey.join("")]);
        let vault = Vault.load(vaultEvent.strategyAdd!.vaultIndex.join(""));
        if(vault != null){
           let strategy = Strategy.load(vaultEvent.strategyAdd!.strategyKey.join("")); 
           if(strategy != null){
                // vault.strategies.load().push(strategy);
                vault.strategyIds.push(strategy.id);
                vault.save();
           }
        }
    }else if(vaultEvent.vaultDeposit != null){
        log.info("Deposit to vault {0}",[vaultEvent.vaultDeposit!.vaultIndex.join("")]);
        let vault = Vault.load(vaultEvent.vaultDeposit!.vaultIndex.join(""));
        if(vault != null){
            vault.totalIdle = vault.totalIdle.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.amount));
            vault.totalShare = vault.totalIdle.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.share));
            vault.save();
        }
    }else if(vaultEvent.withdrwal != null){
        log.info("Withdrwal from vault {0}",[vaultEvent.withdrwal!.vaultIndex.join("")]);
        let vault = Vault.load(vaultEvent.withdrwal!.vaultIndex.join(""));
        if(vault != null){
            vault.totalIdle = BigInt.fromU64(vaultEvent.withdrwal!.totalIdle);
            vault.totalShare =BigInt.fromU64(vaultEvent.withdrwal!.totalShare);
            vault.save();
        }
    }else if(vaultEvent.updateDepositLimit != null){
        log.info("Update deposit limit from vault {0}",[vaultEvent.updateDepositLimit!.vaultIndex.join("")]);
        let vault = Vault.load(vaultEvent.updateDepositLimit!.vaultIndex.join(""));
        if(vault != null){
            vault.depositLimit = BigInt.fromU64(vaultEvent.updateDepositLimit!.newLimit);
            vault.save();
        }
    }else if(vaultEvent.strategyDeposit != null){
        log.info("Deposit to strategy {0}",[vaultEvent.strategyDeposit!.accountKey.join("")]);
        let strategy = Strategy.load(vaultEvent.strategyDeposit!.accountKey.join(""));
        if(strategy != null){
            strategy.totalFund = strategy.totalFund.plus(BigInt.fromU64(vaultEvent.strategyDeposit!.totalFunds));
            strategy.save();
        }
    }else if(vaultEvent.strategyWithdraw != null){
        log.info("Withdraw from strategy {0}",[vaultEvent.strategyWithdraw!.accountKey.join("")]);
        let strategy = Strategy.load(vaultEvent.strategyWithdraw!.accountKey.join(""));
        if(strategy != null){
            strategy.totalFund = strategy.totalFund.minus(BigInt.fromU64(vaultEvent.strategyWithdraw!.totalFunds));
            strategy.save();
        }
    }    
}

function getOrCreateStrategyEntity(strategyInitializeEvent: StrategyInitEvent): Strategy {
    let strategy = Strategy.load(strategyInitializeEvent.accountKey.join(""));
    if (strategy == null) {
        strategy = new Strategy(strategyInitializeEvent.accountKey.join(""));
        strategy.strategyType = strategyInitializeEvent.strategyType;
        strategy.totalIdle =  BigInt.fromI64(strategyInitializeEvent.totalIdle);
        strategy.totalFund =  BigInt.fromI64(strategyInitializeEvent.totalFunds);
        strategy.depositLimit =  BigInt.fromI64(strategyInitializeEvent.depositLimit);
        strategy.depositPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.depositPeriodEnds);
        strategy.lockPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.lockPeriodEnds);
        strategy.save()
    }

    return strategy as Strategy;
}

function getOrCreateTokenEntity(vaultInitEvent: VaultInitEvent): Token {
    let token = Token.load(vaultInitEvent.underlyingMint.join(""));
    if (token == null) {
        token = new Token(vaultInitEvent.underlyingMint.toString());
        token.decimals = vaultInitEvent.underlyingDecimals;
        token.symbol = ""; //TODO: Get symbol from mint
        token.name = "";   //TODO: Get name from mint 
        token.save();
    }

    return token as Token;
}