# \RunesApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_runes_holders**](RunesApi.md#get_runes_holders) | **Get** /runes/{runeId}/holders | Get rune holders
[**get_runes_info**](RunesApi.md#get_runes_info) | **Get** /runes/{runeId} | Get rune info
[**get_runes_info_list**](RunesApi.md#get_runes_info_list) | **Get** /runes | Get runes info list


# **get_runes_holders**
> ::models::ResponsesGetRuneHolders get_runes_holders(ctx, rune_id)
Get rune holders

Get a list of addresses holding a specific rune

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rune_id** | **String**| Rune ID | 

### Return type

[**::models::ResponsesGetRuneHolders**](responses.GetRuneHolders.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_runes_info**
> ::models::ResponsesGetRuneInfo get_runes_info(ctx, rune_id)
Get rune info

Get detailed information about a specific rune

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rune_id** | **String**| Rune ID | 

### Return type

[**::models::ResponsesGetRuneInfo**](responses.GetRuneInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_runes_info_list**
> ::models::ResponsesGetRunesInfoList get_runes_info_list(ctx, optional)
Get runes info list

Get information about all runes

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Page number (default: 1) | 
 **per_page** | **i32**| Items per page (default: 10) | 

### Return type

[**::models::ResponsesGetRunesInfoList**](responses.GetRunesInfoList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

