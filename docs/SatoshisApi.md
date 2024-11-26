# \SatoshisApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_satoshi**](SatoshisApi.md#get_satoshi) | **Get** /sat/{number} | Get satoshi info


# **get_satoshi**
> ::models::InlineResponse20030 get_satoshi(ctx, number)
Get satoshi info

Retrieve information about a specific satoshi

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **number** | **i32**| Satoshi Number | 

### Return type

[**::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

