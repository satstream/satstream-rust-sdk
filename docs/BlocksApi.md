# \BlocksApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_by_hash**](BlocksApi.md#get_block_by_hash) | **Get** /blocks/hash/{hash} | Get block by hash
[**get_block_info**](BlocksApi.md#get_block_info) | **Get** /blocks/{height} | Get block info
[**get_block_transactions**](BlocksApi.md#get_block_transactions) | **Get** /blocks/{height}/transactions | Get block transactions
[**get_current_block_height**](BlocksApi.md#get_current_block_height) | **Get** /blocks/current-height | Get current block height


# **get_block_by_hash**
> ::models::ResponsesGetBlockByHash get_block_by_hash(hash)
Get block by hash

Get information about a specific block by its hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| Block hash | 

### Return type

[**::models::ResponsesGetBlockByHash**](responses.GetBlockByHash.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_info**
> ::models::ResponsesGetBlockInfo get_block_info(height)
Get block info

Get information about a specific block by height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockInfo**](responses.GetBlockInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_transactions**
> ::models::ResponsesGetBlockTransactions get_block_transactions(height)
Get block transactions

Get transactions for a specific block height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockTransactions**](responses.GetBlockTransactions.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_current_block_height**
> ::models::ResponsesGetBlockHeight get_current_block_height()
Get current block height

Get the current block height of the Bitcoin blockchain

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetBlockHeight**](responses.GetBlockHeight.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

