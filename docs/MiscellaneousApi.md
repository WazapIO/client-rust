# \MiscellaneousApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instances_instance_key_misc_profile_pic_get**](MiscellaneousApi.md#instances_instance_key_misc_profile_pic_get) | **GET** /instances/{instance_key}/misc/profile-pic | Get profile pic.
[**instances_instance_key_misc_user_info_post**](MiscellaneousApi.md#instances_instance_key_misc_user_info_post) | **POST** /instances/{instance_key}/misc/user-info | Fetches the users info.



## instances_instance_key_misc_profile_pic_get

> crate::models::MainPeriodApiResponse instances_instance_key_misc_profile_pic_get(instance_key, jid)
Get profile pic.

Returns the profile pic of the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**jid** | **String** | JID | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_misc_user_info_post

> crate::models::MainPeriodApiResponse instances_instance_key_misc_user_info_post(instance_key, data)
Fetches the users info.

Gets the user info for the given user ids. This does not checks if user is registered or not

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodUserInfoPayload**](StructsPeriodUserInfoPayload.md) | Data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

