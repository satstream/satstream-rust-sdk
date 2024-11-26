# \RunesApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_latest_runes**](RunesApi.md#get_latest_runes) | **Get** /runes | Get latest runes
[**get_latest_runes_page**](RunesApi.md#get_latest_runes_page) | **Get** /runes/{page} | Get latest runes page
[**get_rune**](RunesApi.md#get_rune) | **Get** /rune/{rune_name} | Get rune info


# **get_latest_runes**
> ::models::InlineResponse20030 get_latest_runes(ctx, )
Get latest runes

Retrieve information about the last 100 inscribed runes (first page)

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_runes_page**
> ::models::InlineResponse20030 get_latest_runes_page(ctx, page)
Get latest runes page

Retrieve a specific page of 100 inscribed runes

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **page** | **i32**| Page number | 

### Return type

[**::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rune**
> ::models::InlineResponse20029 get_rune(ctx, rune_name)
Get rune info

Retrieve information about a specific rune

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rune_name** | **String**| Rune Name | 

### Return type

[**::models::InlineResponse20029**](inline_response_200_29.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

