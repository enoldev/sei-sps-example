import { Protobuf } from "as-proto/assembly";
import { Events as protoEvents } from "./pb/contract/v1/Events";
import { Approval, Transfer } from "../generated/schema";
import { BigInt, log, crypto, Bytes } from "@graphprotocol/graph-ts";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoEvents>(bytes, protoEvents.decode);

  for (let i = 0; i < input.seiApprovals.length; i++) {
    let approval = input.seiApprovals[i];

    const entityHash = `${crypto.keccak256(Bytes.fromUint8Array(bytes)).toHexString()}-approval-${i}`;
    let entity = new Approval(entityHash);
    entity.blockNumber = approval.evtBlockNumber as i32;
    entity.eventIndex = approval.evtIndex;
    entity.transactionHash = approval.evtTxHash;
    entity.value = approval.value;

    entity.save();
  }

  for (let i = 0; i < input.seiTransfers.length; i++) {
    let transfer = input.seiTransfers[i];

    const entityHash = `${crypto.keccak256(Bytes.fromUint8Array(bytes)).toHexString()}-transfer-${i}`;
    let entity = new Transfer(entityHash);
    entity.blockNumber = transfer.evtBlockNumber as i32;
    entity.eventIndex = transfer.evtIndex;
    entity.transactionHash = transfer.evtTxHash;
    entity.value = transfer.value;

    entity.save();
  }

}
