# \WatchRoomMessagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watch_room_message**](WatchRoomMessagesApi.md#create_watch_room_message) | **POST** /watch-room-messages | 
[**delete_messages_by_room**](WatchRoomMessagesApi.md#delete_messages_by_room) | **DELETE** /watch-rooms/{room_id}/messages | 
[**delete_watch_room_message**](WatchRoomMessagesApi.md#delete_watch_room_message) | **DELETE** /watch-room-messages/{message_id} | 
[**get_watch_room_message_by_id**](WatchRoomMessagesApi.md#get_watch_room_message_by_id) | **GET** /watch-room-messages/{message_id} | 
[**list_messages_by_room**](WatchRoomMessagesApi.md#list_messages_by_room) | **GET** /watch-rooms/{room_id}/messages | 



## create_watch_room_message

> models::WatchRoomMessage create_watch_room_message(new_watch_room_message)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_watch_room_message** | [**NewWatchRoomMessage**](NewWatchRoomMessage.md) |  | [required] |

### Return type

[**models::WatchRoomMessage**](WatchRoomMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_messages_by_room

> i32 delete_messages_by_room(room_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room whose messages to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watch_room_message

> i32 delete_watch_room_message(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i64** | ID of the message to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_watch_room_message_by_id

> models::WatchRoomMessage get_watch_room_message_by_id(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i64** | ID of the message to retrieve | [required] |

### Return type

[**models::WatchRoomMessage**](WatchRoomMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_messages_by_room

> Vec<models::WatchRoomMessage> list_messages_by_room(room_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room whose messages to retrieve | [required] |
**limit** | **i64** | Max number of messages to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::WatchRoomMessage>**](WatchRoomMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

