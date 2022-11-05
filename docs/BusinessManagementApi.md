# WhatsAPI\BusinessManagementApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_catlog**](BusinessManagementApi.md#fetch_catlog) | **GET** /instances/{instance_key}/business/catalog | Fetches the catlog.



## fetch_catlog

> crate::models::ApiResponse fetch_catlog(instance_key)
Fetches the catlog.

Gets list of all products registered by you.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

