# Rust API client for WhatsAPI

The V2 of WhatsAPI Go


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0
- Package version: 2.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `WhatsAPI` and add the following to `Cargo.toml` under `[dependencies]`:

```
WhatsAPI = { path = "./WhatsAPI" }
```

## Documentation for API Endpoints

All URIs are relative to */api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BusinessManagementApi* | [**fetch_catlog**](docs/BusinessManagementApi.md#fetch_catlog) | **GET** /instances/{instance_key}/business/catalog | Fetches the catlog.
*BusinessManagementApi* | [**send_payment_request**](docs/BusinessManagementApi.md#send_payment_request) | **POST** /instances/{instance_key}/business/payment-request | Send a payment request.
*GroupManagementApi* | [**add_participant**](docs/GroupManagementApi.md#add_participant) | **POST** /instances/{instance_key}/groups/{group_id}/participants/add | Add participant.
*GroupManagementApi* | [**create_group**](docs/GroupManagementApi.md#create_group) | **POST** /instances/{instance_key}/groups/create | Create group.
*GroupManagementApi* | [**demote_participant**](docs/GroupManagementApi.md#demote_participant) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/demote | Demote participant.
*GroupManagementApi* | [**get_admin_groups**](docs/GroupManagementApi.md#get_admin_groups) | **GET** /instances/{instance_key}/groups/admin | Get admin groups.
*GroupManagementApi* | [**get_all_groups**](docs/GroupManagementApi.md#get_all_groups) | **GET** /instances/{instance_key}/groups/ | Get all groups.
*GroupManagementApi* | [**get_all_participants**](docs/GroupManagementApi.md#get_all_participants) | **GET** /instances/{instance_key}/groups/{group_id}/participants | Get all participants.
*GroupManagementApi* | [**get_group**](docs/GroupManagementApi.md#get_group) | **GET** /instances/{instance_key}/groups/{group_id} | Get group.
*GroupManagementApi* | [**get_group_from_invite_link**](docs/GroupManagementApi.md#get_group_from_invite_link) | **GET** /instances/{instance_key}/groups/invite-info | Get group from invite link.
*GroupManagementApi* | [**get_group_invite_code**](docs/GroupManagementApi.md#get_group_invite_code) | **GET** /instances/{instance_key}/groups/{group_id}/invite-code | Get group invite code.
*GroupManagementApi* | [**join_group_with_link**](docs/GroupManagementApi.md#join_group_with_link) | **GET** /instances/{instance_key}/groups/join | Join group with invite code.
*GroupManagementApi* | [**leave_group**](docs/GroupManagementApi.md#leave_group) | **DELETE** /instances/{instance_key}/groups/{group_id}/ | Leaves the group.
*GroupManagementApi* | [**promote_participant**](docs/GroupManagementApi.md#promote_participant) | **PUT** /instances/{instance_key}/groups/{group_id}/participants/promote | Promote participant.
*GroupManagementApi* | [**remove_participant**](docs/GroupManagementApi.md#remove_participant) | **DELETE** /instances/{instance_key}/groups/{group_id}/participants/remove | Remove participant.
*GroupManagementApi* | [**set_group_announce**](docs/GroupManagementApi.md#set_group_announce) | **PUT** /instances/{instance_key}/groups/{group_id}/announce | Set group announce.
*GroupManagementApi* | [**set_group_description**](docs/GroupManagementApi.md#set_group_description) | **PUT** /instances/{instance_key}/groups/{group_id}/description | Set group description.
*GroupManagementApi* | [**set_group_locked**](docs/GroupManagementApi.md#set_group_locked) | **PUT** /instances/{instance_key}/groups/{group_id}/lock | Set group locked.
*GroupManagementApi* | [**set_group_name**](docs/GroupManagementApi.md#set_group_name) | **PUT** /instances/{instance_key}/groups/{group_id}/name | Set group name.
*GroupManagementApi* | [**set_group_picture**](docs/GroupManagementApi.md#set_group_picture) | **PUT** /instances/{instance_key}/groups/{group_id}/profile-pic | Set group picture.
*InstanceApi* | [**change_webhook_url**](docs/InstanceApi.md#change_webhook_url) | **PUT** /instances/{instance_key}/webhook | Change Webhook url.
*InstanceApi* | [**create_instance**](docs/InstanceApi.md#create_instance) | **GET** /instances/create | Creates a new instance key.
*InstanceApi* | [**delete_instance**](docs/InstanceApi.md#delete_instance) | **DELETE** /instances/{instance_key}/delete | Delete Instance.
*InstanceApi* | [**get_contacts**](docs/InstanceApi.md#get_contacts) | **GET** /instances/{instance_key}/contacts | Get contacts.
*InstanceApi* | [**get_instance**](docs/InstanceApi.md#get_instance) | **GET** /instances/{instance_key}/ | Get Instance.
*InstanceApi* | [**get_qr_code**](docs/InstanceApi.md#get_qr_code) | **GET** /instances/{instance_key}/qrcode | Get QrCode.
*InstanceApi* | [**list_instances**](docs/InstanceApi.md#list_instances) | **GET** /instances/list | Get all instances.
*InstanceApi* | [**logout_instance**](docs/InstanceApi.md#logout_instance) | **DELETE** /instances/{instance_key}/logout | Logout Instance.
*MessageSendingApi* | [**send_audio**](docs/MessageSendingApi.md#send_audio) | **POST** /instances/{instance_key}/send/audio | Send raw audio.
*MessageSendingApi* | [**send_button_message**](docs/MessageSendingApi.md#send_button_message) | **POST** /instances/{instance_key}/send/buttons | Send a button message.
*MessageSendingApi* | [**send_button_with_media**](docs/MessageSendingApi.md#send_button_with_media) | **POST** /instances/{instance_key}/send/button-media | Send a button message with a media header.
*MessageSendingApi* | [**send_contact**](docs/MessageSendingApi.md#send_contact) | **POST** /instances/{instance_key}/send/contact | Send a contact message.
*MessageSendingApi* | [**send_document**](docs/MessageSendingApi.md#send_document) | **POST** /instances/{instance_key}/send/document | Send raw document.
*MessageSendingApi* | [**send_group_invite**](docs/MessageSendingApi.md#send_group_invite) | **POST** /instances/{instance_key}/send/group-invite | Send a group invite message
*MessageSendingApi* | [**send_image**](docs/MessageSendingApi.md#send_image) | **POST** /instances/{instance_key}/send/image | Send raw image.
*MessageSendingApi* | [**send_list_message**](docs/MessageSendingApi.md#send_list_message) | **POST** /instances/{instance_key}/send/list | Send a List message.
*MessageSendingApi* | [**send_location**](docs/MessageSendingApi.md#send_location) | **POST** /instances/{instance_key}/send/location | Send a location message.
*MessageSendingApi* | [**send_media_message**](docs/MessageSendingApi.md#send_media_message) | **POST** /instances/{instance_key}/send/media | Send a media message.
*MessageSendingApi* | [**send_poll_message**](docs/MessageSendingApi.md#send_poll_message) | **POST** /instances/{instance_key}/send/poll | Send a Poll message.
*MessageSendingApi* | [**send_template**](docs/MessageSendingApi.md#send_template) | **POST** /instances/{instance_key}/send/template | Send a template message.
*MessageSendingApi* | [**send_template_with_media**](docs/MessageSendingApi.md#send_template_with_media) | **POST** /instances/{instance_key}/send/template-media | Send a template message with media.
*MessageSendingApi* | [**send_text_message**](docs/MessageSendingApi.md#send_text_message) | **POST** /instances/{instance_key}/send/text | Send a text message.
*MessageSendingApi* | [**send_video**](docs/MessageSendingApi.md#send_video) | **POST** /instances/{instance_key}/send/video | Send raw video.
*MessageSendingApi* | [**upload_media**](docs/MessageSendingApi.md#upload_media) | **POST** /instances/{instance_key}/send/upload | Upload media.
*MiscellaneousApi* | [**download_media**](docs/MiscellaneousApi.md#download_media) | **POST** /instances/{instance_key}/misc/download | Download media
*MiscellaneousApi* | [**get_profile_pic**](docs/MiscellaneousApi.md#get_profile_pic) | **GET** /instances/{instance_key}/misc/profile-pic | Get profile pic.
*MiscellaneousApi* | [**get_users_info**](docs/MiscellaneousApi.md#get_users_info) | **POST** /instances/{instance_key}/misc/user-info | Fetches the users info.
*MiscellaneousApi* | [**set_chat_presence**](docs/MiscellaneousApi.md#set_chat_presence) | **POST** /instances/{instance_key}/misc/chat-presence | Set chat presence
*MiscellaneousApi* | [**update_profile_pic**](docs/MiscellaneousApi.md#update_profile_pic) | **PUT** /instances/{instance_key}/misc/profile-pic | Update profile picture


## Documentation For Models

 - [ApiResponse](docs/ApiResponse.md)
 - [ButtonMessagePayload](docs/ButtonMessagePayload.md)
 - [ButtonMessageWithMediaPayload](docs/ButtonMessageWithMediaPayload.md)
 - [ContactMessagePayload](docs/ContactMessagePayload.md)
 - [ContactMessagePayloadVcard](docs/ContactMessagePayloadVcard.md)
 - [FileUpload](docs/FileUpload.md)
 - [GroupCreatePayload](docs/GroupCreatePayload.md)
 - [GroupInviteMessagePayload](docs/GroupInviteMessagePayload.md)
 - [GroupUpdateDescriptionPayload](docs/GroupUpdateDescriptionPayload.md)
 - [GroupUpdateNamePayload](docs/GroupUpdateNamePayload.md)
 - [GroupUpdateParticipantsPayload](docs/GroupUpdateParticipantsPayload.md)
 - [ListItem](docs/ListItem.md)
 - [ListMessagePayload](docs/ListMessagePayload.md)
 - [ListSection](docs/ListSection.md)
 - [LocationMessagePayload](docs/LocationMessagePayload.md)
 - [LocationMessagePayloadLocation](docs/LocationMessagePayloadLocation.md)
 - [PaymentRequestPayload](docs/PaymentRequestPayload.md)
 - [PollMessagePayload](docs/PollMessagePayload.md)
 - [ReplyButton](docs/ReplyButton.md)
 - [SendAudioRequest](docs/SendAudioRequest.md)
 - [SendDocumentRequest](docs/SendDocumentRequest.md)
 - [SendMediaPayload](docs/SendMediaPayload.md)
 - [SendVideoRequest](docs/SendVideoRequest.md)
 - [SetGroupPictureRequest](docs/SetGroupPictureRequest.md)
 - [TemplateButton](docs/TemplateButton.md)
 - [TemplateButtonPayload](docs/TemplateButtonPayload.md)
 - [TemplateButtonWithMediaPayload](docs/TemplateButtonWithMediaPayload.md)
 - [TextMessage](docs/TextMessage.md)
 - [UpdateProfilePicRequest](docs/UpdateProfilePicRequest.md)
 - [UploadMediaRequest](docs/UploadMediaRequest.md)
 - [UserInfoPayload](docs/UserInfoPayload.md)
 - [WebhookPayload](docs/WebhookPayload.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

manjit@sponsorbook.io

