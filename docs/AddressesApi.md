# \AddressesApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address_balance**](AddressesApi.md#get_address_balance) | **Get** /addresses/{address}/balance | Get address balance
[**get_address_non_inscription_utxos**](AddressesApi.md#get_address_non_inscription_utxos) | **Get** /addresses/{address}/utxos | Get address non-inscription UTXOs
[**get_address_rune_balance**](AddressesApi.md#get_address_rune_balance) | **Get** /addresses/{address}/runes/{runeid} | Get address rune balance
[**get_address_runes_balance_list**](AddressesApi.md#get_address_runes_balance_list) | **Get** /addresses/{address}/runes | Get address runes balance list
[**get_address_timeframe_balance**](AddressesApi.md#get_address_timeframe_balance) | **Get** /addresses/{address}/balance/timeframe | Get address timeframe balance


# **get_address_balance**
> ::models::ResponsesGetAddressBalance get_address_balance(ctx, address)
Get address balance

Get the current balance of a Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 

### Return type

[**::models::ResponsesGetAddressBalance**](responses.GetAddressBalance.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_non_inscription_utxos**
> ::models::ResponsesGetAddressNonInscriptionUtxo get_address_non_inscription_utxos(ctx, address)
Get address non-inscription UTXOs

Get all non-inscription UTXOs for a Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 

### Return type

[**::models::ResponsesGetAddressNonInscriptionUtxo**](responses.GetAddressNonInscriptionUTXO.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_rune_balance**
> ::models::ResponsesGetAddressRuneBalance get_address_rune_balance(ctx, address, runeid)
Get address rune balance

Get the balance of a specific rune for a Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 
  **runeid** | **String**| Rune ID | 

### Return type

[**::models::ResponsesGetAddressRuneBalance**](responses.GetAddressRuneBalance.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_runes_balance_list**
> ::models::ResponsesGetAddressRunesBalanceList get_address_runes_balance_list(ctx, address)
Get address runes balance list

Get the balance of all runes for a Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 

### Return type

[**::models::ResponsesGetAddressRunesBalanceList**](responses.GetAddressRunesBalanceList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_timeframe_balance**
> ::models::ResponsesGetAddressTimeframeBalance get_address_timeframe_balance(ctx, address, timeframe, optional)
Get address timeframe balance

Get the balance of a Bitcoin address for a specific timeframe

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address | 
  **timeframe** | **String**| Timeframe | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **String**| Bitcoin address | 
 **timeframe** | **String**| Timeframe | 
 **token** | **String**| Token | 

### Return type

[**::models::ResponsesGetAddressTimeframeBalance**](responses.GetAddressTimeframeBalance.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

