# \AddressesApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address**](AddressesApi.md#get_address) | **Get** /address/{address} | Get address info
[**get_address_utxos**](AddressesApi.md#get_address_utxos) | **Get** /address/{address}/outputs | Get UTXOs for an address
[**validate_address**](AddressesApi.md#validate_address) | **Get** /address/{address}/validate | Validate address
[**verify_message**](AddressesApi.md#verify_message) | **Post** /address/verify-message | Verify message


# **get_address**
> ::models::InlineResponse2001 get_address(ctx, address)
Get address info

Get detailed information about a specific address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Address | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_utxos**
> ::models::InlineResponse2002 get_address_utxos(ctx, address, optional)
Get UTXOs for an address

Retrieve UTXOs held by a specific address with optional type filtering

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Address | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **String**| Address | 
 **_type** | **String**| UTXO Type | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validate_address**
> ::models::InlineResponse2003 validate_address(ctx, address)
Validate address

Returns information about the given Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address to validate | 

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verify_message**
> ::models::InlineResponse200 verify_message(ctx, request)
Verify message

Verifies a signed message

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsVerifyMessageRequest**](RequestsVerifyMessageRequest.md)| Message verification parameters | 

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

