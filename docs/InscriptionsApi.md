# \InscriptionsApi

All URIs are relative to *https://api.satstream.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_tx**](InscriptionsApi.md#decode_tx) | **Get** /decode/{txid} | Decode a transaction
[**fetch_inscription_child**](InscriptionsApi.md#fetch_inscription_child) | **Get** /inscription/{inscription_id}/child/{child_index} | Get inscription child info
[**fetch_inscriptions**](InscriptionsApi.md#fetch_inscriptions) | **Post** /inscriptions | Fetch multiple inscriptions
[**get_address**](InscriptionsApi.md#get_address) | **Get** /address/{address} | Get address info
[**get_address_utxos**](InscriptionsApi.md#get_address_utxos) | **Get** /address/{address}/outputs | Get UTXOs for an address
[**get_block_by_height**](InscriptionsApi.md#get_block_by_height) | **Get** /block/height/{block_height} | Get block info by height
[**get_block_count**](InscriptionsApi.md#get_block_count) | **Get** /blockcount | Get the height of the latest block
[**get_block_hash_by_height**](InscriptionsApi.md#get_block_hash_by_height) | **Get** /blockhash/{block_height} | Returns blockhash of specified block.
[**get_block_inscriptions**](InscriptionsApi.md#get_block_inscriptions) | **Get** /inscriptions/block/{block_height} | Get inscriptions in a specific block
[**get_blocks**](InscriptionsApi.md#get_blocks) | **Get** /blocks | Returns the latest block height, last 100 block hashes, and featured inscriptions
[**get_inscription**](InscriptionsApi.md#get_inscription) | **Get** /inscription/{inscription_id} | Get inscription info
[**get_latest_block_height**](InscriptionsApi.md#get_latest_block_height) | **Get** /latestblockheight | Returns the height of the latest block.
[**get_latest_blockhash**](InscriptionsApi.md#get_latest_blockhash) | **Get** /latestblockhash | Returns blockhash for the latest block.
[**get_latest_blocktime**](InscriptionsApi.md#get_latest_blocktime) | **Get** /blocktime | Get the timestamp of the latest block
[**get_latest_inscriptions**](InscriptionsApi.md#get_latest_inscriptions) | **Get** /inscriptions/latest | Get latest inscriptions
[**get_latest_inscriptions_page**](InscriptionsApi.md#get_latest_inscriptions_page) | **Get** /inscriptions/page/{page} | Get latest inscriptions page
[**get_latest_runes**](InscriptionsApi.md#get_latest_runes) | **Get** /runes/latest | Get latest runes
[**get_latest_runes_page**](InscriptionsApi.md#get_latest_runes_page) | **Get** /runes/page/{page} | Get latest runes page
[**get_output_by_outpoint**](InscriptionsApi.md#get_output_by_outpoint) | **Get** /output/outpoint/{outpoint} | Get output info by outpoint
[**get_outputs**](InscriptionsApi.md#get_outputs) | **Post** /outputs | Get multiple outputs
[**get_rune**](InscriptionsApi.md#get_rune) | **Get** /rune/{rune_name} | Get rune info
[**get_satoshi**](InscriptionsApi.md#get_satoshi) | **Get** /sat/{number} | Get satoshi info
[**get_status**](InscriptionsApi.md#get_status) | **Get** /status | Get server status
[**get_transaction**](InscriptionsApi.md#get_transaction) | **Get** /tx/{txid} | Get transaction info


# **decode_tx**
> ::models::GithubComSatstreamSsApiServerApiTransactionResponsesDecodeResponse decode_tx(ctx, txid)
Decode a transaction

Decodes a transaction and returns its inscriptions and runestone data

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiTransactionResponsesDecodeResponse**](github_com_satstream_ss-api_server_api_transaction_responses.DecodeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fetch_inscription_child**
> ::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse fetch_inscription_child(ctx, inscription_id, child_index)
Get inscription child info

Retrieve information about a specific child of an inscription

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscription_id** | **String**| Inscription ID | 
  **child_index** | **i32**| Child Index | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse**](github_com_satstream_ss-api_server_api_inscription_responses.InscriptionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fetch_inscriptions**
> Vec<::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse> fetch_inscriptions(ctx, inscriptions)
Fetch multiple inscriptions

Retrieve information about multiple inscriptions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscriptions** | **Vec&lt;String&gt;**| Inscription IDs | 

### Return type

[**Vec<::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse>**](github_com_satstream_ss-api_server_api_inscription_responses.InscriptionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address**
> ::models::GithubComSatstreamSsApiServerApiAddressResponsesAddressResponse get_address(ctx, address)
Get address info

Get detailed information about a specific address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Address | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiAddressResponsesAddressResponse**](github_com_satstream_ss-api_server_api_address_responses.AddressResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_address_utxos**
> Vec<::models::GithubComSatstreamSsApiServerApiAddressResponsesOutputResponse> get_address_utxos(ctx, address, optional)
Get UTXOs for an address

Retrieve UTXOs held by a specific address with optional type filtering

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | **String**| Address | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **String**| Address | 
 **_type** | **String**| UTXO Type | 

### Return type

[**Vec<::models::GithubComSatstreamSsApiServerApiAddressResponsesOutputResponse>**](github_com_satstream_ss-api_server_api_address_responses.OutputResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_by_height**
> ::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse get_block_by_height(ctx, block_height)
Get block info by height

Get detailed information about a specific block by height

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_height** | **String**| Block Height | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse**](github_com_satstream_ss-api_server_api_block_responses.BlockResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_count**
> ::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockCountResponse get_block_count(ctx, )
Get the height of the latest block

Returns the height of the latest block

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockCountResponse**](github_com_satstream_ss-api_server_api_block_responses.BlockCountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_hash_by_height**
> ::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockHashResponse get_block_hash_by_height(ctx, block_height)
Returns blockhash of specified block.

Returns blockhash of specified block.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_height** | **String**| Block Height | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiBlockResponsesBlockHashResponse**](github_com_satstream_ss-api_server_api_block_responses.BlockHashResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_block_inscriptions**
> ::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse get_block_inscriptions(ctx, block_height)
Get inscriptions in a specific block

Retrieve all inscriptions in a specific block

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **block_height** | **i32**| Block Height | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse**](github_com_satstream_ss-api_server_api_inscription_responses.LatestInscriptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_blocks**
> ::models::GithubComSatstreamSsApiServerApiBlockResponsesBlocksResponse get_blocks(ctx, )
Returns the latest block height, last 100 block hashes, and featured inscriptions

Returns the latest block height, last 100 block hashes, and featured inscriptions

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GithubComSatstreamSsApiServerApiBlockResponsesBlocksResponse**](github_com_satstream_ss-api_server_api_block_responses.BlocksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_inscription**
> ::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse get_inscription(ctx, inscription_id)
Get inscription info

Get information about a specific inscription

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **inscription_id** | **String**| Inscription ID | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse**](github_com_satstream_ss-api_server_api_inscription_responses.InscriptionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_block_height**
> ::models::ResponsesLatestBlockHeightResponse get_latest_block_height(ctx, )
Returns the height of the latest block.

Returns the height of the latest block.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesLatestBlockHeightResponse**](responses.LatestBlockHeightResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_blockhash**
> ::models::ResponsesLatestBlockHashResponse get_latest_blockhash(ctx, )
Returns blockhash for the latest block.

Returns blockhash for the latest block.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesLatestBlockHashResponse**](responses.LatestBlockHashResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_blocktime**
> ::models::ResponsesLatestBlockTimeResponse get_latest_blocktime(ctx, )
Get the timestamp of the latest block

Returns the UNIX timestamp of when the latest block was mined

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ResponsesLatestBlockTimeResponse**](responses.LatestBlockTimeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_inscriptions**
> ::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse get_latest_inscriptions(ctx, )
Get latest inscriptions

Retrieve the latest 100 inscriptions (first page)

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse**](github_com_satstream_ss-api_server_api_inscription_responses.LatestInscriptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_inscriptions_page**
> ::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse get_latest_inscriptions_page(ctx, page)
Get latest inscriptions page

Retrieve a specific page of 100 inscriptions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **page** | **i32**| Page number | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse**](github_com_satstream_ss-api_server_api_inscription_responses.LatestInscriptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_runes**
> ::models::GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse get_latest_runes(ctx, )
Get latest runes

Retrieve information about the last 100 inscribed runes (first page)

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse**](github_com_satstream_ss-api_server_api_rune_responses.RunesListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_latest_runes_page**
> ::models::GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse get_latest_runes_page(ctx, page)
Get latest runes page

Retrieve a specific page of 100 inscribed runes

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **page** | **i32**| Page number | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse**](github_com_satstream_ss-api_server_api_rune_responses.RunesListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_output_by_outpoint**
> ::models::ResponsesGetOutputByOutpointResponse get_output_by_outpoint(ctx, outpoint)
Get output info by outpoint

Retrieve information about a specific UTXO using outpoint string

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **outpoint** | **String**| Outpoint | 

### Return type

[**::models::ResponsesGetOutputByOutpointResponse**](responses.GetOutputByOutpointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_outputs**
> Vec<::models::ResponsesGetOutputsResponse> get_outputs(ctx, outpoints)
Get multiple outputs

Retrieve information about multiple UTXOs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **outpoints** | **Vec&lt;String&gt;**| Outpoints | 

### Return type

[**Vec<::models::ResponsesGetOutputsResponse>**](responses.GetOutputsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rune**
> ::models::GithubComSatstreamSsApiServerApiRuneResponsesRuneResponse get_rune(ctx, rune_name)
Get rune info

Retrieve information about a specific rune

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rune_name** | **String**| Rune Name | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiRuneResponsesRuneResponse**](github_com_satstream_ss-api_server_api_rune_responses.RuneResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_satoshi**
> ::models::GithubComSatstreamSsApiServerApiSatoshiResponsesSatoshiResponse get_satoshi(ctx, number)
Get satoshi info

Retrieve information about a specific satoshi

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **number** | **i32**| Satoshi Number | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiSatoshiResponsesSatoshiResponse**](github_com_satstream_ss-api_server_api_satoshi_responses.SatoshiResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_status**
> ::models::GithubComSatstreamSsApiServerApiStatusResponsesStatusResponse get_status(ctx, )
Get server status

Retrieve information about the server installation and index

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GithubComSatstreamSsApiServerApiStatusResponsesStatusResponse**](github_com_satstream_ss-api_server_api_status_responses.StatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction**
> ::models::GithubComSatstreamSsApiServerApiTransactionResponsesTransactionResponse get_transaction(ctx, txid)
Get transaction info

Retrieve information about a specific transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **txid** | **String**| Transaction ID | 

### Return type

[**::models::GithubComSatstreamSsApiServerApiTransactionResponsesTransactionResponse**](github_com_satstream_ss-api_server_api_transaction_responses.TransactionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

