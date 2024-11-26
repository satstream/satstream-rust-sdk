# \TransactionsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combine_raw_transaction**](TransactionsApi.md#combine_raw_transaction) | **Post** /tx/combine | Combine Raw Transactions
[**convert_to_psbt**](TransactionsApi.md#convert_to_psbt) | **Post** /tx/convert-to-psbt | Convert Raw Transaction to PSBT
[**create_raw_transaction**](TransactionsApi.md#create_raw_transaction) | **Post** /tx/create | Create Raw Transaction
[**decode_tx**](TransactionsApi.md#decode_tx) | **Get** /tx/{txid}/decode | Decode a transaction
[**get_raw_transaction_decoded**](TransactionsApi.md#get_raw_transaction_decoded) | **Get** /tx/{txid}/decoded | Get raw transaction (verbosity 1)
[**get_raw_transaction_hex**](TransactionsApi.md#get_raw_transaction_hex) | **Get** /tx/{txid}/hex | Get raw transaction (verbosity 0)
[**get_raw_transaction_prevout**](TransactionsApi.md#get_raw_transaction_prevout) | **Get** /tx/{txid}/prevout | Get raw transaction (verbosity 2)
[**get_transaction**](TransactionsApi.md#get_transaction) | **Get** /tx/{txid} | Get transaction info
[**get_tx_out**](TransactionsApi.md#get_tx_out) | **Post** /tx/out | Get transaction output
[**get_tx_out_proof**](TransactionsApi.md#get_tx_out_proof) | **Post** /tx/outproof | Get transaction output proof
[**get_tx_out_set_info**](TransactionsApi.md#get_tx_out_set_info) | **Post** /tx/out/set/info | Get transaction output set information
[**get_tx_spending_prevout**](TransactionsApi.md#get_tx_spending_prevout) | **Post** /tx/spending-prevout | Get transaction spending prevout
[**send_raw_transaction**](TransactionsApi.md#send_raw_transaction) | **Post** /tx/send | Send raw transaction
[**verify_tx_out_proof**](TransactionsApi.md#verify_tx_out_proof) | **Post** /tx/outproof/verify | Verify transaction output proof


# **combine_raw_transaction**
> ::models::InlineResponse2005 combine_raw_transaction(ctx, request)
Combine Raw Transactions

Combines multiple partially signed transactions into one transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCombineRawTransactionRequest**](RequestsCombineRawTransactionRequest.md)| Array of hex-encoded raw transactions | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **convert_to_psbt**
> ::models::InlineResponse2005 convert_to_psbt(ctx, request)
Convert Raw Transaction to PSBT

Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction. createpsbt and walletcreatefundedpsbt should be used for new applications.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsConvertToPsbtRequest**](RequestsConvertToPsbtRequest.md)| Raw transaction conversion parameters | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_raw_transaction**
> ::models::InlineResponse2005 create_raw_transaction(ctx, request)
Create Raw Transaction

Creates a raw transaction spending the given inputs and creating new outputs. Note that the transaction's inputs are not signed, and it is not stored in the wallet or transmitted to the network.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCreateRawTxRequest**](RequestsCreateRawTxRequest.md)| Transaction parameters | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_tx**
> ::models::InlineResponse20038 decode_tx(ctx, txid)
Decode a transaction

Decodes a transaction and returns its inscriptions and runestone data

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::InlineResponse20038**](inline_response_200_38.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_decoded**
> ::models::InlineResponse20039 get_raw_transaction_decoded(ctx, txid)
Get raw transaction (verbosity 1)

Get raw transaction as a decoded object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::InlineResponse20039**](inline_response_200_39.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_hex**
> ::models::InlineResponse2005 get_raw_transaction_hex(ctx, txid)
Get raw transaction (verbosity 0)

Get raw transaction as a raw hex string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_prevout**
> ::models::InlineResponse20040 get_raw_transaction_prevout(ctx, txid)
Get raw transaction (verbosity 2)

Get raw transaction with prevout information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction**
> ::models::InlineResponse20037 get_transaction(ctx, txid)
Get transaction info

Retrieve information about a specific transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::InlineResponse20037**](inline_response_200_37.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out**
> ::models::InlineResponse20033 get_tx_out(ctx, request)
Get transaction output

Returns details about an unspent transaction output

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutRequest**](RequestsGetTxOutRequest.md)| Transaction output request parameters | 

### Return type

[**::models::InlineResponse20033**](inline_response_200_33.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out_proof**
> ::models::InlineResponse2005 get_tx_out_proof(ctx, request)
Get transaction output proof

Returns a hex-encoded proof that one or more specified transactions were included in a block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutProofRequest**](RequestsGetTxOutProofRequest.md)| Transaction proof request parameters | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out_set_info**
> ::models::InlineResponse20034 get_tx_out_set_info(ctx, request)
Get transaction output set information

Returns statistics about the unspent transaction output set

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutSetInfoRequest**](RequestsGetTxOutSetInfoRequest.md)| UTXO set info request parameters | 

### Return type

[**::models::InlineResponse20034**](inline_response_200_34.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_spending_prevout**
> ::models::InlineResponse20036 get_tx_spending_prevout(ctx, request)
Get transaction spending prevout

Scans the mempool to find transactions spending any of the given outputs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxSpendingPrevoutRequest**](RequestsGetTxSpendingPrevoutRequest.md)| Transaction spending prevout request | 

### Return type

[**::models::InlineResponse20036**](inline_response_200_36.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_raw_transaction**
> ::models::InlineResponse2005 send_raw_transaction(ctx, request)
Send raw transaction

Submits a raw transaction to local node and network

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsSendRawTransactionRequest**](RequestsSendRawTransactionRequest.md)| Raw transaction to send | 

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verify_tx_out_proof**
> ::models::InlineResponse20035 verify_tx_out_proof(ctx, request)
Verify transaction output proof

Verifies that a proof points to a transaction in a block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsVerifyTxOutProofRequest**](RequestsVerifyTxOutProofRequest.md)| Proof to verify | 

### Return type

[**::models::InlineResponse20035**](inline_response_200_35.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

