# WhatsAPI\GroupManagementApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_participant**](GroupManagementApi.md#add_participant) | **POST** /instances/{instance_key}/groups/{group_id}/participants/add | Add participant.
[**create_group**](GroupManagementApi.md#create_group) | **POST** /instances/{instance_key}/groups/create | Create group.
[**demote_participant**](GroupManagementApi.md#demote_participant) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/demote | Demote participant.
[**get_admin_groups**](GroupManagementApi.md#get_admin_groups) | **GET** /instances/{instance_key}/groups/admin | Get admin groups.
[**get_all_groups**](GroupManagementApi.md#get_all_groups) | **GET** /instances/{instance_key}/groups/ | Get all groups.
[**get_all_participants**](GroupManagementApi.md#get_all_participants) | **GET** /instances/{instance_key}/groups/{group_id}/participants | Get all participants.
[**get_group**](GroupManagementApi.md#get_group) | **GET** /instances/{instance_key}/groups/{group_id} | Get group.
[**get_group_from_invite_link**](GroupManagementApi.md#get_group_from_invite_link) | **GET** /instances/{instance_key}/groups/invite-info | Get group from invite link.
[**get_group_invite_code**](GroupManagementApi.md#get_group_invite_code) | **GET** /instances/{instance_key}/groups/{group_id}/invite-code | Get group invite code.
[**join_group_with_link**](GroupManagementApi.md#join_group_with_link) | **GET** /instances/{instance_key}/groups/join | Join group with invite code.
[**leave_group**](GroupManagementApi.md#leave_group) | **DELETE** /instances/{instance_key}/groups/{group_id}/ | Leaves the group.
[**promote_participant**](GroupManagementApi.md#promote_participant) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/promote | Promote participant.
[**remove_participant**](GroupManagementApi.md#remove_participant) | **DELETE** /instances/{instance_key}/groups/{group_id}/participants/remove | Remove participant.
[**set_group_announce**](GroupManagementApi.md#set_group_announce) | **PUT** /instances/{instance_key}/groups/{group_id}/announce | Set group announce.
[**set_group_description**](GroupManagementApi.md#set_group_description) | **PUT** /instances/{instance_key}/groups/{group_id}/description | Set group description.
[**set_group_locked**](GroupManagementApi.md#set_group_locked) | **PUT** /instances/{instance_key}/groups/{group_id}/lock | Set group locked.
[**set_group_name**](GroupManagementApi.md#set_group_name) | **PUT** /instances/{instance_key}/groups/{group_id}/name | Set group name.
[**set_group_picture**](GroupManagementApi.md#set_group_picture) | **PUT** /instances/{instance_key}/groups/{group_id}/profile-pic | Set group picture.



## add_participant

> crate::models::ApiResponse add_participant(instance_key, group_id, data)
Add participant.

Handles adding participants to a group. You must be admin in the group or the query will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateParticipantsPayload**](GroupUpdateParticipantsPayload.md) | Group update payload | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::ApiResponse create_group(instance_key, data)
Create group.

Creates a group with the participant data. The creator is automatically added to the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**GroupCreatePayload**](GroupCreatePayload.md) | Group create payload | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## demote_participant

> crate::models::ApiResponse demote_participant(instance_key, group_id, data)
Demote participant.

Demotes admins in groups. You must be admin in the group or the query will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateParticipantsPayload**](GroupUpdateParticipantsPayload.md) | Group update payload | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_admin_groups

> crate::models::ApiResponse get_admin_groups(instance_key)
Get admin groups.

Returns list of all groups in which you are admin.

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


## get_all_groups

> crate::models::ApiResponse get_all_groups(instance_key, include_participants)
Get all groups.

Returns list of all groups with participants data. Set include_participants to false to exclude participants data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**include_participants** | Option<**String**> | Include participants data |  |[default to true]

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_participants

> crate::models::ApiResponse get_all_participants(instance_key, group_id)
Get all participants.

Returns all participants of the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::ApiResponse get_group(instance_key, group_id)
Get group.

Fetches the group data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_from_invite_link

> crate::models::ApiResponse get_group_from_invite_link(instance_key, invite_link)
Get group from invite link.

Gets a group info from an invite link. An invite link is a link that can be used to join a group. It is usually in the format https://chat.whatsapp.com/{invitecode}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**invite_link** | **String** | The invite link to check | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_invite_code

> crate::models::ApiResponse get_group_invite_code(instance_key, group_id)
Get group invite code.

Gets the invite code of the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_group_with_link

> crate::models::ApiResponse join_group_with_link(instance_key, invite_code)
Join group with invite code.

Joins a group with group invite link. An invite link is a link that can be used to join a group. It is usually in the format https://chat.whatsapp.com/{invitecode} You have to put invite_code in the url of the request. The invite code is the part after https://chat.whatsapp.com/ For example, if the invite link is https://chat.whatsapp.com/dsfsf34r3d3dsds, then the invite code is `dsfsf34r3d3dsds“

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**invite_code** | **String** | The invite code of group you want to join | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_group

> crate::models::ApiResponse leave_group(instance_key, group_id)
Leaves the group.

Leaves the specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_participant

> crate::models::ApiResponse promote_participant(instance_key, group_id, data)
Promote participant.

Promotes participants to admin. You must be admin in the group or the query will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateParticipantsPayload**](GroupUpdateParticipantsPayload.md) | Group update payload | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_participant

> crate::models::ApiResponse remove_participant(instance_key, group_id, data)
Remove participant.

Handles removing participants from a group. You must be admin in the group or the query will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateParticipantsPayload**](GroupUpdateParticipantsPayload.md) | Group update payload | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_announce

> crate::models::ApiResponse set_group_announce(instance_key, announce, group_id)
Set group announce.

Set if non-admins are allowed to send messages in groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**announce** | **bool** | Announce status | [required] |[default to false]
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_description

> crate::models::ApiResponse set_group_description(instance_key, group_id, data)
Set group description.

Changes the group description

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateDescriptionPayload**](GroupUpdateDescriptionPayload.md) | Group description data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_locked

> crate::models::ApiResponse set_group_locked(instance_key, locked, group_id)
Set group locked.

Set if non-admins are allowed to change the group dp and other stuff

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**locked** | **bool** | Locked status | [required] |[default to false]
**group_id** | **String** | Group id of the group | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_name

> crate::models::ApiResponse set_group_name(instance_key, group_id, data)
Set group name.

Changes the group name. The max limit is 22 chars

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**data** | [**GroupUpdateNamePayload**](GroupUpdateNamePayload.md) | Group name data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_picture

> crate::models::ApiResponse set_group_picture(instance_key, group_id, set_group_picture_request)
Set group picture.

Changes the group profile picture. Currently it only seems to accept JPEG images only

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**set_group_picture_request** | [**SetGroupPictureRequest**](SetGroupPictureRequest.md) |  | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

