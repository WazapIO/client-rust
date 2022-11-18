# WhatsAPI\MiscellaneousApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_media**](MiscellaneousApi.md#download_media) | **POST** /instances/{instance_key}/misc/download | Download media
[**get_profile_pic**](MiscellaneousApi.md#get_profile_pic) | **GET** /instances/{instance_key}/misc/profile-pic | Get profile pic.
[**get_users_info**](MiscellaneousApi.md#get_users_info) | **POST** /instances/{instance_key}/misc/user-info | Fetches the users info.
[**set_chat_presence**](MiscellaneousApi.md#set_chat_presence) | **POST** /instances/{instance_key}/misc/chat-presence | Set chat presence
[**update_profile_pic**](MiscellaneousApi.md#update_profile_pic) | **PUT** /instances/{instance_key}/misc/profile-pic | Update profile picture



## download_media

> crate::models::ApiResponse download_media(instance_key, file_type, data, response_type)
Download media

Downloads the media from the given media keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**file_type** | **String** | File type | [required] |
**data** | [**FileUpload**](FileUpload.md) | Media data | [required] |
**response_type** | Option<**String**> | Response type (file, base64) |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## set_chat_presence

> crate::models::ApiResponse set_chat_presence(instance_key, jid, presence)
Set chat presence

Sets the presence of the given chat. (Typing, Recording, Paused) Options: typing, recording, paused

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**jid** | **String** | JID | [required] |
**presence** | **String** | Presence | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile_pic

> crate::models::ApiResponse update_profile_pic(instance_key, update_profile_pic_request)
Update profile picture

Changes the profile pic of the current logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**update_profile_pic_request** | [**UpdateProfilePicRequest**](UpdateProfilePicRequest.md) |  | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

