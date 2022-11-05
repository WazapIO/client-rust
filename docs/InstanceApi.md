# WhatsAPI\InstanceApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_webhook_url**](InstanceApi.md#change_webhook_url) | **PUT** /instances/{instance_key}/webhook | Change Webhook url.
[**create_instance**](InstanceApi.md#create_instance) | **GET** /instances/create | Creates a new instance key.
[**delete_instance**](InstanceApi.md#delete_instance) | **DELETE** /instances/{instance_key}/delete | Delete Instance.
[**get_contacts**](InstanceApi.md#get_contacts) | **GET** /instances/{instance_key}/contacts | Get contacts.
[**get_instance**](InstanceApi.md#get_instance) | **GET** /instances/{instance_key}/ | Get Instance.
[**get_qr_code**](InstanceApi.md#get_qr_code) | **GET** /instances/{instance_key}/qrcode | Get QrCode.
[**list_instances**](InstanceApi.md#list_instances) | **GET** /instances/list | Get all instances.
[**logout_instance**](InstanceApi.md#logout_instance) | **DELETE** /instances/{instance_key}/logout | Logout Instance.



## change_webhook_url

> crate::models::ApiResponse change_webhook_url(instance_key, data)
Change Webhook url.

Changes the webhook url of an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**WebhookPayload**](WebhookPayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::ApiResponse create_instance(instance_key)
Creates a new instance key.

This endpoint is used to create a new WhatsApp Web instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | Option<**String**> | Insert instance key if you want to provide custom key |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> crate::models::ApiResponse delete_instance(instance_key)
Delete Instance.

Deletes the instance with the provided key.

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


## get_contacts

> crate::models::ApiResponse get_contacts(instance_key)
Get contacts.

Fetches the list of contacts in the instance.

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


## get_instance

> crate::models::ApiResponse get_instance(instance_key)
Get Instance.

Returns the instance data of single instance with connection status.

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


## get_qr_code

> crate::models::ApiResponse get_qr_code(instance_key)
Get QrCode.

Returns the qrcode in the base64 format.

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


## list_instances

> crate::models::ApiResponse list_instances()
Get all instances.

Fetches the list of all Instances with login status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_instance

> crate::models::ApiResponse logout_instance(instance_key)
Logout Instance.

Logouts of the instance with the provided key.

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

