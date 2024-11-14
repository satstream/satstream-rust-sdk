# \MempoolApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address_mempool_transactions**](MempoolApi.md#get_address_mempool_transactions) | **Get** /mempool/addresses/{address}/transactions | Get address mempool transactions
[**get_mempool_transaction_info**](MempoolApi.md#get_mempool_transaction_info) | **Get** /mempool/transactions/{txid} | Get mempool transaction info
[**get_mempool_transactions**](MempoolApi.md#get_mempool_transactions) | **Get** /mempool/transactions | Get mempool transactions


# **get_address_mempool_transactions**
> ::models::ResponsesGetAddressMempoolTxs get_address_mempool_transactions(ctx, address)
Get address mempool transactions

Get all mempool transactions for a specific address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 

### Return type

[**::models::ResponsesGetAddressMempoolTxs**](responses.GetAddressMempoolTxs.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_transaction_info**
> ::models::ResponsesGetMempoolTxInfo get_mempool_transaction_info(ctx, txid)
Get mempool transaction info

Get information about a specific transaction in the mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::ResponsesGetMempoolTxInfo**](responses.GetMempoolTxInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_transactions**
> ::models::ResponsesGetMempoolTransactions get_mempool_transactions()
Get mempool transactions

Get all transactions currently in the mempool

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetMempoolTransactions**](responses.GetMempoolTransactions.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

