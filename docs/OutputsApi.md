# \OutputsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_output_by_outpoint**](OutputsApi.md#get_output_by_outpoint) | **Get** /output/{outpoint} | Get output info by outpoint
[**get_outputs**](OutputsApi.md#get_outputs) | **Post** /outputs | Get multiple outputs


# **get_output_by_outpoint**
> ::models::InlineResponse20025 get_output_by_outpoint(ctx, outpoint)
Get output info by outpoint

Retrieve information about a specific UTXO using outpoint string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **outpoint** | **String**| Outpoint | 

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_outputs**
> ::models::InlineResponse2002 get_outputs(ctx, outpoints)
Get multiple outputs

Retrieve information about multiple UTXOs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **outpoints** | **Vec&lt;String&gt;**| Outpoints | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

