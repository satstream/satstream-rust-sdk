# DecodedPsbtInput

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bip32_derivs** | [**::std::collections::HashMap<String, ::models::Bip32Deriv>**](Bip32Deriv.md) | The BIP32 derivation paths | [optional] [default to null]
**final_scriptsig** | [***::models::DecodedPsbtInputFinalScriptsig**](DecodedPSBTInput_final_scriptsig.md) |  | [optional] [default to null]
**final_scriptwitness** | **Vec<String>** | The final script witness | [optional] [default to null]
**non_witness_utxo** | [***::models::DecodedPsbtInputNonWitnessUtxo**](DecodedPSBTInput_non_witness_utxo.md) |  | [optional] [default to null]
**partial_signatures** | **::std::collections::HashMap<String, String>** | The public key and signature pairs | [optional] [default to null]
**redeem_script** | [***::models::DecodedPsbtInputRedeemScript**](DecodedPSBTInput_redeem_script.md) |  | [optional] [default to null]
**sighash** | **String** | The sighash type to be used | [optional] [default to null]
**unknown** | [**::std::collections::HashMap<String, ::models::Object>**](.md) | Unknown fields | [optional] [default to null]
**witness_script** | [***::models::DecodedPsbtInputWitnessScript**](DecodedPSBTInput_witness_script.md) |  | [optional] [default to null]
**witness_utxo** | [***::models::DecodedPsbtInputWitnessUtxo**](DecodedPSBTInput_witness_utxo.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


