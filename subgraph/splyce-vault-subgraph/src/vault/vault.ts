import { BigInt } from "@graphprotocol/graph-ts";
import { Deposit, Vault } from "../../generated/schema";
import { VaultEvent } from "../pb/vault/events/v1/VaultEvent";

import * as accountLibrary from '../account/account';

export function deposit(vaultEvent: VaultEvent): void{
    accountLibrary.updateAccountEntity(vaultEvent.vaultDeposit!.authority,
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
        vault.totalIdle = vault.totalIdle.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.amount));
        vault.totalShare = vault.totalShare.plus(BigInt.fromU64(vaultEvent.vaultDeposit!.share));
        vault.save();
    }

}