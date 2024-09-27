// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.0

import { Writer, Reader } from "as-proto/assembly";

export class VaultInitEvent {
  static encode(message: VaultInitEvent, writer: Writer): void {
    writer.uint32(10);
    writer.string(message.vaultIndex);

    writer.uint32(18);
    writer.string(message.underlyingMint);

    writer.uint32(26);
    writer.string(message.underlyingTokenAcc);

    writer.uint32(32);
    writer.uint32(message.underlyingDecimals);

    writer.uint32(42);
    writer.string(message.shareMint);

    writer.uint32(50);
    writer.string(message.shareTokenAcc);

    writer.uint32(56);
    writer.uint32(message.shareDecimals);

    writer.uint32(64);
    writer.uint64(message.depositLimit);

    writer.uint32(72);
    writer.uint64(message.minUserDeposit);

    writer.uint32(80);
    writer.uint64(message.performanceFee);
  }

  static decode(reader: Reader, length: i32): VaultInitEvent {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new VaultInitEvent();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.vaultIndex = reader.string();
          break;

        case 2:
          message.underlyingMint = reader.string();
          break;

        case 3:
          message.underlyingTokenAcc = reader.string();
          break;

        case 4:
          message.underlyingDecimals = reader.uint32();
          break;

        case 5:
          message.shareMint = reader.string();
          break;

        case 6:
          message.shareTokenAcc = reader.string();
          break;

        case 7:
          message.shareDecimals = reader.uint32();
          break;

        case 8:
          message.depositLimit = reader.uint64();
          break;

        case 9:
          message.minUserDeposit = reader.uint64();
          break;

        case 10:
          message.performanceFee = reader.uint64();
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  vaultIndex: string;
  underlyingMint: string;
  underlyingTokenAcc: string;
  underlyingDecimals: u32;
  shareMint: string;
  shareTokenAcc: string;
  shareDecimals: u32;
  depositLimit: u64;
  minUserDeposit: u64;
  performanceFee: u64;

  constructor(
    vaultIndex: string = "",
    underlyingMint: string = "",
    underlyingTokenAcc: string = "",
    underlyingDecimals: u32 = 0,
    shareMint: string = "",
    shareTokenAcc: string = "",
    shareDecimals: u32 = 0,
    depositLimit: u64 = 0,
    minUserDeposit: u64 = 0,
    performanceFee: u64 = 0
  ) {
    this.vaultIndex = vaultIndex;
    this.underlyingMint = underlyingMint;
    this.underlyingTokenAcc = underlyingTokenAcc;
    this.underlyingDecimals = underlyingDecimals;
    this.shareMint = shareMint;
    this.shareTokenAcc = shareTokenAcc;
    this.shareDecimals = shareDecimals;
    this.depositLimit = depositLimit;
    this.minUserDeposit = minUserDeposit;
    this.performanceFee = performanceFee;
  }
}
