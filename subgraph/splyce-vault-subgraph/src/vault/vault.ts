import { BigInt, log } from "@graphprotocol/graph-ts";
import { Deposit, Vault } from "../../generated/schema";
import { VaultEvent } from "../pb/vault/events/v1/VaultEvent";

import * as accountLibrary from '../account/account';
import * as vaultPosition from './vault-position';


export function deposit(vaultEvent: VaultEvent): void{

    log.info(
        '[Vault] Deposit vault: {} receiver: {} depositAmount: {} sharesMinted: {}',
        [
          vaultEvent.vaultDeposit!.vaultIndex,
          vaultEvent.vaultDeposit!.authority,
          vaultEvent.vaultDeposit!.amount.toString(),
          vaultEvent.vaultDeposit!.share.toString(),
        ]
      );

    let account = accountLibrary.updateAccountEntity(vaultEvent.vaultDeposit!.authority,
                        vaultEvent.vaultDeposit!.tokenAccount,
                        vaultEvent.vaultDeposit!.shareAccount);

    let deposit = new Deposit(vaultEvent.transactionHash);
    deposit.timestamp = BigInt.fromI64(vaultEvent.blockTimestamp);
    deposit.blockNumber = BigInt.fromI64(vaultEvent.blockHeight);
    deposit.account = vaultEvent.vaultDeposit!.authority;
    deposit.vault = vaultEvent.vaultDeposit!.vaultIndex;
    deposit.tokenAmount = BigInt.fromU64(vaultEvent.vaultDeposit!.amount);
    deposit.sharesMinted = BigInt.fromU64(vaultEvent.vaultDeposit!.share);
    deposit.save();

    let vault = Vault.load(vaultEvent.vaultDeposit!.vaultIndex);
    if(vault != null){
        vault.totalDebt = BigInt.fromU64(vaultEvent.vaultDeposit!.totalDebt);
        vault.totalIdle = BigInt.fromU64(vaultEvent.vaultDeposit!.amount);
        vault.totalShare = BigInt.fromU64(vaultEvent.vaultDeposit!.share);
        vault.save();

        vaultPosition.deposit(
            account,
            vault,
            vaultEvent.transactionHash,
            deposit.tokenAmount,
            deposit.sharesMinted,
            vault.totalShare,
            vault.totalDebt,
            vault.totalIdle,
            vault.totalShare
        )
    }

    

}