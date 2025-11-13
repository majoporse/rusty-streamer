# \WatchlistApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_watchlist_item**](WatchlistApi.md#create_watchlist_item) | **POST** /watchlist | 
[**delete_watchlist_item**](WatchlistApi.md#delete_watchlist_item) | **DELETE** /users/{user_id}/watchlist/{content_id} | 
[**get_watchlist_by_user**](WatchlistApi.md#get_watchlist_by_user) | **GET** /users/{user_id}/watchlist | 



## create_watchlist_item

> models::Watchlist create_watchlist_item(new_watchlist)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_watchlist** | [**NewWatchlist**](NewWatchlist.md) |  | [required] |

### Return type

[**models::Watchlist**](Watchlist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_watchlist_item

> i32 delete_watchlist_item(user_id, content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | UUID of the user | [required] |
**content_id** | **String** | UUID of the content item | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_watchlist_by_user

> Vec<models::Watchlist> get_watchlist_by_user(user_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | UUID of the user whose watchlist to retrieve | [required] |
**limit** | **i64** | Max number of items to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::Watchlist>**](Watchlist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

