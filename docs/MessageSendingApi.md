# \MessageSendingApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instances_instance_key_business_catalog_get**](MessageSendingApi.md#instances_instance_key_business_catalog_get) | **GET** /instances/{instance_key}/business/catalog | Fetches the catlog.
[**instances_instance_key_send_audio_post**](MessageSendingApi.md#instances_instance_key_send_audio_post) | **POST** /instances/{instance_key}/send/audio | Send raw audio.
[**instances_instance_key_send_button_media_post**](MessageSendingApi.md#instances_instance_key_send_button_media_post) | **POST** /instances/{instance_key}/send/button-media | Send a button message with a media header.
[**instances_instance_key_send_buttons_post**](MessageSendingApi.md#instances_instance_key_send_buttons_post) | **POST** /instances/{instance_key}/send/buttons | Send a button message.
[**instances_instance_key_send_contact_post**](MessageSendingApi.md#instances_instance_key_send_contact_post) | **POST** /instances/{instance_key}/send/contact | Send a contact message.
[**instances_instance_key_send_document_post**](MessageSendingApi.md#instances_instance_key_send_document_post) | **POST** /instances/{instance_key}/send/document | Send raw document.
[**instances_instance_key_send_image_post**](MessageSendingApi.md#instances_instance_key_send_image_post) | **POST** /instances/{instance_key}/send/image | Send raw image.
[**instances_instance_key_send_list_post**](MessageSendingApi.md#instances_instance_key_send_list_post) | **POST** /instances/{instance_key}/send/list | Send a List message.
[**instances_instance_key_send_location_post**](MessageSendingApi.md#instances_instance_key_send_location_post) | **POST** /instances/{instance_key}/send/location | Send a location message.
[**instances_instance_key_send_media_post**](MessageSendingApi.md#instances_instance_key_send_media_post) | **POST** /instances/{instance_key}/send/media | Send a media message.
[**instances_instance_key_send_poll_post**](MessageSendingApi.md#instances_instance_key_send_poll_post) | **POST** /instances/{instance_key}/send/poll | Send a Poll message with media.
[**instances_instance_key_send_template_media_post**](MessageSendingApi.md#instances_instance_key_send_template_media_post) | **POST** /instances/{instance_key}/send/template-media | Send a template message with media.
[**instances_instance_key_send_template_post**](MessageSendingApi.md#instances_instance_key_send_template_post) | **POST** /instances/{instance_key}/send/template | Send a template message.
[**instances_instance_key_send_text_post**](MessageSendingApi.md#instances_instance_key_send_text_post) | **POST** /instances/{instance_key}/send/text | Send a text message.
[**instances_instance_key_send_upload_post**](MessageSendingApi.md#instances_instance_key_send_upload_post) | **POST** /instances/{instance_key}/send/upload | Upload media.
[**instances_instance_key_send_video_post**](MessageSendingApi.md#instances_instance_key_send_video_post) | **POST** /instances/{instance_key}/send/video | Send raw video.



## instances_instance_key_business_catalog_get

> crate::models::MainPeriodApiResponse instances_instance_key_business_catalog_get(instance_key)
Fetches the catlog.

Gets list of all products registered by you.

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


## instances_instance_key_send_audio_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_audio_post(instance_key, to, instances_instance_key_send_audio_post_request, caption)
Send raw audio.

Sends a audio message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**instances_instance_key_send_audio_post_request** | [**InstancesInstanceKeySendAudioPostRequest**](InstancesInstanceKeySendAudioPostRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_button_media_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_button_media_post(instance_key, data)
Send a button message with a media header.

Sends an interactive button message to the given user. This message also has media header with it. Make sure that all the button ids are unique

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodButtonMessageWithMediaPayload**](StructsPeriodButtonMessageWithMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_buttons_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_buttons_post(instance_key, data)
Send a button message.

Sends an interactive button message to the given user. Make sure that all the button ids are unique

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodButtonMessagePayload**](StructsPeriodButtonMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_contact_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_contact_post(instance_key, data)
Send a contact message.

Sends a contact (vcard) message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodContactMessagePayload**](StructsPeriodContactMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_document_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_document_post(instance_key, to, instances_instance_key_send_document_post_request, caption)
Send raw document.

Sends a document message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**instances_instance_key_send_document_post_request** | [**InstancesInstanceKeySendDocumentPostRequest**](InstancesInstanceKeySendDocumentPostRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_image_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_image_post(instance_key, to, instances_instance_key_send_image_post_request, caption)
Send raw image.

Sends a image message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**instances_instance_key_send_image_post_request** | [**InstancesInstanceKeySendImagePostRequest**](InstancesInstanceKeySendImagePostRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_list_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_list_post(instance_key, data)
Send a List message.

Sends an interactive List message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodListMessagePayload**](StructsPeriodListMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_location_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_location_post(instance_key, data)
Send a location message.

Sends a location message to the given user. This is static location and does not update Note: The Address and Url fields are optional

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodLocationMessagePayload**](StructsPeriodLocationMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_media_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_media_post(instance_key, data)
Send a media message.

Sends a media message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodSendMediaPayload**](StructsPeriodSendMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_poll_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_poll_post(instance_key, data)
Send a Poll message with media.

Sends an interactive poll message with a media header to the given user. The poll message is a new feature that is currently in beta.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodPollMessagePayload**](StructsPeriodPollMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_template_media_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_template_media_post(instance_key, data)
Send a template message with media.

Sends an interactive template message with a media header to the given user. Note: The valid button types are \"replyButton\", \"urlButton\", \"callButton\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodTemplateButtonWithMediaPayload**](StructsPeriodTemplateButtonWithMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_template_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_template_post(instance_key, data)
Send a template message.

Sends an interactive template message to the given user. Note: The valid button types are \"replyButton\", \"urlButton\", \"callButton\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodTemplateButtonPayload**](StructsPeriodTemplateButtonPayload.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_text_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_text_post(instance_key, data)
Send a text message.

Sends a text message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**StructsPeriodTextMessage**](StructsPeriodTextMessage.md) | Message data | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_upload_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_upload_post(instance_key, r#type, instances_instance_key_send_upload_post_request)
Upload media.

Uploads media to WhatsApp servers and returns the media keys. Store the returned media keys, as you will need them to send media messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**r#type** | **String** | Media type | [required] |
**instances_instance_key_send_upload_post_request** | [**InstancesInstanceKeySendUploadPostRequest**](InstancesInstanceKeySendUploadPostRequest.md) |  | [required] |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instances_instance_key_send_video_post

> crate::models::MainPeriodApiResponse instances_instance_key_send_video_post(instance_key, to, instances_instance_key_send_video_post_request, caption)
Send raw video.

Sends a video message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**instances_instance_key_send_video_post_request** | [**InstancesInstanceKeySendVideoPostRequest**](InstancesInstanceKeySendVideoPostRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::MainPeriodApiResponse**](main.APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

