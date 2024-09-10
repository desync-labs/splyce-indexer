import { BigDecimal, log, BigInt } from "@graphprotocol/graph-ts";
import { Protobuf } from 'as-proto/assembly';
import { VaultEvent } from "./pb/vault/events/v1/VaultEvent";
import {Token, Vault} from "../generated/schema";
import { VaultInitEvent } from "./pb/vault/events/v1/VaultInitEvent";

export function handleTransactions(bytes: Uint8Array): void {
    const vaultEvent: VaultEvent = Protobuf.decode<VaultEvent>(bytes, VaultEvent.decode);
    if (vaultEvent.vaultInitialize !== null) {
        log.info("VaultInitialize: {0}", [vaultEvent.vaultInitialize!.vaultIndex.toString()]);

        let vault = new Vault(vaultEvent.vaultInitialize!.vaultIndex.toString());
        vault.depositLimit = BigInt.fromU64(vaultEvent.vaultInitialize!.depositLimit);
        vault.shutdown = false;
        vault.totalDebt =    BigInt.fromI32(0);
        vault.totalIdle = BigInt.fromI32(0);
        vault.apr  =   BigDecimal.fromString("0");
        
        //Create token entity
        vault.token  =  getOrCreateTokenEntity(vaultEvent.vaultInitialize!).id;

        vault.save();
    }

}

function getOrCreateTokenEntity(vaultInitEvent: VaultInitEvent): Token {
    let token = Token.load(vaultInitEvent.underlyingMint.toString());
    if (token == null) {
        token = new Token(vaultInitEvent.underlyingMint.toString());
        token.decimals = vaultInitEvent.underlyingDecimals;
        token.symbol = ""; //TODO: Get symbol from mint
        token.name = "";   //TODO: Get name from mint 
        token.save();
    }

    return token as Token;
}