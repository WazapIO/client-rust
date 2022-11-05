# WhatsAPI\GroupManagementApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instances_instance_key_groups_admin_get**](GroupManagementApi.md#instances_instance_key_groups_admin_get) | **GET** /instances/{instance_key}/groups/admin | Get admin groupss.
[**instances_instance_key_groups_create_post**](GroupManagementApi.md#instances_instance_key_groups_create_post) | **POST** /instances/{instance_key}/groups/create | Create group.
[**instances_instance_key_groups_get**](GroupManagementApi.md#instances_instance_key_groups_get) | **GET** /instances/{instance_key}/groups/ | Get all groups.
[**instances_instance_key_groups_group_id_announce_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_announce_put) | **PUT** /instances/{instance_key}/groups/{group_id}/announce | Set group announce.
[**instances_instance_key_groups_group_id_delete**](GroupManagementApi.md#instances_instance_key_groups_group_id_delete) | **DELETE** /instances/{instance_key}/groups/{group_id}/ | Leaves the group.
[**instances_instance_key_groups_group_id_description_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_description_put) | **PUT** /instances/{instance_key}/groups/{group_id}/description | Set group description.
[**instances_instance_key_groups_group_id_get**](GroupManagementApi.md#instances_instance_key_groups_group_id_get) | **GET** /instances/{instance_key}/groups/{group_id} | Get group.
[**instances_instance_key_groups_group_id_invite_code_get**](GroupManagementApi.md#instances_instance_key_groups_group_id_invite_code_get) | **GET** /instances/{instance_key}/groups/{group_id}/invite-code | Get group invite code.
[**instances_instance_key_groups_group_id_lock_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_lock_put) | **PUT** /instances/{instance_key}/groups/{group_id}/lock | Set group locked.
[**instances_instance_key_groups_group_id_name_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_name_put) | **PUT** /instances/{instance_key}/groups/{group_id}/name | Set group name.
[**instances_instance_key_groups_group_id_participants_add_post**](GroupManagementApi.md#instances_instance_key_groups_group_id_participants_add_post) | **POST** /instances/{instance_key}/groups/{group_id}/participants/add | Add participant.
[**instances_instance_key_groups_group_id_participants_demote_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_participants_demote_put) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/demote | Demote participant.
[**instances_instance_key_groups_group_id_participants_promote_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_participants_promote_put) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/promote | Promote participant.
[**instances_instance_key_groups_group_id_participants_remove_delete**](GroupManagementApi.md#instances_instance_key_groups_group_id_participants_remove_delete) | **DELETE** /instances/{instance_key}/groups/{group_id}/participants/remove | Remove participant.
[**instances_instance_key_groups_group_id_profile_pic_put**](GroupManagementApi.md#instances_instance_key_groups_group_id_profile_pic_put) | **PUT** /instances/{instance_key}/groups/{group_id}/profile-pic | Set group picture.
[**instances_instance_key_groups_invite_info_get**](GroupManagementApi.md#instances_instance_key_groups_invite_info_get) | **GET** /instances/{instance_key}/groups/invite-info | Get group from invite link.



## instances_instance_key_groups_admin_get

> crate::models::ApiResponse instances_instance_key_groups_admin_get(instance_key)
Get admin groupss.

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


## instances_instance_key_groups_create_post

> crate::models::ApiResponse instances_instance_key_groups_create_post(instance_key, data)
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


## instances_instance_key_groups_get

> crate::models::ApiResponse instances_instance_key_groups_get(instance_key, include_participants)
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


## instances_instance_key_groups_group_id_announce_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_announce_put(instance_key, announce, group_id)
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


## instances_instance_key_groups_group_id_delete

> crate::models::ApiResponse instances_instance_key_groups_group_id_delete(instance_key, group_id)
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


## instances_instance_key_groups_group_id_description_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_description_put(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_get

> crate::models::ApiResponse instances_instance_key_groups_group_id_get(instance_key, group_id)
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


## instances_instance_key_groups_group_id_invite_code_get

> crate::models::ApiResponse instances_instance_key_groups_group_id_invite_code_get(instance_key, group_id)
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


## instances_instance_key_groups_group_id_lock_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_lock_put(instance_key, locked, group_id)
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


## instances_instance_key_groups_group_id_name_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_name_put(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_participants_add_post

> crate::models::ApiResponse instances_instance_key_groups_group_id_participants_add_post(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_participants_demote_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_participants_demote_put(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_participants_promote_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_participants_promote_put(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_participants_remove_delete

> crate::models::ApiResponse instances_instance_key_groups_group_id_participants_remove_delete(instance_key, group_id, data)
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


## instances_instance_key_groups_group_id_profile_pic_put

> crate::models::ApiResponse instances_instance_key_groups_group_id_profile_pic_put(instance_key, group_id, instances_instance_key_groups_group_id_profile_pic_put_request)
Set group picture.

Changes the group profile picture. Currently it only seems to accept JPEG images only

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**group_id** | **String** | Group id of the group | [required] |
**instances_instance_key_groups_group_id_profile_pic_put_request** | [**InstancesInstanceKeyGroupsGroupIdProfilePicPutRequest**](InstancesInstanceKeyGroupsGroupIdProfilePicPutRequest.md) |  | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_groups_invite_info_get

> crate::models::ApiResponse instances_instance_key_groups_invite_info_get(instance_key, invite_link)
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

