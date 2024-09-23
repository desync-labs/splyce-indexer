import { BigInt, log } from "@graphprotocol/graph-ts";
import { Strategy } from "../../generated/schema";
import { BIGINT_ZERO } from "../constants";
import { StrategyInitEvent } from "../pb/vault/events/v1/StrategyInitEvent";
import { VaultAddStrategyEvent } from "../pb/vault/events/v1/VaultAddStrategyEvent";
import { UpdatedCurrentDebtForStrategyEvent } from "../pb/vault/events/v1/UpdatedCurrentDebtForStrategyEvent";

import * as vaultLibrary from '../vault/vault';

export function getOrCreateStrategyEntity(strategyInitializeEvent: StrategyInitEvent): Strategy {
    let strategy = Strategy.load(strategyInitializeEvent.accountKey);
    if (strategy == null) {
        strategy = new Strategy(strategyInitializeEvent.accountKey);

        strategy.strategyType = strategyInitializeEvent.strategyType;
        strategy.amount =  BIGINT_ZERO;
        strategy.totalAssets =  BIGINT_ZERO;
        strategy.depositLimit =  BigInt.fromU64(strategyInitializeEvent.depositLimit);
        strategy.depositPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.depositPeriodEnds);
        strategy.lockPeriodEnds =  BigInt.fromI64(strategyInitializeEvent.lockPeriodEnds);
        strategy.currentDebt = BIGINT_ZERO
        strategy.maxDebt = BIGINT_ZERO

        strategy.vault = strategyInitializeEvent.vault;

        strategy.save()
    }

    return strategy as Strategy;
}

export function addStrategyToVault(strategyInitializeEvent: VaultAddStrategyEvent, vaultId: string): void {
    let strategy = Strategy.load(strategyInitializeEvent.strategyKey); 
    if(strategy != null){
        log.info("Strategy {} added to vault {1}",[strategy.id,vaultId]);
        strategy.maxDebt = BigInt.fromU64(strategyInitializeEvent.maxDebt);
        strategy.currentDebt = BigInt.fromU64(strategyInitializeEvent.currentDebt);
        strategy.vault = vaultId;
        strategy.save();
    }
}
export function updateCurrentDebt(strategyDebtUpdateEvent: UpdatedCurrentDebtForStrategyEvent, vaultIndex: string): void {
    let strategy = Strategy.load(strategyDebtUpdateEvent.strategyKey);
    if(strategy != null){
        strategy.currentDebt = BigInt.fromU64(strategyDebtUpdateEvent.newDebt);
        strategy.save();
    }

    vaultLibrary.updateDebt(vaultIndex,
                            strategyDebtUpdateEvent.totalDebt, 
                            strategyDebtUpdateEvent.totalIdle);
}

