// import { Vault } from "../../generated/schema";

// export function updateVaultDayData(
//     txHash: string,
//     vault: Vault,
//     vaultUpdate: VaultUpdate
//   ): void {
//     let timestamp = transaction.timestamp;
//     let dayIndex = getDayIndexFromTimestamp(timestamp);
//     let vaultDayID = getDayIDFromIndex(vault.id, dayIndex);
  
//     let vaultDayData = VaultDayData.load(vaultDayID);
//     if (vaultDayData === null) {
//       log.debug('[VaultDayData] No VaultDayData found for {}. Creating.', [
//         vaultDayID,
//       ]);
//       vaultDayData = new VaultDayData(vaultDayID);
//       vaultDayData.timestamp = getDayStartTimestamp(timestamp);
//       vaultDayData.vault = vault.id;
//       vaultDayData.pricePerShare = vaultUpdate.pricePerShare;
//       vaultDayData.deposited = BIGINT_ZERO;
//       vaultDayData.withdrawn = BIGINT_ZERO;
//       vaultDayData.totalReturnsGenerated = BIGINT_ZERO;
//       vaultDayData.totalReturnsGeneratedUSDC = BIGINT_ZERO;
//       vaultDayData.dayReturnsGenerated = BIGINT_ZERO;
//       vaultDayData.dayReturnsGeneratedUSDC = BIGINT_ZERO;
//       vaultDayData.blockNumber = transaction.blockNumber;
//       log.debug('[VaultDayData] VaultDayData object built.', []);
//     }
  
//     log.debug('[VaultDayData] Resolving token price for {}', [vault.token]);
  
//     vaultDayData.pricePerShare = vaultUpdate.pricePerShare;
//     vaultDayData.deposited = vaultDayData.deposited.plus(
//       vaultUpdate.tokensDeposited
//     );
//     vaultDayData.withdrawn = vaultDayData.withdrawn.plus(
//       vaultUpdate.tokensWithdrawn
//     );
//     vaultDayData.dayReturnsGenerated = vaultDayData.dayReturnsGenerated.plus(
//       vaultUpdate.returnsGenerated
//     );
  
//     let underlying = Token.load(vault.token);
//     // @ts-ignore
//     let u8Decimals = u8(underlying!.decimals);
//     let priceDivisor = BigInt.fromI32(10).pow(u8Decimals);
  
//     log.debug(
//       '[VaultDayData] Basic data fields resolved, moving on to historical fields.',
//       []
//     );
  
//     // Multiple days can pass between a vaultDayData being posted, so we look up to maxSearchDepth days in the past.
//     // In the future, should use a better approach.
//     let daysInPast = 1;
//     let maxSearchDepth = 100;
//     while (daysInPast <= maxSearchDepth) {
//       let dayToCheck: string = getDayIDFromIndex(
//         vault.id,
//         dayIndex.minus(BigInt.fromI32(daysInPast))
//       );
//       let previousVaultDayData = VaultDayData.load(dayToCheck);
//       if (previousVaultDayData !== null) {
//         log.info(
//           "[INFO] Adding previous day's totalReturnsGenerated to today's: {} {}",
//           [
//             previousVaultDayData.totalReturnsGenerated.toString(),
//             vaultDayData.dayReturnsGenerated.toString(),
//           ]
//         );
//         vaultDayData.totalReturnsGenerated = previousVaultDayData.totalReturnsGenerated.plus(
//           vaultDayData.dayReturnsGenerated
//         );
//         break;
//       } else {
//         daysInPast += 1;
//         if (daysInPast > maxSearchDepth) {
//           log.warning(
//             "[WARN] No VaultDayData found for this vault at {} days into past. Initializing totalReturnsGenerated with today's earnings.",
//             [maxSearchDepth.toString()]
//           );
//           vaultDayData.totalReturnsGenerated = vaultDayData.dayReturnsGenerated;
//         }
//       }
//     }
  
//     vaultDayData.save();
//   }
  
//   export function getDayIndexFromTimestamp(timestamp: BigInt): BigInt {
//     return timestamp.div(BigInt.fromI32(86400000));
//   }
  
//   export function getDayIDFromIndex(vaultID: string, dayID: BigInt): string {
//     return (
//       vaultID
//         .toString()
//         .concat('-')
//         // @ts-ignore
//         .concat(dayID.toString())
//     );
//   }
  
//   function getDayStartTimestamp(timestamp: BigInt): BigInt {
//     let milliSecsInDay = BigInt.fromI32(86400000);
//     return timestamp.div(milliSecsInDay).times(milliSecsInDay);
//   }