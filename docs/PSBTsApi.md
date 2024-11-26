# \PSBTsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_psbt**](PSBTsApi.md#analyze_psbt) | **Post** /psbt/analyze | Analyze PSBT
[**combine_psbt**](PSBTsApi.md#combine_psbt) | **Post** /psbt/combine | Combine PSBTs
[**create_psbt**](PSBTsApi.md#create_psbt) | **Post** /psbt/create | Create PSBT
[**decode_psbt**](PSBTsApi.md#decode_psbt) | **Post** /psbt/decode | Decode PSBT
[**join_psbts**](PSBTsApi.md#join_psbts) | **Post** /psbt/join | Join PSBTs


# **analyze_psbt**
> ::models::InlineResponse20026 analyze_psbt(ctx, request)
Analyze PSBT

Analyzes and provides information about the current status of a PSBT and its inputs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsAnalyzePsbtRequest**](RequestsAnalyzePsbtRequest.md)| PSBT to analyze | 

### Return type

[**::models::InlineResponse20026**](inline_response_200_26.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **combine_psbt**
> ::models::InlineResponse2005 combine_psbt(ctx, request)
Combine PSBTs

Combines multiple partially signed Bitcoin transactions into one transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCombinePsbtRequest**](RequestsCombinePsbtRequest.md)| Array of PSBTs to combine | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_psbt**
> ::models::InlineResponse2005 create_psbt(ctx, request)
Create PSBT

Creates a transaction in the Partially Signed Transaction format. Implements the Creator role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCreatePsbtRequest**](RequestsCreatePsbtRequest.md)| Transaction parameters | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_psbt**
> ::models::InlineResponse20027 decode_psbt(ctx, request)
Decode PSBT

Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsAnalyzePsbtRequest**](RequestsAnalyzePsbtRequest.md)| PSBT to decode | 

### Return type

[**::models::InlineResponse20027**](inline_response_200_27.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **join_psbts**
> ::models::InlineResponse2005 join_psbts(ctx, request)
Join PSBTs

Joins multiple distinct PSBTs with different inputs and outputs into one PSBT

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsJoinPsbtsRequest**](RequestsJoinPsbtsRequest.md)| PSBTs to join | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

