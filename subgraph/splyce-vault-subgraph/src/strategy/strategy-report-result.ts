import { BigDecimal, BigInt, log } from "@graphprotocol/graph-ts";
import { Strategy, StrategyHistoricalApr, StrategyReport, StrategyReportResult, Vault, VaultHistoricalApr } from "../../generated/schema";
import { BIGDECIMAL_ZERO, DAYS_PER_YEAR, MS_PER_DAY } from "../constants";

export function create(
    txHash: string,
    previousReport: StrategyReport,
    currentReport: StrategyReport,
    blockNumber: BigInt,
    blockTimestamp: BigInt
  ): StrategyReportResult | null{
    if (currentReport.id == previousReport.id) {
      log.info(
        '[StrategyReportResult] Previous report {} and current report {} are the same. No strategy report result created.',
        [previousReport.id, currentReport.id]
      );
      return null;
    } else {
      log.info(
        '[StrategyReportResult] Create strategy report result between previous {} and current report {}. Strategy {} TxHash: {}',
        [previousReport.id, currentReport.id, currentReport.strategy, txHash]
      );
  
      let id = txHash + '-' + currentReport.id + '-' + previousReport.id;
      let strategyReportResult = new StrategyReportResult(id);
      strategyReportResult.timestamp = blockTimestamp;
      strategyReportResult.blockNumber = blockNumber;
      strategyReportResult.currentReport = currentReport.id;
      strategyReportResult.previousReport = previousReport.id;
      strategyReportResult.startTimestamp = previousReport.timestamp;
      strategyReportResult.endTimestamp = currentReport.timestamp;
      strategyReportResult.duration = currentReport.timestamp
        .toBigDecimal()
        .minus(previousReport.timestamp.toBigDecimal());
      strategyReportResult.durationPr = BIGDECIMAL_ZERO;
      strategyReportResult.apr = BIGDECIMAL_ZERO;
      strategyReportResult.transactionHash = txHash;
  
      let profit = currentReport.gain.plus(currentReport.loss);
      let msInDays = strategyReportResult.duration.div(MS_PER_DAY);
      log.info(
        '[StrategyReportResult] Report Result - Start / End: {} / {} - Duration: {} (days {}) - Profit: {} - TxHash: {}',
        [
          strategyReportResult.startTimestamp.toString(),
          strategyReportResult.endTimestamp.toString(),
          strategyReportResult.duration.toString(),
          msInDays.toString(),
          profit.toString(),
          txHash,
        ]
      );
  
      if (!previousReport.currentDebt.isZero() && !msInDays.equals(BIGDECIMAL_ZERO)) {
        let profitOverTotalDebt = profit
          .toBigDecimal()
          .div(previousReport.currentDebt.toBigDecimal());
        strategyReportResult.durationPr = profitOverTotalDebt;
        let yearOverDuration = DAYS_PER_YEAR.div(msInDays);
        let apr = profitOverTotalDebt.times(yearOverDuration).times(BigDecimal.fromString('100'));
  
        log.info(
          '[StrategyReportResult] Report Result - Duration: {} ms / {} days - Duration (Year): {} - Profit / Total Debt: {} / APR: {} - TxHash: {}',
          [
            strategyReportResult.duration.toString(),
            msInDays.toString(),
            yearOverDuration.toString(),
            profitOverTotalDebt.toString(),
            apr.toString(),
            txHash,
          ]
        );
        strategyReportResult.apr = apr;
      }
  
      let strategy = Strategy.load(currentReport.strategy);
      let vault: Vault | null = null;
      if (strategy != null) {
        vault = Vault.load(strategy.vault!);
      }

      if(strategy != null && vault != null){
          let reportCount = strategy.reportsCount;
          let numeratorVault = (vault.apr).plus(strategyReportResult.apr);
          let numeratorStrategy = (strategy.apr).plus(strategyReportResult.apr);
          let vaultApr: BigDecimal;
          let strategyApr: BigDecimal;
          // let vaultsHistoricalApr = vault.historicalApr;
          // let strategyHistoricalApr = strategy.historicalApr;      
          if (reportCount.equals(BIGDECIMAL_ZERO)) {
            vaultApr = numeratorVault      
            strategyApr = numeratorStrategy;           
          } else {
            vaultApr = numeratorVault.div(reportCount);
            strategyApr = numeratorStrategy.div(reportCount);
          }
          vault.apr = vaultApr;
          strategy.apr = strategyApr;
          let newVaultHistoricalApr = new VaultHistoricalApr(id);
          newVaultHistoricalApr.timestamp = currentReport.timestamp;
          newVaultHistoricalApr.apr = vaultApr;
          newVaultHistoricalApr.vault = vault.id;
          let newStrategyHistoricalApr = new StrategyHistoricalApr(id);
          newStrategyHistoricalApr.timestamp = currentReport.timestamp;
          newStrategyHistoricalApr.apr = strategyApr;
          newStrategyHistoricalApr.strategy = strategy.id;
          //Add the strates addr to the vaults withdrawlQueue
          newVaultHistoricalApr.save();
          newStrategyHistoricalApr.save();
          strategy.reportsCount = reportCount.plus(BigDecimal.fromString('1'));
          strategy.save();
          vault.save();
      
      }

   strategyReportResult.save();
      return strategyReportResult;
    }
  }