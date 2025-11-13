# \WatchHistoryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watch_history**](WatchHistoryApi.md#create_watch_history) | **POST** /watch-history | 
[**delete_watch_history**](WatchHistoryApi.md#delete_watch_history) | **DELETE** /watch-history/{id} | 
[**get_watch_history_by_id**](WatchHistoryApi.md#get_watch_history_by_id) | **GET** /watch-history/{id} | 
[**list_watch_history_by_user**](WatchHistoryApi.md#list_watch_history_by_user) | **GET** /users/{user_id}/watch-history | 
[**update_watch_history**](WatchHistoryApi.md#update_watch_history) | **PUT** /watch-history/{id} | 



## create_watch_history

> models::WatchHistory create_watch_history(new_watch_history)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_watch_history** | [**NewWatchHistory**](NewWatchHistory.md) |  | [required] |

### Return type

[**models::WatchHistory**](WatchHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watch_history

> i32 delete_watch_history(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of the watch history entry to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_watch_history_by_id

> models::WatchHistory get_watch_history_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of the watch history entry | [required] |

### Return type

[**models::WatchHistory**](WatchHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_watch_history_by_user

> Vec<models::WatchHistory> list_watch_history_by_user(user_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | UUID of the user whose history to retrieve | [required] |
**limit** | **i64** | Max number of items to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::WatchHistory>**](WatchHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_watch_history

> models::WatchHistory update_watch_history(id, update_watch_history)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of the watch history entry to update | [required] |
**update_watch_history** | [**UpdateWatchHistory**](UpdateWatchHistory.md) |  | [required] |

### Return type

[**models::WatchHistory**](WatchHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

