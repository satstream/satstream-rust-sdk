# \TransactionsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broadcast_transaction**](TransactionsApi.md#broadcast_transaction) | **Post** /transactions/broadcast | Broadcast transaction
[**get_transaction**](TransactionsApi.md#get_transaction) | **Get** /indexer/tx/{hash} | Get transaction
[**get_transaction_info**](TransactionsApi.md#get_transaction_info) | **Get** /transactions/{txid} | Get transaction info


# **broadcast_transaction**
> ::models::ResponsesSendRawTransaction broadcast_transaction(ctx, transaction)
Broadcast transaction

Broadcast a raw transaction to the Bitcoin network

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **transaction** | **String**| Raw transaction hex | 

### Return type

[**::models::ResponsesSendRawTransaction**](responses.SendRawTransaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction**
> ::models::ResponsesGetTransaction get_transaction(hash)
Get transaction

Get a transaction by its hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **hash** | **String**| Transaction hash | 

### Return type

[**::models::ResponsesGetTransaction**](responses.GetTransaction.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_info**
> ::models::ResponsesGetTxInfo get_transaction_info(ctx, txid)
Get transaction info

Get detailed information about a specific transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::ResponsesGetTxInfo**](responses.GetTxInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

