# SubTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**transaction_id** | **String** |  | 
**amount** | **i64** | The subtransaction amount in milliunits format | 
**memo** | **String** |  | [optional] 
**payee_id** | **String** |  | [optional] 
**category_id** | **String** |  | [optional] 
**transfer_account_id** | **String** | If a transfer, the account_id which the subtransaction transfers to | [optional] 
**deleted** | **bool** | Whether or not the subtransaction has been deleted.  Deleted subtransactions will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


