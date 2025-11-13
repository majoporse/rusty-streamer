# \WatchRoomParticipantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watch_room_participant**](WatchRoomParticipantsApi.md#create_watch_room_participant) | **POST** /watch-room-participants | 
[**delete_participants_by_room**](WatchRoomParticipantsApi.md#delete_participants_by_room) | **DELETE** /watch-rooms/{room_id}/participants | 
[**delete_watch_room_participant**](WatchRoomParticipantsApi.md#delete_watch_room_participant) | **DELETE** /watch-rooms/{room_id}/participants/{user_id} | 
[**get_watch_room_participant_by_id**](WatchRoomParticipantsApi.md#get_watch_room_participant_by_id) | **GET** /watch-room-participants/{participant_id} | 
[**list_participants_by_room**](WatchRoomParticipantsApi.md#list_participants_by_room) | **GET** /watch-rooms/{room_id}/participants | 



## create_watch_room_participant

> models::WatchRoomParticipant create_watch_room_participant(new_watch_room_participant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_watch_room_participant** | [**NewWatchRoomParticipant**](NewWatchRoomParticipant.md) |  | [required] |

### Return type

[**models::WatchRoomParticipant**](WatchRoomParticipant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_participants_by_room

> i32 delete_participants_by_room(room_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room whose participants to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watch_room_participant

> i32 delete_watch_room_participant(room_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room | [required] |
**user_id** | **String** | UUID of the participant to remove | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_watch_room_participant_by_id

> models::WatchRoomParticipant get_watch_room_participant_by_id(participant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **i64** | ID of the participant to retrieve | [required] |

### Return type

[**models::WatchRoomParticipant**](WatchRoomParticipant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_participants_by_room

> Vec<models::WatchRoomParticipant> list_participants_by_room(room_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room to list participants for | [required] |
**limit** | **i64** | Max number of participants to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::WatchRoomParticipant>**](WatchRoomParticipant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

