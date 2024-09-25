import { BigInt, log } from "@graphprotocol/graph-ts";
import { Strategy, StrategyReport } from "../../generated/schema";

export function getOrCreate(
    transactionHash: string,
    strategy: Strategy,
    gain: BigInt,
    loss: BigInt,
    currentDebt: BigInt,
    protocolFees: BigInt,
    totalFees: BigInt,
    blockNumber: BigInt,
    blockTimestamp: BigInt
  ): StrategyReport {
    log.info('[StrategyReport] Get or create strategy report', []);
  
    let strategyReportId = transactionHash + '-' + strategy.id;
    let strategyReport = StrategyReport.load(strategyReportId);
    if (strategyReport === null) {
      strategyReport = new StrategyReport(strategyReportId);
      strategyReport.strategy = strategy.id;
      strategyReport.blockNumber = blockNumber;
      strategyReport.timestamp = blockTimestamp;
    //   strategyReport.transaction = transactionId;
    strategyReport.transactionHash = transactionHash;
      strategyReport.gain = gain;
      strategyReport.loss = loss;
      strategyReport.currentDebt = currentDebt;
      strategyReport.protocolFees = protocolFees;
      strategyReport.totalFees = totalFees;
      strategyReport.save();
    }
    return strategyReport as StrategyReport;
  }