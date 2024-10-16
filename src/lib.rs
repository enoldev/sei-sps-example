mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const SEI_TRACKED_CONTRACT: [u8; 20] = hex!("3894085ef7ff0f0aedf52e2a2704928d1ec074f1");

fn map_sei_events(blk: &eth::Block, events: &mut contract::Events) {
    events.sei_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SEI_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sei_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::SeiApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sei_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SEI_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sei_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::SeiTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_sei_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.sei_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SEI_TRACKED_CONTRACT && abi::sei_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::sei_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::sei_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SeiApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sei_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SEI_TRACKED_CONTRACT && abi::sei_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::sei_contract::functions::Transfer::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::sei_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SeiTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sei_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SEI_TRACKED_CONTRACT && abi::sei_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::sei_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::sei_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SeiTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_sei_events(&blk, &mut events);
    substreams::skip_empty_output();
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_sei_calls(&blk, &mut calls);
    substreams::skip_empty_output();
    Ok(calls)
}

