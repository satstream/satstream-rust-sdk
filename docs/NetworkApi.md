# \NetworkApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chain_tx_stats**](NetworkApi.md#get_chain_tx_stats) | **Post** /chain/txstats | Get chain tx stats
[**get_difficulty**](NetworkApi.md#get_difficulty) | **Get** /chain/difficulty | Get difficulty


# **get_chain_tx_stats**
> ::models::InlineResponse20014 get_chain_tx_stats(ctx, request)
Get chain tx stats

Computes statistics about the total number and rate of transactions in the chain

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetChainTxStatsRequest**](RequestsGetChainTxStatsRequest.md)| Chain tx stats request parameters | 

### Return type

[**::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_difficulty**
> ::models::InlineResponse20013 get_difficulty(ctx, )
Get difficulty

Returns the proof-of-work difficulty as a multiple of the minimum difficulty

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

