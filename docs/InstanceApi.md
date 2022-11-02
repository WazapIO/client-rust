# \InstanceApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instances_create_get**](InstanceApi.md#instances_create_get) | **GET** /instances/create | Creates a new instance key.
[**instances_instance_key_contacts_get**](InstanceApi.md#instances_instance_key_contacts_get) | **GET** /instances/{instance_key}/contacts | Get contacts.
[**instances_instance_key_delete_delete**](InstanceApi.md#instances_instance_key_delete_delete) | **DELETE** /instances/{instance_key}/delete | Delete Instance.
[**instances_instance_key_get**](InstanceApi.md#instances_instance_key_get) | **GET** /instances/{instance_key}/ | Get Instance.
[**instances_instance_key_logout_delete**](InstanceApi.md#instances_instance_key_logout_delete) | **DELETE** /instances/{instance_key}/logout | Logout Instance.
[**instances_instance_key_qrcode_get**](InstanceApi.md#instances_instance_key_qrcode_get) | **GET** /instances/{instance_key}/qrcode | Get QrCode.
[**instances_instance_key_webhook_put**](InstanceApi.md#instances_instance_key_webhook_put) | **PUT** /instances/{instance_key}/webhook | Change Webhook url.
[**instances_list_get**](InstanceApi.md#instances_list_get) | **GET** /instances/list | Get all instances.



## instances_create_get

> crate::models::MainPeriodApiResponse instances_create_get(instance_key)
Creates a new instance key.

This endpoint is used to create a new WhatsApp Web instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | Option<**String**> | Insert instance key if you want to provide custom key |  |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_contacts_get

> crate::models::MainPeriodApiResponse instances_instance_key_contacts_get(instance_key)
Get contacts.

Fetches the list of contacts in the instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_delete_delete

> crate::models::MainPeriodApiResponse instances_instance_key_delete_delete(instance_key)
Delete Instance.

Deletes the instance with the provided key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_get

> crate::models::MainPeriodApiResponse instances_instance_key_get(instance_key)
Get Instance.

Returns the instance data of single instance with connection status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_logout_delete

> crate::models::MainPeriodApiResponse instances_instance_key_logout_delete(instance_key)
Logout Instance.

Logouts of the instance with the provided key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_qrcode_get

> crate::models::MainPeriodApiResponse instances_instance_key_qrcode_get(instance_key)
Get QrCode.

Returns the qrcode in the base64 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_webhook_put

> crate::models::MainPeriodApiResponse instances_instance_key_webhook_put(instance_key, data)
Change Webhook url.

Changes the webhook url of an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodWebhookPayload**](StructsPeriodWebhookPayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_list_get

> crate::models::MainPeriodApiResponse instances_list_get()
Get all instances.

Fetches the list of all Instances with login status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

