# FeeHorizonEstimate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**decay** | **f32** | Exponential decay (per block) for historical moving average | [optional] [default to null]
**errors** | **Vec<String>** | Errors encountered during processing | [optional] [default to null]
**fail** | [***::models::FeeHorizonEstimateFail**](FeeHorizonEstimate_fail.md) |  | [optional] [default to null]
**feerate** | **f32** | Estimate fee rate in BTC/kvB | [optional] [default to null]
**pass** | [***::models::FeeHorizonEstimatePass**](FeeHorizonEstimate_pass.md) |  | [optional] [default to null]
**scale** | **f32** | Resolution of confirmation targets at this time horizon | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


