# WhatsAPI\MessageSendingApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_audio**](MessageSendingApi.md#send_audio) | **POST** /instances/{instance_key}/send/audio | Send raw audio.
[**send_button_message**](MessageSendingApi.md#send_button_message) | **POST** /instances/{instance_key}/send/buttons | Send a button message.
[**send_button_with_media**](MessageSendingApi.md#send_button_with_media) | **POST** /instances/{instance_key}/send/button-media | Send a button message with a media header.
[**send_contact**](MessageSendingApi.md#send_contact) | **POST** /instances/{instance_key}/send/contact | Send a contact message.
[**send_document**](MessageSendingApi.md#send_document) | **POST** /instances/{instance_key}/send/document | Send raw document.
[**send_group_invite**](MessageSendingApi.md#send_group_invite) | **POST** /instances/{instance_key}/send/group-invite | Send a group invite message
[**send_image**](MessageSendingApi.md#send_image) | **POST** /instances/{instance_key}/send/image | Send raw image.
[**send_list_message**](MessageSendingApi.md#send_list_message) | **POST** /instances/{instance_key}/send/list | Send a List message.
[**send_location**](MessageSendingApi.md#send_location) | **POST** /instances/{instance_key}/send/location | Send a location message.
[**send_media_message**](MessageSendingApi.md#send_media_message) | **POST** /instances/{instance_key}/send/media | Send a media message.
[**send_poll_message**](MessageSendingApi.md#send_poll_message) | **POST** /instances/{instance_key}/send/poll | Send a Poll message.
[**send_template**](MessageSendingApi.md#send_template) | **POST** /instances/{instance_key}/send/template | Send a template message.
[**send_template_with_media**](MessageSendingApi.md#send_template_with_media) | **POST** /instances/{instance_key}/send/template-media | Send a template message with media.
[**send_text_message**](MessageSendingApi.md#send_text_message) | **POST** /instances/{instance_key}/send/text | Send a text message.
[**send_video**](MessageSendingApi.md#send_video) | **POST** /instances/{instance_key}/send/video | Send raw video.
[**upload_media**](MessageSendingApi.md#upload_media) | **POST** /instances/{instance_key}/send/upload | Upload media.
[**upload_media_from_url**](MessageSendingApi.md#upload_media_from_url) | **POST** /instances/{instance_key}/send/upload-url | Upload media from url.



## send_audio

> crate::models::ApiResponse send_audio(instance_key, to, send_audio_request, caption)
Send raw audio.

Sends a audio message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**send_audio_request** | [**SendAudioRequest**](SendAudioRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_button_message

> crate::models::ApiResponse send_button_message(instance_key, data)
Send a button message.

Sends an interactive button message to the given user. Make sure that all the button ids are unique

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**ButtonMessagePayload**](ButtonMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_button_with_media

> crate::models::ApiResponse send_button_with_media(instance_key, data)
Send a button message with a media header.

Sends an interactive button message to the given user. This message also has media header with it. Make sure that all the button ids are unique

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**ButtonMessageWithMediaPayload**](ButtonMessageWithMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_contact

> crate::models::ApiResponse send_contact(instance_key, data)
Send a contact message.

Sends a contact (vcard) message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**ContactMessagePayload**](ContactMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_document

> crate::models::ApiResponse send_document(instance_key, to, send_document_request, caption)
Send raw document.

Sends a document message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**send_document_request** | [**SendDocumentRequest**](SendDocumentRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_group_invite

> crate::models::ApiResponse send_group_invite(instance_key, data)
Send a group invite message

Sends a group invite message to the specified number. Don't include \"https://chat.whatsapp.com/\" in the invite code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**GroupInviteMessagePayload**](GroupInviteMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_image

> crate::models::ApiResponse send_image(instance_key, to, update_profile_pic_request, caption)
Send raw image.

Sends a image message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**update_profile_pic_request** | [**UpdateProfilePicRequest**](UpdateProfilePicRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_list_message

> crate::models::ApiResponse send_list_message(instance_key, data)
Send a List message.

Sends an interactive List message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**ListMessagePayload**](ListMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_location

> crate::models::ApiResponse send_location(instance_key, data)
Send a location message.

Sends a location message to the given user. This is static location and does not update Note: The Address and Url fields are optional

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**LocationMessagePayload**](LocationMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_media_message

> crate::models::ApiResponse send_media_message(instance_key, data)
Send a media message.

Sends a media message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**SendMediaPayload**](SendMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_poll_message

> crate::models::ApiResponse send_poll_message(instance_key, data)
Send a Poll message.

Sends an interactive poll message to the given user. The poll message is a new feature that is currently in beta.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**PollMessagePayload**](PollMessagePayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_template

> crate::models::ApiResponse send_template(instance_key, data)
Send a template message.

Sends an interactive template message to the given user. Note: The valid button types are \"replyButton\", \"urlButton\", \"callButton\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**TemplateButtonPayload**](TemplateButtonPayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_template_with_media

> crate::models::ApiResponse send_template_with_media(instance_key, data)
Send a template message with media.

Sends an interactive template message with a media header to the given user. Note: The valid button types are \"replyButton\", \"urlButton\", \"callButton\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**TemplateButtonWithMediaPayload**](TemplateButtonWithMediaPayload.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_text_message

> crate::models::ApiResponse send_text_message(instance_key, data)
Send a text message.

Sends a text message to the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**data** | [**TextMessage**](TextMessage.md) | Message data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_video

> crate::models::ApiResponse send_video(instance_key, to, send_video_request, caption)
Send raw video.

Sends a video message by uploading to the WhatsApp servers every time. This is not recommended for bulk sending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**to** | **String** | The recipient's number | [required] |
**send_video_request** | [**SendVideoRequest**](SendVideoRequest.md) |  | [required] |
**caption** | Option<**String**> | Attached caption |  |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_media

> crate::models::ApiResponse upload_media(instance_key, r#type, upload_media_request)
Upload media.

Uploads media to WhatsApp servers and returns the media keys. Store the returned media keys, as you will need them to send media messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**r#type** | **String** | Media type | [required] |
**upload_media_request** | [**UploadMediaRequest**](UploadMediaRequest.md) |  | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_media_from_url

> crate::models::ApiResponse upload_media_from_url(instance_key, r#type, data)
Upload media from url.

Uploads media from a url to WhatsApp servers and returns the media keys. Store the returned media keys, as you will need them to send media messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_key** | **String** | Instance key | [required] |
**r#type** | **String** | Media type | [required] |
**data** | [**UrlMediaUploadPayload**](UrlMediaUploadPayload.md) | Media data | [required] |

### Return type

[**crate::models::ApiResponse**](APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

