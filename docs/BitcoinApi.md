# \BitcoinApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_psbt**](BitcoinApi.md#analyze_psbt) | **Post** /psbt/analyze | Analyze PSBT
[**combine_psbt**](BitcoinApi.md#combine_psbt) | **Post** /psbt/combine | Combine PSBTs
[**combine_raw_transaction**](BitcoinApi.md#combine_raw_transaction) | **Post** /tx/combine | Combine Raw Transactions
[**convert_to_psbt**](BitcoinApi.md#convert_to_psbt) | **Post** /tx/convert-to-psbt | Convert Raw Transaction to PSBT
[**create_psbt**](BitcoinApi.md#create_psbt) | **Post** /psbt/create | Create PSBT
[**create_raw_transaction**](BitcoinApi.md#create_raw_transaction) | **Post** /tx/create | Create Raw Transaction
[**decode_psbt**](BitcoinApi.md#decode_psbt) | **Post** /psbt/decode | Decode PSBT
[**decode_script**](BitcoinApi.md#decode_script) | **Post** /script/decode | Decode Script
[**estimate_raw_fee**](BitcoinApi.md#estimate_raw_fee) | **Post** /fee/estimate-raw | Estimate Raw Fee
[**estimate_smart_fee**](BitcoinApi.md#estimate_smart_fee) | **Post** /fee/estimate-smart | Estimate smart fee
[**get_block_by_hash_decoded**](BitcoinApi.md#get_block_by_hash_decoded) | **Get** /block/hash/{hash}/decoded | Get block by hash (verbosity 2)
[**get_block_by_hash_hex**](BitcoinApi.md#get_block_by_hash_hex) | **Get** /block/hash/{hash}/hex | Get block by hash (verbosity 0)
[**get_block_by_hash_prevout**](BitcoinApi.md#get_block_by_hash_prevout) | **Get** /block/hash/{hash}/prevout | Get block by hash (verbosity 3)
[**get_block_by_hash_summary**](BitcoinApi.md#get_block_by_hash_summary) | **Get** /block/hash/{hash}/summary | Get block by hash (verbosity 1)
[**get_block_by_height_decoded**](BitcoinApi.md#get_block_by_height_decoded) | **Get** /block/height/{height}/decoded | Get block by height (verbosity 2)
[**get_block_by_height_hex**](BitcoinApi.md#get_block_by_height_hex) | **Get** /block/height/{height}/hex | Get block by height (verbosity 0)
[**get_block_by_height_prevout**](BitcoinApi.md#get_block_by_height_prevout) | **Get** /block/height/{height}/prevout | Get block by height (verbosity 3)
[**get_block_by_height_summary**](BitcoinApi.md#get_block_by_height_summary) | **Get** /block/height/{height}/summary | Get block by height (verbosity 1)
[**get_block_stats**](BitcoinApi.md#get_block_stats) | **Post** /block/stats | Get block stats
[**get_blockchain_info**](BitcoinApi.md#get_blockchain_info) | **Get** /blockchain/info | Get blockchain information
[**get_chain_tx_stats**](BitcoinApi.md#get_chain_tx_stats) | **Post** /chain/txstats | Get chain tx stats
[**get_difficulty**](BitcoinApi.md#get_difficulty) | **Get** /difficulty | Get difficulty
[**get_mempool_ancestors**](BitcoinApi.md#get_mempool_ancestors) | **Post** /mempool/ancestors | Get mempool ancestors
[**get_mempool_descendants**](BitcoinApi.md#get_mempool_descendants) | **Post** /mempool/descendants | Get mempool descendants
[**get_mempool_info**](BitcoinApi.md#get_mempool_info) | **Get** /mempool/info | Get mempool information
[**get_mining_info**](BitcoinApi.md#get_mining_info) | **Get** /mining/info | Get mining information
[**get_network_hashps**](BitcoinApi.md#get_network_hashps) | **Post** /network/hashps | Get network hash per second
[**get_raw_mempool**](BitcoinApi.md#get_raw_mempool) | **Post** /mempool/raw | Get raw mempool
[**get_raw_transaction_decoded**](BitcoinApi.md#get_raw_transaction_decoded) | **Get** /tx/{txid}/decoded | Get raw transaction (verbosity 1)
[**get_raw_transaction_hex**](BitcoinApi.md#get_raw_transaction_hex) | **Get** /tx/{txid}/hex | Get raw transaction (verbosity 0)
[**get_raw_transaction_prevout**](BitcoinApi.md#get_raw_transaction_prevout) | **Get** /tx/{txid}/prevout | Get raw transaction (verbosity 2)
[**get_tx_out**](BitcoinApi.md#get_tx_out) | **Post** /tx/out | Get transaction output
[**get_tx_out_proof**](BitcoinApi.md#get_tx_out_proof) | **Post** /tx/out/proof | Get transaction output proof
[**get_tx_out_set_info**](BitcoinApi.md#get_tx_out_set_info) | **Post** /tx/out/set/info | Get transaction output set information
[**get_tx_spending_prevout**](BitcoinApi.md#get_tx_spending_prevout) | **Post** /tx/spending/prevout | Get transaction spending prevout
[**join_psbts**](BitcoinApi.md#join_psbts) | **Post** /psbt/join | Join PSBTs
[**send_raw_transaction**](BitcoinApi.md#send_raw_transaction) | **Post** /tx/send | Send raw transaction
[**test_mempool_accept**](BitcoinApi.md#test_mempool_accept) | **Post** /mempool/test-accept | Test mempool accept
[**validate_address**](BitcoinApi.md#validate_address) | **Get** /address/{address}/validate | Validate address
[**verify_message**](BitcoinApi.md#verify_message) | **Post** /address/verify-message | Verify message
[**verify_tx_out_proof**](BitcoinApi.md#verify_tx_out_proof) | **Post** /tx/out/proof/verify | Verify transaction output proof


# **analyze_psbt**
> ::models::ResponsesAnalyzePsbtResponse analyze_psbt(ctx, request)
Analyze PSBT

Analyzes and provides information about the current status of a PSBT and its inputs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsAnalyzePsbtRequest**](RequestsAnalyzePsbtRequest.md)| PSBT to analyze | 

### Return type

[**::models::ResponsesAnalyzePsbtResponse**](responses.AnalyzePSBTResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **combine_psbt**
> ::models::ResponsesCombinePsbtResponse combine_psbt(ctx, request)
Combine PSBTs

Combines multiple partially signed Bitcoin transactions into one transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCombinePsbtRequest**](RequestsCombinePsbtRequest.md)| Array of PSBTs to combine | 

### Return type

[**::models::ResponsesCombinePsbtResponse**](responses.CombinePSBTResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **combine_raw_transaction**
> ::models::ResponsesCombineRawTransactionResponse combine_raw_transaction(ctx, request)
Combine Raw Transactions

Combines multiple partially signed transactions into one transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsCombineRawTransactionRequest**](RequestsCombineRawTransactionRequest.md)| Array of hex-encoded raw transactions | 

### Return type

[**::models::ResponsesCombineRawTransactionResponse**](responses.CombineRawTransactionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **convert_to_psbt**
> ::models::ResponsesConvertToPsbtResponse convert_to_psbt(ctx, request)
Convert Raw Transaction to PSBT

Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction. createpsbt and walletcreatefundedpsbt should be used for new applications.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsConvertToPsbtRequest**](RequestsConvertToPsbtRequest.md)| Raw transaction conversion parameters | 

### Return type

[**::models::ResponsesConvertToPsbtResponse**](responses.ConvertToPSBTResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_psbt**
> ::models::ResponsesCreatePsbtResponse create_psbt(ctx, request)
Create PSBT

Creates a transaction in the Partially Signed Transaction format. Implements the Creator role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtRequest**](GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtRequest.md)| Transaction parameters | 

### Return type

[**::models::ResponsesCreatePsbtResponse**](responses.CreatePSBTResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_raw_transaction**
> ::models::ResponsesCreateRawTransactionResponse create_raw_transaction(ctx, request)
Create Raw Transaction

Creates a raw transaction spending the given inputs and creating new outputs. Note that the transaction's inputs are not signed, and it is not stored in the wallet or transmitted to the network.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtRequest**](GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtRequest.md)| Transaction parameters | 

### Return type

[**::models::ResponsesCreateRawTransactionResponse**](responses.CreateRawTransactionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_psbt**
> ::models::ResponsesDecodePsbtResponse decode_psbt(ctx, request)
Decode PSBT

Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsAnalyzePsbtRequest**](RequestsAnalyzePsbtRequest.md)| PSBT to decode | 

### Return type

[**::models::ResponsesDecodePsbtResponse**](responses.DecodePSBTResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_script**
> ::models::ResponsesDecodeScriptResponse decode_script(ctx, request)
Decode Script

Decode a hex-encoded script and return detailed information about it.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsDecodeScriptRequest**](RequestsDecodeScriptRequest.md)| Script to decode | 

### Return type

[**::models::ResponsesDecodeScriptResponse**](responses.DecodeScriptResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **estimate_raw_fee**
> ::models::ResponsesEstimateRawFeeResponse estimate_raw_fee(ctx, request)
Estimate Raw Fee

Estimates the approximate fee per kilobyte needed for a transaction to begin confirmation within conf_target blocks if possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsEstimateRawFeeRequest**](RequestsEstimateRawFeeRequest.md)| Fee estimation parameters | 

### Return type

[**::models::ResponsesEstimateRawFeeResponse**](responses.EstimateRawFeeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **estimate_smart_fee**
> ::models::ResponsesEstimateSmartFeeResponse estimate_smart_fee(ctx, request)
Estimate smart fee

Estimates the approximate fee per kilobyte needed for a transaction to begin confirmation within conf_target blocks

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsEstimateSmartFeeRequest**](RequestsEstimateSmartFeeRequest.md)| Fee estimation parameters | 

### Return type

[**::models::ResponsesEstimateSmartFeeResponse**](responses.EstimateSmartFeeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_hash_decoded**
> ::models::ResponsesGetBlockDecodedResponse get_block_by_hash_decoded(ctx, hash)
Get block by hash (verbosity 2)

Get block by hash as a decoded object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hash** | **String**| Block hash | 

### Return type

[**::models::ResponsesGetBlockDecodedResponse**](responses.GetBlockDecodedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_hash_hex**
> ::models::ResponsesGetBlockHexResponse get_block_by_hash_hex(ctx, hash)
Get block by hash (verbosity 0)

Get block by hash as a raw hex string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hash** | **String**| Block hash | 

### Return type

[**::models::ResponsesGetBlockHexResponse**](responses.GetBlockHexResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_hash_prevout**
> ::models::ResponsesGetBlockPrevoutResponse get_block_by_hash_prevout(ctx, hash)
Get block by hash (verbosity 3)

Get block by hash with prevout information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hash** | **String**| Block hash | 

### Return type

[**::models::ResponsesGetBlockPrevoutResponse**](responses.GetBlockPrevoutResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_hash_summary**
> ::models::ResponsesGetBlockSummaryResponse get_block_by_hash_summary(ctx, hash)
Get block by hash (verbosity 1)

Get block by hash as a summary object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hash** | **String**| Block hash | 

### Return type

[**::models::ResponsesGetBlockSummaryResponse**](responses.GetBlockSummaryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_height_decoded**
> ::models::ResponsesGetBlockDecodedResponse get_block_by_height_decoded(ctx, height)
Get block by height (verbosity 2)

Get block by height as a decoded object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockDecodedResponse**](responses.GetBlockDecodedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_height_hex**
> ::models::ResponsesGetBlockHexResponse get_block_by_height_hex(ctx, height)
Get block by height (verbosity 0)

Get block by height as a raw hex string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockHexResponse**](responses.GetBlockHexResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_height_prevout**
> ::models::ResponsesGetBlockPrevoutResponse get_block_by_height_prevout(ctx, height)
Get block by height (verbosity 3)

Get block by height with prevout information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockPrevoutResponse**](responses.GetBlockPrevoutResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_height_summary**
> ::models::ResponsesGetBlockSummaryResponse get_block_by_height_summary(ctx, height)
Get block by height (verbosity 1)

Get block by height as a summary object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **height** | **i32**| Block height | 

### Return type

[**::models::ResponsesGetBlockSummaryResponse**](responses.GetBlockSummaryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_stats**
> ::models::ResponsesGetBlockStatsResponse get_block_stats(ctx, request)
Get block stats

Computes per block statistics for a given window

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetBlockStatsRequest**](RequestsGetBlockStatsRequest.md)| Block stats request parameters | 

### Return type

[**::models::ResponsesGetBlockStatsResponse**](responses.GetBlockStatsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_blockchain_info**
> ::models::ResponsesGetBlockchainInfoResponse get_blockchain_info(ctx, )
Get blockchain information

Returns an object containing various state info regarding blockchain processing

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetBlockchainInfoResponse**](responses.GetBlockchainInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_chain_tx_stats**
> ::models::ResponsesGetChainTxStatsResponse get_chain_tx_stats(ctx, request)
Get chain tx stats

Computes statistics about the total number and rate of transactions in the chain

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetChainTxStatsRequest**](RequestsGetChainTxStatsRequest.md)| Chain tx stats request parameters | 

### Return type

[**::models::ResponsesGetChainTxStatsResponse**](responses.GetChainTxStatsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_difficulty**
> ::models::ResponsesGetDifficultyResponse get_difficulty(ctx, )
Get difficulty

Returns the proof-of-work difficulty as a multiple of the minimum difficulty

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetDifficultyResponse**](responses.GetDifficultyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_ancestors**
> ::models::ResponsesGetMempoolAncestorsResponse get_mempool_ancestors(ctx, request)
Get mempool ancestors

Returns all in-mempool ancestors for a transaction in the mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetMempoolAncestorsRequest**](RequestsGetMempoolAncestorsRequest.md)| Mempool ancestors request parameters | 

### Return type

[**::models::ResponsesGetMempoolAncestorsResponse**](responses.GetMempoolAncestorsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_descendants**
> ::models::ResponsesGetMempoolDescendantsResponse get_mempool_descendants(ctx, request)
Get mempool descendants

Returns all in-mempool descendants for a transaction in the mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetMempoolDescendantsRequest**](RequestsGetMempoolDescendantsRequest.md)| Mempool descendants request parameters | 

### Return type

[**::models::ResponsesGetMempoolDescendantsResponse**](responses.GetMempoolDescendantsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mempool_info**
> ::models::ResponsesGetMempoolInfoResponse get_mempool_info(ctx, )
Get mempool information

Returns details on the active state of the TX memory pool

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetMempoolInfoResponse**](responses.GetMempoolInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mining_info**
> ::models::ResponsesGetMiningInfoResponse get_mining_info(ctx, )
Get mining information

Returns a json object containing mining-related information

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesGetMiningInfoResponse**](responses.GetMiningInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_hashps**
> ::models::ResponsesGetNetworkHashPsResponse get_network_hashps(ctx, request)
Get network hash per second

Returns the estimated network hashes per second based on the last n blocks

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetNetworkHashPsRequest**](RequestsGetNetworkHashPsRequest.md)| Network hash rate parameters | 

### Return type

[**::models::ResponsesGetNetworkHashPsResponse**](responses.GetNetworkHashPSResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_mempool**
> ::models::ResponsesGetRawMempoolResponse get_raw_mempool(ctx, request)
Get raw mempool

Returns all transaction ids in memory pool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetRawMempoolRequest**](RequestsGetRawMempoolRequest.md)| Raw mempool request parameters | 

### Return type

[**::models::ResponsesGetRawMempoolResponse**](responses.GetRawMempoolResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_decoded**
> ::models::ResponsesGetRawTransactionDecodedResponse get_raw_transaction_decoded(ctx, txid)
Get raw transaction (verbosity 1)

Get raw transaction as a decoded object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::ResponsesGetRawTransactionDecodedResponse**](responses.GetRawTransactionDecodedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_hex**
> ::models::ResponsesGetRawTransactionHexResponse get_raw_transaction_hex(ctx, txid)
Get raw transaction (verbosity 0)

Get raw transaction as a raw hex string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::ResponsesGetRawTransactionHexResponse**](responses.GetRawTransactionHexResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_raw_transaction_prevout**
> ::models::ResponsesGetRawTransactionPrevoutResponse get_raw_transaction_prevout(ctx, txid)
Get raw transaction (verbosity 2)

Get raw transaction with prevout information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::ResponsesGetRawTransactionPrevoutResponse**](responses.GetRawTransactionPrevoutResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out**
> ::models::ResponsesGetTxOutResponse get_tx_out(ctx, request)
Get transaction output

Returns details about an unspent transaction output

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutRequest**](RequestsGetTxOutRequest.md)| Transaction output request parameters | 

### Return type

[**::models::ResponsesGetTxOutResponse**](responses.GetTxOutResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out_proof**
> ::models::ResponsesGetTxOutProofResponse get_tx_out_proof(ctx, request)
Get transaction output proof

Returns a hex-encoded proof that one or more specified transactions were included in a block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutProofRequest**](RequestsGetTxOutProofRequest.md)| Transaction proof request parameters | 

### Return type

[**::models::ResponsesGetTxOutProofResponse**](responses.GetTxOutProofResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_out_set_info**
> ::models::ResponsesGetTxOutSetInfoResponse get_tx_out_set_info(ctx, request)
Get transaction output set information

Returns statistics about the unspent transaction output set

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxOutSetInfoRequest**](RequestsGetTxOutSetInfoRequest.md)| UTXO set info request parameters | 

### Return type

[**::models::ResponsesGetTxOutSetInfoResponse**](responses.GetTxOutSetInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tx_spending_prevout**
> ::models::ResponsesGetTxSpendingPrevoutResponse get_tx_spending_prevout(ctx, request)
Get transaction spending prevout

Scans the mempool to find transactions spending any of the given outputs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsGetTxSpendingPrevoutRequest**](RequestsGetTxSpendingPrevoutRequest.md)| Transaction spending prevout request | 

### Return type

[**::models::ResponsesGetTxSpendingPrevoutResponse**](responses.GetTxSpendingPrevoutResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **join_psbts**
> ::models::ResponsesJoinPsbtsResponse join_psbts(ctx, request)
Join PSBTs

Joins multiple distinct PSBTs with different inputs and outputs into one PSBT

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsJoinPsbtsRequest**](RequestsJoinPsbtsRequest.md)| PSBTs to join | 

### Return type

[**::models::ResponsesJoinPsbtsResponse**](responses.JoinPSBTsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_raw_transaction**
> ::models::ResponsesSendRawTransactionResponse send_raw_transaction(ctx, request)
Send raw transaction

Submits a raw transaction to local node and network

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsSendRawTransactionRequest**](RequestsSendRawTransactionRequest.md)| Raw transaction to send | 

### Return type

[**::models::ResponsesSendRawTransactionResponse**](responses.SendRawTransactionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **test_mempool_accept**
> ::models::ResponsesTestMempoolAcceptResponse test_mempool_accept(ctx, request)
Test mempool accept

Tests whether raw transactions would be accepted by mempool

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsTestMempoolAcceptRequest**](RequestsTestMempoolAcceptRequest.md)| Raw transactions to test | 

### Return type

[**::models::ResponsesTestMempoolAcceptResponse**](responses.TestMempoolAcceptResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validate_address**
> ::models::ResponsesValidateAddressResponse validate_address(ctx, address)
Validate address

Returns information about the given Bitcoin address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Bitcoin address to validate | 

### Return type

[**::models::ResponsesValidateAddressResponse**](responses.ValidateAddressResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verify_message**
> ::models::ResponsesVerifyMessageResponse verify_message(ctx, request)
Verify message

Verifies a signed message

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsVerifyMessageRequest**](RequestsVerifyMessageRequest.md)| Message verification parameters | 

### Return type

[**::models::ResponsesVerifyMessageResponse**](responses.VerifyMessageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verify_tx_out_proof**
> ::models::ResponsesVerifyTxOutProofResponse verify_tx_out_proof(ctx, request)
Verify transaction output proof

Verifies that a proof points to a transaction in a block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**RequestsVerifyTxOutProofRequest**](RequestsVerifyTxOutProofRequest.md)| Proof to verify | 

### Return type

[**::models::ResponsesVerifyTxOutProofResponse**](responses.VerifyTxOutProofResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

