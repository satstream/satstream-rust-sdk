# UtxoSetInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bestblock** | **String** | The hash of the block at which these statistics are calculated | [optional] [default to null]
**block_info** | [***::models::UtxoSetInfoBlockInfo**](UTXOSetInfo_block_info.md) |  | [optional] [default to null]
**bogosize** | **i32** | Database-independent metric indicating the UTXO set size | [optional] [default to null]
**disk_size** | **i32** | The estimated size of the chainstate on disk | [optional] [default to null]
**hash_serialized_2** | **String** | The serialized hash (only for hash_serialized_2) | [optional] [default to null]
**height** | **i32** | The block height of the returned statistics | [optional] [default to null]
**muhash** | **String** | The serialized hash (only for muhash) | [optional] [default to null]
**total_amount** | **f32** | The total amount of coins in the UTXO set | [optional] [default to null]
**total_unspendable_amount** | **f32** | Total amount permanently excluded from UTXO set | [optional] [default to null]
**transactions** | **i32** | The number of transactions with unspent outputs | [optional] [default to null]
**txouts** | **i32** | The number of unspent transaction outputs | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


