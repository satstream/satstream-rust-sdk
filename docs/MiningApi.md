# \MiningApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_mining_info**](MiningApi.md#get_mining_info) | **Get** /mining/info | Get mining information
[**get_network_hashps**](MiningApi.md#get_network_hashps) | **Post** /mining/networkhashps | Get network hash per second


# **get_mining_info**
> ::models::InlineResponse20024 get_mining_info(ctx, )
Get mining information

Returns a json object containing mining-related information

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20024**](inline_response_200_24.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_hashps**
> ::models::InlineResponse20013 get_network_hashps(ctx, request)
Get network hash per second

Returns the estimated network hashes per second based on the last n blocks

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetNetworkHashPsRequest**](RequestsGetNetworkHashPsRequest.md)| Network hash rate parameters | 

### Return type

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

