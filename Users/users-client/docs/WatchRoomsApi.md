# \WatchRoomsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watch_room**](WatchRoomsApi.md#create_watch_room) | **POST** /watch-rooms | 
[**delete_watch_room**](WatchRoomsApi.md#delete_watch_room) | **DELETE** /watch-rooms/{room_id} | 
[**get_watch_room_by_id**](WatchRoomsApi.md#get_watch_room_by_id) | **GET** /watch-rooms/{room_id} | 
[**list_rooms_by_host**](WatchRoomsApi.md#list_rooms_by_host) | **GET** /users/{host_user_id}/watch-rooms | 
[**update_watch_room**](WatchRoomsApi.md#update_watch_room) | **PUT** /watch-rooms/{room_id} | 



## create_watch_room

> models::WatchRoom create_watch_room(new_watch_room)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_watch_room** | [**NewWatchRoom**](NewWatchRoom.md) |  | [required] |

### Return type

[**models::WatchRoom**](WatchRoom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watch_room

> i32 delete_watch_room(room_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_watch_room_by_id

> models::WatchRoom get_watch_room_by_id(room_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the watch room to retrieve | [required] |

### Return type

[**models::WatchRoom**](WatchRoom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rooms_by_host

> Vec<models::WatchRoom> list_rooms_by_host(host_user_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_user_id** | **String** | UUID of the user hosting the rooms | [required] |
**limit** | **i64** | Max number of rooms to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::WatchRoom>**](WatchRoom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_watch_room

> models::WatchRoom update_watch_room(room_id, update_watch_room)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | UUID of the room to update | [required] |
**update_watch_room** | [**UpdateWatchRoom**](UpdateWatchRoom.md) |  | [required] |

### Return type

[**models::WatchRoom**](WatchRoom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

