# \InscriptionsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_inscription_child**](InscriptionsApi.md#fetch_inscription_child) | **Get** /inscription/{inscription_id}/{child_index} | Get inscription child info
[**fetch_inscriptions**](InscriptionsApi.md#fetch_inscriptions) | **Post** /inscriptions | Fetch multiple inscriptions
[**get_block_inscriptions**](InscriptionsApi.md#get_block_inscriptions) | **Get** /inscriptions/block/{block_height} | Get inscriptions in a specific block
[**get_inscription**](InscriptionsApi.md#get_inscription) | **Get** /inscription/{inscription_id} | Get inscription info
[**get_latest_inscriptions**](InscriptionsApi.md#get_latest_inscriptions) | **Get** /inscriptions | Get latest inscriptions
[**get_latest_inscriptions_page**](InscriptionsApi.md#get_latest_inscriptions_page) | **Get** /inscriptions/{page} | Get latest inscriptions page


# **fetch_inscription_child**
> ::models::InlineResponse20017 fetch_inscription_child(ctx, inscription_id, child_index)
Get inscription child info

Retrieve information about a specific child of an inscription

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscription_id** | **String**| Inscription ID | 
  **child_index** | **i32**| Child Index | 

### Return type

[**::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fetch_inscriptions**
> Vec<Value> fetch_inscriptions(ctx, inscriptions)
Fetch multiple inscriptions

Retrieve information about multiple inscriptions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscriptions** | **Vec&lt;String&gt;**| Inscription IDs | 

### Return type

[**Vec<Value>**](Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_inscriptions**
> ::models::InlineResponse20018 get_block_inscriptions(ctx, block_height)
Get inscriptions in a specific block

Retrieve all inscriptions in a specific block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_height** | **i32**| Block Height | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_inscription**
> ::models::InlineResponse20017 get_inscription(ctx, inscription_id)
Get inscription info

Get information about a specific inscription

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscription_id** | **String**| Inscription ID | 

### Return type

[**::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_inscriptions**
> ::models::InlineResponse20018 get_latest_inscriptions(ctx, )
Get latest inscriptions

Retrieve the latest 100 inscriptions (first page)

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_inscriptions_page**
> ::models::InlineResponse20018 get_latest_inscriptions_page(ctx, page)
Get latest inscriptions page

Retrieve a specific page of 100 inscriptions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **page** | **i32**| Page number | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

