# WhatsAPI\MiscellaneousApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile_pic**](MiscellaneousApi.md#get_profile_pic) | **GET** /instances/{instance_key}/misc/profile-pic | Get profile pic.
[**get_users_info**](MiscellaneousApi.md#get_users_info) | **POST** /instances/{instance_key}/misc/user-info | Fetches the users info.



## get_profile_pic

> crate::models::ApiResponse get_profile_pic(instance_key, jid)
Get profile pic.

Returns the profile pic of the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**jid** | **String** | JID | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_info

> crate::models::ApiResponse get_users_info(instance_key, data)
Fetches the users info.

Gets the user info for the given user ids. This does not checks if user is registered or not

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**UserInfoPayload**](UserInfoPayload.md) | Data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

