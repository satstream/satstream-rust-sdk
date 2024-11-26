# Rust API client for satstream-rust-sdk

Satstream API

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 1.0.19
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [https://satstream.io](https://satstream.io)

## Installation
Put the package under your project folder and add the following in import:
```
    "./satstream-rust-sdk"
```

## Documentation for API Endpoints

All URIs are relative to *https://api.satstream.io/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BitcoinApi* | [**analyze_psbt**](docs/BitcoinApi.md#analyze_psbt) | **Post** /psbt/analyze | Analyze PSBT
*BitcoinApi* | [**combine_psbt**](docs/BitcoinApi.md#combine_psbt) | **Post** /psbt/combine | Combine PSBTs
*BitcoinApi* | [**combine_raw_transaction**](docs/BitcoinApi.md#combine_raw_transaction) | **Post** /tx/combine | Combine Raw Transactions
*BitcoinApi* | [**convert_to_psbt**](docs/BitcoinApi.md#convert_to_psbt) | **Post** /tx/convert-to-psbt | Convert Raw Transaction to PSBT
*BitcoinApi* | [**create_psbt**](docs/BitcoinApi.md#create_psbt) | **Post** /psbt/create | Create PSBT
*BitcoinApi* | [**create_raw_transaction**](docs/BitcoinApi.md#create_raw_transaction) | **Post** /tx/create | Create Raw Transaction
*BitcoinApi* | [**decode_psbt**](docs/BitcoinApi.md#decode_psbt) | **Post** /psbt/decode | Decode PSBT
*BitcoinApi* | [**decode_script**](docs/BitcoinApi.md#decode_script) | **Post** /script/decode | Decode Script
*BitcoinApi* | [**estimate_raw_fee**](docs/BitcoinApi.md#estimate_raw_fee) | **Post** /fee/estimate-raw | Estimate Raw Fee
*BitcoinApi* | [**estimate_smart_fee**](docs/BitcoinApi.md#estimate_smart_fee) | **Post** /fee/estimate-smart | Estimate smart fee
*BitcoinApi* | [**get_block_by_hash_decoded**](docs/BitcoinApi.md#get_block_by_hash_decoded) | **Get** /block/hash/{hash}/decoded | Get block by hash (verbosity 2)
*BitcoinApi* | [**get_block_by_hash_hex**](docs/BitcoinApi.md#get_block_by_hash_hex) | **Get** /block/hash/{hash}/hex | Get block by hash (verbosity 0)
*BitcoinApi* | [**get_block_by_hash_prevout**](docs/BitcoinApi.md#get_block_by_hash_prevout) | **Get** /block/hash/{hash}/prevout | Get block by hash (verbosity 3)
*BitcoinApi* | [**get_block_by_hash_summary**](docs/BitcoinApi.md#get_block_by_hash_summary) | **Get** /block/hash/{hash}/summary | Get block by hash (verbosity 1)
*BitcoinApi* | [**get_block_by_height_decoded**](docs/BitcoinApi.md#get_block_by_height_decoded) | **Get** /block/height/{height}/decoded | Get block by height (verbosity 2)
*BitcoinApi* | [**get_block_by_height_hex**](docs/BitcoinApi.md#get_block_by_height_hex) | **Get** /block/height/{height}/hex | Get block by height (verbosity 0)
*BitcoinApi* | [**get_block_by_height_prevout**](docs/BitcoinApi.md#get_block_by_height_prevout) | **Get** /block/height/{height}/prevout | Get block by height (verbosity 3)
*BitcoinApi* | [**get_block_by_height_summary**](docs/BitcoinApi.md#get_block_by_height_summary) | **Get** /block/height/{height}/summary | Get block by height (verbosity 1)
*BitcoinApi* | [**get_block_stats**](docs/BitcoinApi.md#get_block_stats) | **Post** /block/stats | Get block stats
*BitcoinApi* | [**get_blockchain_info**](docs/BitcoinApi.md#get_blockchain_info) | **Get** /blockchain/info | Get blockchain information
*BitcoinApi* | [**get_chain_tx_stats**](docs/BitcoinApi.md#get_chain_tx_stats) | **Post** /chain/txstats | Get chain tx stats
*BitcoinApi* | [**get_difficulty**](docs/BitcoinApi.md#get_difficulty) | **Get** /difficulty | Get difficulty
*BitcoinApi* | [**get_mempool_ancestors**](docs/BitcoinApi.md#get_mempool_ancestors) | **Post** /mempool/ancestors | Get mempool ancestors
*BitcoinApi* | [**get_mempool_descendants**](docs/BitcoinApi.md#get_mempool_descendants) | **Post** /mempool/descendants | Get mempool descendants
*BitcoinApi* | [**get_mempool_info**](docs/BitcoinApi.md#get_mempool_info) | **Get** /mempool/info | Get mempool information
*BitcoinApi* | [**get_mining_info**](docs/BitcoinApi.md#get_mining_info) | **Get** /mining/info | Get mining information
*BitcoinApi* | [**get_network_hashps**](docs/BitcoinApi.md#get_network_hashps) | **Post** /network/hashps | Get network hash per second
*BitcoinApi* | [**get_raw_mempool**](docs/BitcoinApi.md#get_raw_mempool) | **Post** /mempool/raw | Get raw mempool
*BitcoinApi* | [**get_raw_transaction_decoded**](docs/BitcoinApi.md#get_raw_transaction_decoded) | **Get** /tx/{txid}/decoded | Get raw transaction (verbosity 1)
*BitcoinApi* | [**get_raw_transaction_hex**](docs/BitcoinApi.md#get_raw_transaction_hex) | **Get** /tx/{txid}/hex | Get raw transaction (verbosity 0)
*BitcoinApi* | [**get_raw_transaction_prevout**](docs/BitcoinApi.md#get_raw_transaction_prevout) | **Get** /tx/{txid}/prevout | Get raw transaction (verbosity 2)
*BitcoinApi* | [**get_tx_out**](docs/BitcoinApi.md#get_tx_out) | **Post** /tx/out | Get transaction output
*BitcoinApi* | [**get_tx_out_proof**](docs/BitcoinApi.md#get_tx_out_proof) | **Post** /tx/out/proof | Get transaction output proof
*BitcoinApi* | [**get_tx_out_set_info**](docs/BitcoinApi.md#get_tx_out_set_info) | **Post** /tx/out/set/info | Get transaction output set information
*BitcoinApi* | [**get_tx_spending_prevout**](docs/BitcoinApi.md#get_tx_spending_prevout) | **Post** /tx/spending/prevout | Get transaction spending prevout
*BitcoinApi* | [**join_psbts**](docs/BitcoinApi.md#join_psbts) | **Post** /psbt/join | Join PSBTs
*BitcoinApi* | [**send_raw_transaction**](docs/BitcoinApi.md#send_raw_transaction) | **Post** /tx/send | Send raw transaction
*BitcoinApi* | [**test_mempool_accept**](docs/BitcoinApi.md#test_mempool_accept) | **Post** /mempool/test-accept | Test mempool accept
*BitcoinApi* | [**validate_address**](docs/BitcoinApi.md#validate_address) | **Get** /address/{address}/validate | Validate address
*BitcoinApi* | [**verify_message**](docs/BitcoinApi.md#verify_message) | **Post** /address/verify-message | Verify message
*BitcoinApi* | [**verify_tx_out_proof**](docs/BitcoinApi.md#verify_tx_out_proof) | **Post** /tx/out/proof/verify | Verify transaction output proof
*BlocksApi* | [**get_block_by_hash**](docs/BlocksApi.md#get_block_by_hash) | **Get** /block/hash/{block_hash} | Get block info by hash
*InscriptionsApi* | [**decode_tx**](docs/InscriptionsApi.md#decode_tx) | **Get** /decode/{txid} | Decode a transaction
*InscriptionsApi* | [**fetch_inscription_child**](docs/InscriptionsApi.md#fetch_inscription_child) | **Get** /inscription/{inscription_id}/child/{child_index} | Get inscription child info
*InscriptionsApi* | [**fetch_inscriptions**](docs/InscriptionsApi.md#fetch_inscriptions) | **Post** /inscriptions | Fetch multiple inscriptions
*InscriptionsApi* | [**get_address**](docs/InscriptionsApi.md#get_address) | **Get** /address/{address} | Get address info
*InscriptionsApi* | [**get_address_utxos**](docs/InscriptionsApi.md#get_address_utxos) | **Get** /address/{address}/outputs | Get UTXOs for an address
*InscriptionsApi* | [**get_block_by_height**](docs/InscriptionsApi.md#get_block_by_height) | **Get** /block/height/{block_height} | Get block info by height
*InscriptionsApi* | [**get_block_count**](docs/InscriptionsApi.md#get_block_count) | **Get** /blockcount | Get the height of the latest block
*InscriptionsApi* | [**get_block_hash_by_height**](docs/InscriptionsApi.md#get_block_hash_by_height) | **Get** /blockhash/{block_height} | Returns blockhash of specified block.
*InscriptionsApi* | [**get_block_inscriptions**](docs/InscriptionsApi.md#get_block_inscriptions) | **Get** /inscriptions/block/{block_height} | Get inscriptions in a specific block
*InscriptionsApi* | [**get_blocks**](docs/InscriptionsApi.md#get_blocks) | **Get** /blocks | Returns the latest block height, last 100 block hashes, and featured inscriptions
*InscriptionsApi* | [**get_inscription**](docs/InscriptionsApi.md#get_inscription) | **Get** /inscription/{inscription_id} | Get inscription info
*InscriptionsApi* | [**get_latest_block_height**](docs/InscriptionsApi.md#get_latest_block_height) | **Get** /latestblockheight | Returns the height of the latest block.
*InscriptionsApi* | [**get_latest_blockhash**](docs/InscriptionsApi.md#get_latest_blockhash) | **Get** /latestblockhash | Returns blockhash for the latest block.
*InscriptionsApi* | [**get_latest_blocktime**](docs/InscriptionsApi.md#get_latest_blocktime) | **Get** /blocktime | Get the timestamp of the latest block
*InscriptionsApi* | [**get_latest_inscriptions**](docs/InscriptionsApi.md#get_latest_inscriptions) | **Get** /inscriptions/latest | Get latest inscriptions
*InscriptionsApi* | [**get_latest_inscriptions_page**](docs/InscriptionsApi.md#get_latest_inscriptions_page) | **Get** /inscriptions/page/{page} | Get latest inscriptions page
*InscriptionsApi* | [**get_latest_runes**](docs/InscriptionsApi.md#get_latest_runes) | **Get** /runes/latest | Get latest runes
*InscriptionsApi* | [**get_latest_runes_page**](docs/InscriptionsApi.md#get_latest_runes_page) | **Get** /runes/page/{page} | Get latest runes page
*InscriptionsApi* | [**get_output_by_outpoint**](docs/InscriptionsApi.md#get_output_by_outpoint) | **Get** /output/outpoint/{outpoint} | Get output info by outpoint
*InscriptionsApi* | [**get_outputs**](docs/InscriptionsApi.md#get_outputs) | **Post** /outputs | Get multiple outputs
*InscriptionsApi* | [**get_rune**](docs/InscriptionsApi.md#get_rune) | **Get** /rune/{rune_name} | Get rune info
*InscriptionsApi* | [**get_satoshi**](docs/InscriptionsApi.md#get_satoshi) | **Get** /sat/{number} | Get satoshi info
*InscriptionsApi* | [**get_status**](docs/InscriptionsApi.md#get_status) | **Get** /status | Get server status
*InscriptionsApi* | [**get_transaction**](docs/InscriptionsApi.md#get_transaction) | **Get** /tx/{txid} | Get transaction info


## Documentation For Models

 - [GithubComSatstreamSsApiServerApiAddressResponsesAddressResponse](docs/GithubComSatstreamSsApiServerApiAddressResponsesAddressResponse.md)
 - [GithubComSatstreamSsApiServerApiAddressResponsesError](docs/GithubComSatstreamSsApiServerApiAddressResponsesError.md)
 - [GithubComSatstreamSsApiServerApiAddressResponsesOutputResponse](docs/GithubComSatstreamSsApiServerApiAddressResponsesOutputResponse.md)
 - [GithubComSatstreamSsApiServerApiBlockResponsesBlockCountResponse](docs/GithubComSatstreamSsApiServerApiBlockResponsesBlockCountResponse.md)
 - [GithubComSatstreamSsApiServerApiBlockResponsesBlockHashResponse](docs/GithubComSatstreamSsApiServerApiBlockResponsesBlockHashResponse.md)
 - [GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse](docs/GithubComSatstreamSsApiServerApiBlockResponsesBlockResponse.md)
 - [GithubComSatstreamSsApiServerApiBlockResponsesBlocksResponse](docs/GithubComSatstreamSsApiServerApiBlockResponsesBlocksResponse.md)
 - [GithubComSatstreamSsApiServerApiBlockResponsesError](docs/GithubComSatstreamSsApiServerApiBlockResponsesError.md)
 - [GithubComSatstreamSsApiServerApiFeeResponsesError](docs/GithubComSatstreamSsApiServerApiFeeResponsesError.md)
 - [GithubComSatstreamSsApiServerApiInscriptionResponsesError](docs/GithubComSatstreamSsApiServerApiInscriptionResponsesError.md)
 - [GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse](docs/GithubComSatstreamSsApiServerApiInscriptionResponsesInscriptionResponse.md)
 - [GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse](docs/GithubComSatstreamSsApiServerApiInscriptionResponsesLatestInscriptionsResponse.md)
 - [GithubComSatstreamSsApiServerApiMempoolResponsesError](docs/GithubComSatstreamSsApiServerApiMempoolResponsesError.md)
 - [GithubComSatstreamSsApiServerApiMiningResponsesError](docs/GithubComSatstreamSsApiServerApiMiningResponsesError.md)
 - [GithubComSatstreamSsApiServerApiNetworkResponsesError](docs/GithubComSatstreamSsApiServerApiNetworkResponsesError.md)
 - [GithubComSatstreamSsApiServerApiOutputResponsesError](docs/GithubComSatstreamSsApiServerApiOutputResponsesError.md)
 - [GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtInput](docs/GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtInput.md)
 - [GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput](docs/GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtOutput.md)
 - [GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtRequest](docs/GithubComSatstreamSsApiServerApiPsbtRequestsCreatePsbtRequest.md)
 - [GithubComSatstreamSsApiServerApiPsbtResponsesError](docs/GithubComSatstreamSsApiServerApiPsbtResponsesError.md)
 - [GithubComSatstreamSsApiServerApiRuneResponsesError](docs/GithubComSatstreamSsApiServerApiRuneResponsesError.md)
 - [GithubComSatstreamSsApiServerApiRuneResponsesRuneResponse](docs/GithubComSatstreamSsApiServerApiRuneResponsesRuneResponse.md)
 - [GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse](docs/GithubComSatstreamSsApiServerApiRuneResponsesRunesListResponse.md)
 - [GithubComSatstreamSsApiServerApiSatoshiResponsesError](docs/GithubComSatstreamSsApiServerApiSatoshiResponsesError.md)
 - [GithubComSatstreamSsApiServerApiSatoshiResponsesSatoshiResponse](docs/GithubComSatstreamSsApiServerApiSatoshiResponsesSatoshiResponse.md)
 - [GithubComSatstreamSsApiServerApiScriptResponsesError](docs/GithubComSatstreamSsApiServerApiScriptResponsesError.md)
 - [GithubComSatstreamSsApiServerApiStatusResponsesError](docs/GithubComSatstreamSsApiServerApiStatusResponsesError.md)
 - [GithubComSatstreamSsApiServerApiStatusResponsesStatusResponse](docs/GithubComSatstreamSsApiServerApiStatusResponsesStatusResponse.md)
 - [GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtInput](docs/GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtInput.md)
 - [GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtOutput](docs/GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtOutput.md)
 - [GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtRequest](docs/GithubComSatstreamSsApiServerApiTransactionRequestsCreatePsbtRequest.md)
 - [GithubComSatstreamSsApiServerApiTransactionResponsesDecodeResponse](docs/GithubComSatstreamSsApiServerApiTransactionResponsesDecodeResponse.md)
 - [GithubComSatstreamSsApiServerApiTransactionResponsesError](docs/GithubComSatstreamSsApiServerApiTransactionResponsesError.md)
 - [GithubComSatstreamSsApiServerApiTransactionResponsesTransactionResponse](docs/GithubComSatstreamSsApiServerApiTransactionResponsesTransactionResponse.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBip32Deriv](docs/GithubComSatstreamSsUtilsBitcoinCliBip32Deriv.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlock1](docs/GithubComSatstreamSsUtilsBitcoinCliBlock1.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlock2](docs/GithubComSatstreamSsUtilsBitcoinCliBlock2.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlock3](docs/GithubComSatstreamSsUtilsBitcoinCliBlock3.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlockStats](docs/GithubComSatstreamSsUtilsBitcoinCliBlockStats.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlockVin2](docs/GithubComSatstreamSsUtilsBitcoinCliBlockVin2.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlockVin3](docs/GithubComSatstreamSsUtilsBitcoinCliBlockVin3.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBlockchainInfo](docs/GithubComSatstreamSsUtilsBitcoinCliBlockchainInfo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBtcTx2](docs/GithubComSatstreamSsUtilsBitcoinCliBtcTx2.md)
 - [GithubComSatstreamSsUtilsBitcoinCliBtcTx3](docs/GithubComSatstreamSsUtilsBitcoinCliBtcTx3.md)
 - [GithubComSatstreamSsUtilsBitcoinCliChainTxStats](docs/GithubComSatstreamSsUtilsBitcoinCliChainTxStats.md)
 - [GithubComSatstreamSsUtilsBitcoinCliDecodedPsbt](docs/GithubComSatstreamSsUtilsBitcoinCliDecodedPsbt.md)
 - [GithubComSatstreamSsUtilsBitcoinCliDecodedPsbtInput](docs/GithubComSatstreamSsUtilsBitcoinCliDecodedPsbtInput.md)
 - [GithubComSatstreamSsUtilsBitcoinCliDecodedPsbtOutput](docs/GithubComSatstreamSsUtilsBitcoinCliDecodedPsbtOutput.md)
 - [GithubComSatstreamSsUtilsBitcoinCliDecodedScript](docs/GithubComSatstreamSsUtilsBitcoinCliDecodedScript.md)
 - [GithubComSatstreamSsUtilsBitcoinCliEstimateRawFee](docs/GithubComSatstreamSsUtilsBitcoinCliEstimateRawFee.md)
 - [GithubComSatstreamSsUtilsBitcoinCliFeeHorizonEstimate](docs/GithubComSatstreamSsUtilsBitcoinCliFeeHorizonEstimate.md)
 - [GithubComSatstreamSsUtilsBitcoinCliFeeRange](docs/GithubComSatstreamSsUtilsBitcoinCliFeeRange.md)
 - [GithubComSatstreamSsUtilsBitcoinCliMempoolInfo](docs/GithubComSatstreamSsUtilsBitcoinCliMempoolInfo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliMiningInfo](docs/GithubComSatstreamSsUtilsBitcoinCliMiningInfo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPrevOut](docs/GithubComSatstreamSsUtilsBitcoinCliPrevOut.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPsbtAnalysis](docs/GithubComSatstreamSsUtilsBitcoinCliPsbtAnalysis.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPsbtBip32Deriv](docs/GithubComSatstreamSsUtilsBitcoinCliPsbtBip32Deriv.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPsbtInputAnalysis](docs/GithubComSatstreamSsUtilsBitcoinCliPsbtInputAnalysis.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPsbtMissingData](docs/GithubComSatstreamSsUtilsBitcoinCliPsbtMissingData.md)
 - [GithubComSatstreamSsUtilsBitcoinCliPsbtWitnessUtxo](docs/GithubComSatstreamSsUtilsBitcoinCliPsbtWitnessUtxo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliRawTx1](docs/GithubComSatstreamSsUtilsBitcoinCliRawTx1.md)
 - [GithubComSatstreamSsUtilsBitcoinCliRawTx2](docs/GithubComSatstreamSsUtilsBitcoinCliRawTx2.md)
 - [GithubComSatstreamSsUtilsBitcoinCliScript](docs/GithubComSatstreamSsUtilsBitcoinCliScript.md)
 - [GithubComSatstreamSsUtilsBitcoinCliScriptPubKey](docs/GithubComSatstreamSsUtilsBitcoinCliScriptPubKey.md)
 - [GithubComSatstreamSsUtilsBitcoinCliScriptSig](docs/GithubComSatstreamSsUtilsBitcoinCliScriptSig.md)
 - [GithubComSatstreamSsUtilsBitcoinCliSegwitDetails](docs/GithubComSatstreamSsUtilsBitcoinCliSegwitDetails.md)
 - [GithubComSatstreamSsUtilsBitcoinCliSmartFeeEstimate](docs/GithubComSatstreamSsUtilsBitcoinCliSmartFeeEstimate.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTestMempoolAcceptResult](docs/GithubComSatstreamSsUtilsBitcoinCliTestMempoolAcceptResult.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTestMempoolFees](docs/GithubComSatstreamSsUtilsBitcoinCliTestMempoolFees.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTxOut](docs/GithubComSatstreamSsUtilsBitcoinCliTxOut.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTxSpendingPrevoutInput](docs/GithubComSatstreamSsUtilsBitcoinCliTxSpendingPrevoutInput.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTxSpendingPrevoutResult](docs/GithubComSatstreamSsUtilsBitcoinCliTxSpendingPrevoutResult.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTxVin1](docs/GithubComSatstreamSsUtilsBitcoinCliTxVin1.md)
 - [GithubComSatstreamSsUtilsBitcoinCliTxVin2](docs/GithubComSatstreamSsUtilsBitcoinCliTxVin2.md)
 - [GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo](docs/GithubComSatstreamSsUtilsBitcoinCliUtxoBlockInfo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliUtxoSetInfo](docs/GithubComSatstreamSsUtilsBitcoinCliUtxoSetInfo.md)
 - [GithubComSatstreamSsUtilsBitcoinCliUtxoUnspendables](docs/GithubComSatstreamSsUtilsBitcoinCliUtxoUnspendables.md)
 - [GithubComSatstreamSsUtilsBitcoinCliValidateAddressResult](docs/GithubComSatstreamSsUtilsBitcoinCliValidateAddressResult.md)
 - [GithubComSatstreamSsUtilsBitcoinCliVout](docs/GithubComSatstreamSsUtilsBitcoinCliVout.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesAddressResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesAddressResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesBlockResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesBlockResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesBlocksResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesBlocksResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesDecodeResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesDecodeResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription](docs/GithubComSatstreamSsUtilsOrdServerResponsesDecodedInscription.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesDuration](docs/GithubComSatstreamSsUtilsOrdServerResponsesDuration.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesInput](docs/GithubComSatstreamSsUtilsOrdServerResponsesInput.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData](docs/GithubComSatstreamSsUtilsOrdServerResponsesInscriptionData.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesInscriptionResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesInscriptionResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesLatestInscriptionsResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesLatestInscriptionsResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesOutput](docs/GithubComSatstreamSsUtilsOrdServerResponsesOutput.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesOutputResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesOutputResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRuneEntry](docs/GithubComSatstreamSsUtilsOrdServerResponsesRuneEntry.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRuneListEntry](docs/GithubComSatstreamSsUtilsOrdServerResponsesRuneListEntry.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRuneResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesRuneResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRuneTerms](docs/GithubComSatstreamSsUtilsOrdServerResponsesRuneTerms.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRunesBalance](docs/GithubComSatstreamSsUtilsOrdServerResponsesRunesBalance.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRunesListResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesRunesListResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesRunestoneData](docs/GithubComSatstreamSsUtilsOrdServerResponsesRunestoneData.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesSatoshiResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesStatusResponse.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesTransaction](docs/GithubComSatstreamSsUtilsOrdServerResponsesTransaction.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails](docs/GithubComSatstreamSsUtilsOrdServerResponsesTransactionDetails.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput](docs/GithubComSatstreamSsUtilsOrdServerResponsesTransactionInput.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput](docs/GithubComSatstreamSsUtilsOrdServerResponsesTransactionOutput.md)
 - [GithubComSatstreamSsUtilsOrdServerResponsesTransactionResponse](docs/GithubComSatstreamSsUtilsOrdServerResponsesTransactionResponse.md)
 - [GithubComSatstreamSsutilsBitcoincliBlockVin2ScriptSig](docs/GithubComSatstreamSsutilsBitcoincliBlockVin2ScriptSig.md)
 - [GithubComSatstreamSsutilsBitcoincliBlockVin3Prevout](docs/GithubComSatstreamSsutilsBitcoincliBlockVin3Prevout.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputFinalScriptsig](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputFinalScriptsig.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputNonWitnessUtxo](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputNonWitnessUtxo.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputRedeemScript](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputRedeemScript.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputWitnessScript](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputWitnessScript.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputWitnessUtxo](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtInputWitnessUtxo.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedPsbtTx](docs/GithubComSatstreamSsutilsBitcoincliDecodedPsbtTx.md)
 - [GithubComSatstreamSsutilsBitcoincliDecodedScriptSegwit](docs/GithubComSatstreamSsutilsBitcoincliDecodedScriptSegwit.md)
 - [GithubComSatstreamSsutilsBitcoincliEstimateRawFeeLong](docs/GithubComSatstreamSsutilsBitcoincliEstimateRawFeeLong.md)
 - [GithubComSatstreamSsutilsBitcoincliEstimateRawFeeMedium](docs/GithubComSatstreamSsutilsBitcoincliEstimateRawFeeMedium.md)
 - [GithubComSatstreamSsutilsBitcoincliEstimateRawFeeShort](docs/GithubComSatstreamSsutilsBitcoincliEstimateRawFeeShort.md)
 - [GithubComSatstreamSsutilsBitcoincliFeeHorizonEstimateFail](docs/GithubComSatstreamSsutilsBitcoincliFeeHorizonEstimateFail.md)
 - [GithubComSatstreamSsutilsBitcoincliFeeHorizonEstimatePass](docs/GithubComSatstreamSsutilsBitcoincliFeeHorizonEstimatePass.md)
 - [GithubComSatstreamSsutilsBitcoincliPsbtWitnessUtxoScriptPubKey](docs/GithubComSatstreamSsutilsBitcoincliPsbtWitnessUtxoScriptPubKey.md)
 - [GithubComSatstreamSsutilsBitcoincliTestMempoolAcceptResultFees](docs/GithubComSatstreamSsutilsBitcoincliTestMempoolAcceptResultFees.md)
 - [GithubComSatstreamSsutilsBitcoincliTxOutScriptPubKey](docs/GithubComSatstreamSsutilsBitcoincliTxOutScriptPubKey.md)
 - [GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables](docs/GithubComSatstreamSsutilsBitcoincliUtxoBlockInfoUnspendables.md)
 - [GithubComSatstreamSsutilsBitcoincliUtxoSetInfoBlockInfo](docs/GithubComSatstreamSsutilsBitcoincliUtxoSetInfoBlockInfo.md)
 - [GithubComSatstreamSsutilsOrdServerResponsesRuneListEntry1](docs/GithubComSatstreamSsutilsOrdServerResponsesRuneListEntry1.md)
 - [RequestsAnalyzePsbtRequest](docs/RequestsAnalyzePsbtRequest.md)
 - [RequestsCombinePsbtRequest](docs/RequestsCombinePsbtRequest.md)
 - [RequestsCombineRawTransactionRequest](docs/RequestsCombineRawTransactionRequest.md)
 - [RequestsConvertToPsbtRequest](docs/RequestsConvertToPsbtRequest.md)
 - [RequestsDecodeScriptRequest](docs/RequestsDecodeScriptRequest.md)
 - [RequestsEstimateRawFeeRequest](docs/RequestsEstimateRawFeeRequest.md)
 - [RequestsEstimateSmartFeeRequest](docs/RequestsEstimateSmartFeeRequest.md)
 - [RequestsGetBlockStatsRequest](docs/RequestsGetBlockStatsRequest.md)
 - [RequestsGetChainTxStatsRequest](docs/RequestsGetChainTxStatsRequest.md)
 - [RequestsGetMempoolAncestorsRequest](docs/RequestsGetMempoolAncestorsRequest.md)
 - [RequestsGetMempoolDescendantsRequest](docs/RequestsGetMempoolDescendantsRequest.md)
 - [RequestsGetNetworkHashPsRequest](docs/RequestsGetNetworkHashPsRequest.md)
 - [RequestsGetRawMempoolRequest](docs/RequestsGetRawMempoolRequest.md)
 - [RequestsGetTxOutProofRequest](docs/RequestsGetTxOutProofRequest.md)
 - [RequestsGetTxOutRequest](docs/RequestsGetTxOutRequest.md)
 - [RequestsGetTxOutSetInfoRequest](docs/RequestsGetTxOutSetInfoRequest.md)
 - [RequestsGetTxSpendingPrevoutRequest](docs/RequestsGetTxSpendingPrevoutRequest.md)
 - [RequestsJoinPsbtsRequest](docs/RequestsJoinPsbtsRequest.md)
 - [RequestsSendRawTransactionRequest](docs/RequestsSendRawTransactionRequest.md)
 - [RequestsTestMempoolAcceptRequest](docs/RequestsTestMempoolAcceptRequest.md)
 - [RequestsVerifyMessageRequest](docs/RequestsVerifyMessageRequest.md)
 - [RequestsVerifyTxOutProofRequest](docs/RequestsVerifyTxOutProofRequest.md)
 - [ResponsesAnalyzePsbtResponse](docs/ResponsesAnalyzePsbtResponse.md)
 - [ResponsesCombinePsbtResponse](docs/ResponsesCombinePsbtResponse.md)
 - [ResponsesCombineRawTransactionResponse](docs/ResponsesCombineRawTransactionResponse.md)
 - [ResponsesConvertToPsbtResponse](docs/ResponsesConvertToPsbtResponse.md)
 - [ResponsesCreatePsbtResponse](docs/ResponsesCreatePsbtResponse.md)
 - [ResponsesCreateRawTransactionResponse](docs/ResponsesCreateRawTransactionResponse.md)
 - [ResponsesDecodePsbtResponse](docs/ResponsesDecodePsbtResponse.md)
 - [ResponsesDecodePsbtResponseData](docs/ResponsesDecodePsbtResponseData.md)
 - [ResponsesDecodeScriptResponse](docs/ResponsesDecodeScriptResponse.md)
 - [ResponsesDecodeScriptResponseData](docs/ResponsesDecodeScriptResponseData.md)
 - [ResponsesEstimateRawFeeResponse](docs/ResponsesEstimateRawFeeResponse.md)
 - [ResponsesEstimateRawFeeResponseData](docs/ResponsesEstimateRawFeeResponseData.md)
 - [ResponsesEstimateSmartFeeResponse](docs/ResponsesEstimateSmartFeeResponse.md)
 - [ResponsesEstimateSmartFeeResponseData](docs/ResponsesEstimateSmartFeeResponseData.md)
 - [ResponsesGetBlockDecodedResponse](docs/ResponsesGetBlockDecodedResponse.md)
 - [ResponsesGetBlockHexResponse](docs/ResponsesGetBlockHexResponse.md)
 - [ResponsesGetBlockPrevoutResponse](docs/ResponsesGetBlockPrevoutResponse.md)
 - [ResponsesGetBlockStatsResponse](docs/ResponsesGetBlockStatsResponse.md)
 - [ResponsesGetBlockStatsResponseData](docs/ResponsesGetBlockStatsResponseData.md)
 - [ResponsesGetBlockSummaryResponse](docs/ResponsesGetBlockSummaryResponse.md)
 - [ResponsesGetBlockchainInfoResponse](docs/ResponsesGetBlockchainInfoResponse.md)
 - [ResponsesGetBlockchainInfoResponseData](docs/ResponsesGetBlockchainInfoResponseData.md)
 - [ResponsesGetChainTxStatsResponse](docs/ResponsesGetChainTxStatsResponse.md)
 - [ResponsesGetChainTxStatsResponseData](docs/ResponsesGetChainTxStatsResponseData.md)
 - [ResponsesGetDifficultyResponse](docs/ResponsesGetDifficultyResponse.md)
 - [ResponsesGetMempoolAncestorsResponse](docs/ResponsesGetMempoolAncestorsResponse.md)
 - [ResponsesGetMempoolDescendantsResponse](docs/ResponsesGetMempoolDescendantsResponse.md)
 - [ResponsesGetMempoolInfoResponse](docs/ResponsesGetMempoolInfoResponse.md)
 - [ResponsesGetMempoolInfoResponseData](docs/ResponsesGetMempoolInfoResponseData.md)
 - [ResponsesGetMiningInfoResponse](docs/ResponsesGetMiningInfoResponse.md)
 - [ResponsesGetMiningInfoResponseData](docs/ResponsesGetMiningInfoResponseData.md)
 - [ResponsesGetNetworkHashPsResponse](docs/ResponsesGetNetworkHashPsResponse.md)
 - [ResponsesGetOutputByOutpointResponse](docs/ResponsesGetOutputByOutpointResponse.md)
 - [ResponsesGetOutputsResponse](docs/ResponsesGetOutputsResponse.md)
 - [ResponsesGetRawMempoolResponse](docs/ResponsesGetRawMempoolResponse.md)
 - [ResponsesGetRawTransactionDecodedResponse](docs/ResponsesGetRawTransactionDecodedResponse.md)
 - [ResponsesGetRawTransactionHexResponse](docs/ResponsesGetRawTransactionHexResponse.md)
 - [ResponsesGetRawTransactionPrevoutResponse](docs/ResponsesGetRawTransactionPrevoutResponse.md)
 - [ResponsesGetTxOutProofResponse](docs/ResponsesGetTxOutProofResponse.md)
 - [ResponsesGetTxOutResponse](docs/ResponsesGetTxOutResponse.md)
 - [ResponsesGetTxOutResponseData](docs/ResponsesGetTxOutResponseData.md)
 - [ResponsesGetTxOutSetInfoResponse](docs/ResponsesGetTxOutSetInfoResponse.md)
 - [ResponsesGetTxOutSetInfoResponseData](docs/ResponsesGetTxOutSetInfoResponseData.md)
 - [ResponsesGetTxSpendingPrevoutResponse](docs/ResponsesGetTxSpendingPrevoutResponse.md)
 - [ResponsesJoinPsbtsResponse](docs/ResponsesJoinPsbtsResponse.md)
 - [ResponsesLatestBlockHashResponse](docs/ResponsesLatestBlockHashResponse.md)
 - [ResponsesLatestBlockHeightResponse](docs/ResponsesLatestBlockHeightResponse.md)
 - [ResponsesLatestBlockTimeResponse](docs/ResponsesLatestBlockTimeResponse.md)
 - [ResponsesSendRawTransactionResponse](docs/ResponsesSendRawTransactionResponse.md)
 - [ResponsesTestMempoolAcceptResponse](docs/ResponsesTestMempoolAcceptResponse.md)
 - [ResponsesValidateAddressResponse](docs/ResponsesValidateAddressResponse.md)
 - [ResponsesValidateAddressResponseData](docs/ResponsesValidateAddressResponseData.md)
 - [ResponsesVerifyMessageResponse](docs/ResponsesVerifyMessageResponse.md)
 - [ResponsesVerifyTxOutProofResponse](docs/ResponsesVerifyTxOutProofResponse.md)


## Documentation For Authorization

## ApiKeyAuth
- **Type**: API key 

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextAPIKey, sw.APIKey{
		Key: "APIKEY",
		Prefix: "Bearer", // Omit if not necessary.
	})
    r, err := client.Service.Operation(auth, args)
```

## Author

team@satstream.io

