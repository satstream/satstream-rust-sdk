# RequestsCreatePsbtRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inputs** | [**Vec<::models::RequestsCreatePsbtInput>**](requests.CreatePSBTInput.md) | The inputs for the transaction | [default to null]
**locktime** | **i32** | Raw locktime. Non-0 value also locktime-activates inputs | [optional] [default to null]
**outputs** | [**Vec<::models::RequestsCreatePsbtOutput>**](requests.CreatePSBTOutput.md) | The outputs for the transaction (each address can only appear once) | [default to null]
**replaceable** | **bool** | Marks this transaction as BIP125-replaceable | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


