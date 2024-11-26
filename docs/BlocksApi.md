# \BlocksApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_count**](BlocksApi.md#get_block_count) | **Get** /blockcount | Get the height of the latest block
[**get_block_decoded**](BlocksApi.md#get_block_decoded) | **Get** /block/raw/{identifier}/decoded | Get block by hash or height (verbosity 2)
[**get_block_hash_by_height**](BlocksApi.md#get_block_hash_by_height) | **Get** /blockhash/{block_height} | Returns blockhash of specified block.
[**get_block_hex**](BlocksApi.md#get_block_hex) | **Get** /block/raw/{identifier}/hex | Get block by hash or height (verbosity 0)
[**get_block_info**](BlocksApi.md#get_block_info) | **Get** /block/{identifier} | Get block info by hash or height
[**get_block_prevout**](BlocksApi.md#get_block_prevout) | **Get** /block/raw/{identifier}/prevout | Get block by hash or height (verbosity 3)
[**get_block_stats**](BlocksApi.md#get_block_stats) | **Post** /block/stats | Get block stats
[**get_block_summary**](BlocksApi.md#get_block_summary) | **Get** /block/raw/{identifier}/summary | Get block by hash or height (verbosity 1)
[**get_blockchain_info**](BlocksApi.md#get_blockchain_info) | **Get** /blockchain/info | Get blockchain information
[**get_blocks**](BlocksApi.md#get_blocks) | **Get** /blocks | Returns the latest block height, last 100 block hashes, and featured inscriptions
[**get_latest_block_height**](BlocksApi.md#get_latest_block_height) | **Get** /blockheight | Returns the height of the latest block.
[**get_latest_blockhash**](BlocksApi.md#get_latest_blockhash) | **Get** /blockhash | Returns blockhash for the latest block.
[**get_latest_blocktime**](BlocksApi.md#get_latest_blocktime) | **Get** /blocktime | Get the timestamp of the latest block


# **get_block_count**
> ::models::InlineResponse20011 get_block_count(ctx, )
Get the height of the latest block

Returns the height of the latest block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_decoded**
> ::models::InlineResponse2004 get_block_decoded(ctx, identifier)
Get block by hash or height (verbosity 2)

Get block by hash or height as a decoded object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **identifier** | **String**| Block hash or height | 

### Return type

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_hash_by_height**
> ::models::InlineResponse2005 get_block_hash_by_height(ctx, block_height)
Returns blockhash of specified block.

Returns blockhash of specified block.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_height** | **String**| Block Height | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_hex**
> ::models::InlineResponse2005 get_block_hex(ctx, identifier)
Get block by hash or height (verbosity 0)

Get block by hash or height as a raw hex string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **identifier** | **String**| Block hash or height | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_info**
> ::models::InlineResponse2009 get_block_info(ctx, identifier)
Get block info by hash or height

Get detailed information about a specific block by hash or height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **identifier** | **String**| Block hash or height | 

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_prevout**
> ::models::InlineResponse2006 get_block_prevout(ctx, identifier)
Get block by hash or height (verbosity 3)

Get block by hash or height with prevout information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **identifier** | **String**| Block hash or height | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_stats**
> ::models::InlineResponse2008 get_block_stats(ctx, request)
Get block stats

Computes per block statistics for a given window

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetBlockStatsRequest**](RequestsGetBlockStatsRequest.md)| Block stats request parameters | 

### Return type

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_summary**
> ::models::InlineResponse2007 get_block_summary(ctx, identifier)
Get block by hash or height (verbosity 1)

Get block by hash or height as a summary object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **identifier** | **String**| Block hash or height | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_blockchain_info**
> ::models::InlineResponse20010 get_blockchain_info(ctx, )
Get blockchain information

Returns an object containing various state info regarding blockchain processing

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_blocks**
> ::models::InlineResponse20012 get_blocks(ctx, )
Returns the latest block height, last 100 block hashes, and featured inscriptions

Returns the latest block height, last 100 block hashes, and featured inscriptions

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_block_height**
> ::models::InlineResponse20011 get_latest_block_height(ctx, )
Returns the height of the latest block.

Returns the height of the latest block.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_blockhash**
> ::models::InlineResponse2005 get_latest_blockhash(ctx, )
Returns blockhash for the latest block.

Returns blockhash for the latest block.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_blocktime**
> ::models::InlineResponse20011 get_latest_blocktime(ctx, )
Get the timestamp of the latest block

Returns the UNIX timestamp of when the latest block was mined

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

