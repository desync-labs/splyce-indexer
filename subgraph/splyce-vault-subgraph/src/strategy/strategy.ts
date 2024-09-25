import { BigInt, log } from "@graphprotocol/graph-ts";
import { Strategy, StrategyReport } from "../../generated/schema";
import { BIGDECIMAL_ZERO, BIGINT_ZERO } from "../constants";
import { StrategyInitEvent } from "../pb/vault/events/v1/StrategyInitEvent";
import { VaultAddStrategyEvent } from "../pb/vault/events/v1/VaultAddStrategyEvent";
import { UpdatedCurrentDebtForStrategyEvent } from "../pb/vault/events/v1/UpdatedCurrentDebtForStrategyEvent";

import * as vaultLibrary from '../vault/vault';
import * as strategyReportLibrary from './strategy-report';
import * as strategyReportResultLibrary from './strategy-report-result';


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

        //TODO: Will be updated with report processing logic
        strategy.apr = BIGDECIMAL_ZERO;

        strategy.vault = strategyInitializeEvent.vault;
        strategy.delegatedAssets = BIGINT_ZERO;
        strategy.activation = BIGINT_ZERO;
        strategy.reportsCount = BIGDECIMAL_ZERO;

        strategy.save()
    }

    return strategy as Strategy;
}

export function addStrategyToVault(strategyInitializeEvent: VaultAddStrategyEvent, vaultId: string, blockTimestamp:i64): void {
    let strategy = Strategy.load(strategyInitializeEvent.strategyKey); 
    if(strategy != null){
        log.info("Strategy {} added to vault {1}",[strategy.id,vaultId]);
        strategy.maxDebt = BigInt.fromU64(strategyInitializeEvent.maxDebt);
        strategy.currentDebt = BigInt.fromU64(strategyInitializeEvent.currentDebt);
        strategy.vault = vaultId;
        strategy.activation = BigInt.fromI64(blockTimestamp);
        strategy.save();
    }
}
export function updateCurrentDebt(strategyDebtUpdateEvent: UpdatedCurrentDebtForStrategyEvent): void {
    let strategy = Strategy.load(strategyDebtUpdateEvent.strategyKey);
    if(strategy != null){
        strategy.currentDebt = BigInt.fromU64(strategyDebtUpdateEvent.newDebt);
        strategy.save();
    }

    vaultLibrary.updateDebt(strategyDebtUpdateEvent.vaultIndex,
                            strategyDebtUpdateEvent.totalDebt, 
                            strategyDebtUpdateEvent.totalIdle);
}

export function createReport(
    txHash: string,
    strategyId: string,
    gain: BigInt,
    loss: BigInt,
    currentDebt: BigInt,
    protocolFees: BigInt,
    totalFees: BigInt,
    blockNumber: BigInt,
    blockTimestamp: BigInt
  ): StrategyReport | null {
    log.info('[Strategy] Create report for strategy {}', [strategyId]);
    let strategy = Strategy.load(strategyId);
    if (strategy !== null) {
      let currentReportId = strategy.latestReport;
      log.info(
        '[Strategy] Getting current report {} for strategy {}. TxHash: {}',
        [currentReportId ? currentReportId : 'null', strategy.id, txHash]
      );
      if (gain > BIGINT_ZERO || loss < BIGINT_ZERO) {
        log.info(
          '[Strategy] Create new report for strategy {}. TxHash: {}',
          [strategy.id, txHash]
        );
        let latestReport = strategyReportLibrary.getOrCreate(
          txHash,
          strategy as Strategy,
          gain,
          loss,
          currentDebt,
          protocolFees,
          totalFees,
          blockNumber,
          blockTimestamp
        );
        strategy.latestReport = latestReport.id;
        strategy.save();
  
        // Getting latest report to compare to the new one and create a new report result.
        if (currentReportId !== null) {
          let currentReport = StrategyReport.load(currentReportId);
          if (currentReport !== null) {
            log.info(
              '[Strategy] Create report result (latest {} vs current {}) for strategy {}. TxHash: {}',
              [latestReport.id, currentReport.id, strategyId, txHash]
            );
            strategyReportResultLibrary.create(
              txHash,
              currentReport as StrategyReport,
              latestReport,
              blockNumber,
              blockTimestamp
            );
          }
        } else {
          log.info(
            '[Strategy] Report result NOT created. Only one report created {} for strategy {}. TxHash: {}',
            [latestReport.id, strategyId, txHash]
          );
        }
        return latestReport;
      } else {
        return null;
      }
  
    } else {
      log.warning(
        '[Strategy] Failed to load strategy {} while handling StrategyReport',
        [strategyId]
      );
      return null;
    }
  }