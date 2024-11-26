# MempoolEntry

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ancestorcount** | **i32** | Number of in-mempool ancestor transactions | [optional] [default to null]
**ancestorsize** | **i32** | Virtual size of in-mempool ancestors | [optional] [default to null]
**bip125_replaceable** | **bool** | Whether this transaction is replaceable | [optional] [default to null]
**depends** | **Vec<String>** | Parent transaction IDs | [optional] [default to null]
**descendantcount** | **i32** | Number of in-mempool descendant transactions | [optional] [default to null]
**descendantsize** | **i32** | Virtual size of in-mempool descendants | [optional] [default to null]
**fees** | [***::models::MempoolEntryFees**](MempoolEntry_fees.md) |  | [optional] [default to null]
**height** | **i32** | Block height when transaction entered pool | [optional] [default to null]
**spentby** | **Vec<String>** | Child transaction IDs | [optional] [default to null]
**time** | **i32** | Time transaction entered pool | [optional] [default to null]
**unbroadcast** | **bool** | Whether this transaction is currently unbroadcast | [optional] [default to null]
**vsize** | **i32** | Virtual transaction size | [optional] [default to null]
**weight** | **i32** | Transaction weight | [optional] [default to null]
**wtxid** | **String** | Hash of serialized transaction with witness data | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


