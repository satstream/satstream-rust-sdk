# \MempoolApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_mempool_ancestors**](MempoolApi.md#get_mempool_ancestors) | **Post** /mempool/ancestors | Get mempool ancestors
[**get_mempool_descendants**](MempoolApi.md#get_mempool_descendants) | **Post** /mempool/descendants | Get mempool descendants
[**get_mempool_info**](MempoolApi.md#get_mempool_info) | **Get** /mempool/info | Get mempool information
[**get_raw_mempool**](MempoolApi.md#get_raw_mempool) | **Post** /mempool/raw | Get raw mempool
[**test_mempool_accept**](MempoolApi.md#test_mempool_accept) | **Post** /mempool/test-accept | Test mempool accept


# **get_mempool_ancestors**
> ::models::InlineResponse20020 get_mempool_ancestors(ctx, request)
Get mempool ancestors

Returns all in-mempool ancestors for a transaction in the mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetMempoolAncestorsRequest**](RequestsGetMempoolAncestorsRequest.md)| Mempool ancestors request parameters | 

### Return type

[**::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_descendants**
> ::models::InlineResponse20021 get_mempool_descendants(ctx, request)
Get mempool descendants

Returns all in-mempool descendants for a transaction in the mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetMempoolDescendantsRequest**](RequestsGetMempoolDescendantsRequest.md)| Mempool descendants request parameters | 

### Return type

[**::models::InlineResponse20021**](inline_response_200_21.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_info**
> ::models::InlineResponse20022 get_mempool_info(ctx, )
Get mempool information

Returns details on the active state of the TX memory pool

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20022**](inline_response_200_22.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_mempool**
> ::models::InlineResponse20023 get_raw_mempool(ctx, request)
Get raw mempool

Returns all transaction ids in memory pool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetRawMempoolRequest**](RequestsGetRawMempoolRequest.md)| Raw mempool request parameters | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **test_mempool_accept**
> ::models::InlineResponse20024 test_mempool_accept(ctx, request)
Test mempool accept

Tests whether raw transactions would be accepted by mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsTestMempoolAcceptRequest**](RequestsTestMempoolAcceptRequest.md)| Raw transactions to test | 

### Return type

[**::models::InlineResponse20024**](inline_response_200_24.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

