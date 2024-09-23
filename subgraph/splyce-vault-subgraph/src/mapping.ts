import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import {Strategy, Token, Vault, Withdrawal} from "../generated/schema";
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";
import { StrategyInitEvent } from "./pb/vault/events/v1/StrategyInitEvent";

import * as vaultLibrary from './vault/vault';
import { BIGDECIMAL_ZERO, BIGINT_ZERO } from "./constants";
import * as strategyLiberary from './strategy/strategy';
import { UpdatedCurrentDebtForStrategyEvent } from "./pb/vault/events/v1/UpdatedCurrentDebtForStrategyEvent";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        handleVaultInit(vaultEvent.vaultInitialize!);
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


function handleVaultInit(vaultInitEvent: VaultInitEvent): void {
   
    log.info("Initializing vault: {}", [vaultInitEvent.vaultIndex]);
    vaultLibrary.createVaultEntity(vaultInitEvent);
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

function handleCurrentDebtUpdate(vaultEvent: VaultEvent): void {
    log.info("Add current debt for vault{} strategy {}",
                [vaultEvent.updatedDebtForStrategy!.vaultIndex,
                vaultEvent.updatedDebtForStrategy!.strategyKey
    ]);

    strategyLiberary.updateCurrentDebt(vaultEvent.updatedDebtForStrategy!, vaultEvent.updatedDebtForStrategy!.vaultIndex);
}