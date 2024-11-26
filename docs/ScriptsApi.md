# \ScriptsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_script**](ScriptsApi.md#decode_script) | **Post** /script/decode | Decode Script


# **decode_script**
> ::models::InlineResponse20032 decode_script(ctx, request)
Decode Script

Decode a hex-encoded script and return detailed information about it.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsDecodeScriptRequest**](RequestsDecodeScriptRequest.md)| Script to decode | 

### Return type

[**::models::InlineResponse20032**](inline_response_200_32.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

