# \BlocksApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_by_hash**](BlocksApi.md#get_block_by_hash) | **Get** /block/hash/{block_hash} | Get block info by hash


# **get_block_by_hash**
> ::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse get_block_by_hash(ctx, block_hash)
Get block info by hash

Get detailed information about a specific block by hash

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_hash** | **String**| Block Hash | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse**](github_com_satstream_ss-api_server_api_block_responses.BlockResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

