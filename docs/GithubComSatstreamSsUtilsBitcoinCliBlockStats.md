# GithubComSatstreamSsUtilsBitcoinCliBlockStats

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avgfee** | **f32** | Average fee in the block | [optional] [default to null]
**avgfeerate** | **f32** | Average feerate (in satoshis per virtual byte) | [optional] [default to null]
**avgtxsize** | **f32** | Average transaction size | [optional] [default to null]
**blockhash** | **String** | The block hash (to check for potential reorgs) | [optional] [default to null]
**feerate_percentiles** | **Vec<f32>** | Feerates at the 10th, 25th, 50th, 75th, and 90th percentile | [optional] [default to null]
**height** | **i32** | The height of the block | [optional] [default to null]
**ins** | **i32** | The number of inputs (excluding coinbase) | [optional] [default to null]
**maxfee** | **f32** | Maximum fee in the block | [optional] [default to null]
**maxfeerate** | **f32** | Maximum feerate (in satoshis per virtual byte) | [optional] [default to null]
**maxtxsize** | **i32** | Maximum transaction size | [optional] [default to null]
**medianfee** | **f32** | Truncated median fee in the block | [optional] [default to null]
**mediantime** | **i32** | The block median time past | [optional] [default to null]
**mediantxsize** | **i32** | Truncated median transaction size | [optional] [default to null]
**minfee** | **f32** | Minimum fee in the block | [optional] [default to null]
**minfeerate** | **f32** | Minimum feerate (in satoshis per virtual byte) | [optional] [default to null]
**mintxsize** | **i32** | Minimum transaction size | [optional] [default to null]
**outs** | **i32** | The number of outputs | [optional] [default to null]
**subsidy** | **f32** | The block subsidy | [optional] [default to null]
**swtotal_size** | **i32** | Total size of all segwit transactions | [optional] [default to null]
**swtotal_weight** | **i32** | Total weight of all segwit transactions | [optional] [default to null]
**swtxs** | **i32** | The number of segwit transactions | [optional] [default to null]
**time** | **i32** | The block time | [optional] [default to null]
**total_out** | **f32** | Total amount in all outputs | [optional] [default to null]
**total_size** | **i32** | Total size of all non-coinbase transactions | [optional] [default to null]
**total_weight** | **i32** | Total weight of all non-coinbase transactions | [optional] [default to null]
**totalfee** | **f32** | The fee total | [optional] [default to null]
**txs** | **i32** | The number of transactions (excluding coinbase) | [optional] [default to null]
**utxo_increase** | **i32** | The increase/decrease in the number of unspent outputs | [optional] [default to null]
**utxo_size_inc** | **i32** | The increase/decrease in size for the utxo index | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


