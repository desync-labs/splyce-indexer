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
import { StrategyReportedEvent } from "./pb/vault/events/v1/StrategyReportedEvent";
import {  SetPerformanceFeeEvent } from "./pb/vault/events/v1/SetPerformanceFeeEvent";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);

    if (vaultEvent.vaultInitialize !== null) {
        handleVaultInit(vaultEvent.vaultInitialize!, vaultEvent.blockTimestamp);
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
    }else if(vaultEvent.updatedDebtForStrategy != null){
        log.info("Updated debt for  strategy {}",[vaultEvent.updatedDebtForStrategy!.strategyKey]);
        handleCurrentDebtUpdate(vaultEvent.updatedDebtForStrategy!);
    }
    else if(vaultEvent.strategyReported != null){
        log.info('[Vault mappings] Handle strategy reported', []);
        handleStrategyReported(vaultEvent.strategyReported!,
                                vaultEvent.transactionHash,
                                vaultEvent.blockHeight,
                                vaultEvent.blockTimestamp
                            );
    }else if(vaultEvent.setPerformanceFee != null){
        log.info("Setting performance fee for strategy {}",[vaultEvent.setPerformanceFee!.accountKey]);
        handlePerformanceFeeUpdate(vaultEvent.setPerformanceFee!);
    }   

}


function handleVaultInit(vaultInitEvent: VaultInitEvent, blockTimestamp: i64): void {
   
    log.info("Initializing vault: {}", [vaultInitEvent.vaultIndex]);
    vaultLibrary.createVaultEntity(vaultInitEvent,blockTimestamp);
}

function handleDeposit(vaultEvent: VaultEvent): void {
    log.info("Deposit to vault {}",[vaultEvent.vaultDeposit!.vaultIndex]);
    vaultLibrary.deposit(vaultEvent);
}

function handleWithdraw(vaultEvent: VaultEvent): void {
    log.info("Handle withdrawal {}",[vaultEvent.withdrwal!.vaultIndex]);
    vaultLibrary.withdraw(vaultEvent);
}

function handleStrategyInit(vaultEvent: VaultEvent): void {
    log.info("Initializing strategy: {}", [vaultEvent.strategyInitialize!.accountKey]);
    strategyLiberary.getOrCreateStrategyEntity(vaultEvent.strategyInitialize!);
}

function handleStrategyAdd(vaultEvent: VaultEvent): void {
    log.info("Add strategy to vault {}",[vaultEvent.strategyAdd!.strategyKey]);
    strategyLiberary.addStrategyToVault(vaultEvent.strategyAdd!, vaultEvent.strategyAdd!.vaultIndex, vaultEvent.blockTimestamp);
}

function handleCurrentDebtUpdate(updatedDebtForStrategyEvent: UpdatedCurrentDebtForStrategyEvent): void {
    log.info("Add current debt for vault{} strategy {}",
                [updatedDebtForStrategyEvent.vaultIndex,
                    updatedDebtForStrategyEvent.strategyKey
    ]);

    strategyLiberary.updateCurrentDebt(updatedDebtForStrategyEvent);
}

export function handleStrategyReported(strategyReportedEvent: StrategyReportedEvent,txHash: string, blockNumber: u64, timestamp:i64): void {
    log.info('[Vault mappings] Handle strategy reported', []);
    strategyLiberary.createReport(
        txHash,
        strategyReportedEvent.strategyKey,
        BigInt.fromU64(strategyReportedEvent.gain),
        BigInt.fromU64(strategyReportedEvent.loss),
        BigInt.fromU64(strategyReportedEvent.currentDebt),
        BigInt.fromU64(strategyReportedEvent.protocolFees),
        BigInt.fromU64(strategyReportedEvent.totalFees),
        BigInt.fromU64(blockNumber),
        BigInt.fromI64(timestamp)
    );

}

function handlePerformanceFeeUpdate(performanceFeeEvent: SetPerformanceFeeEvent): void {
    log.info("Update peroformance fee for strategy {} with fee {}",
                [performanceFeeEvent.accountKey,
                    performanceFeeEvent.fee.toString()
    ]);

    strategyLiberary.updatePerformanceFee(performanceFeeEvent);
}