# \GenresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_genre**](GenresApi.md#create_genre) | **POST** /genres | 
[**delete_genre**](GenresApi.md#delete_genre) | **DELETE** /genres/{genre_id} | 
[**get_genre_by_id**](GenresApi.md#get_genre_by_id) | **GET** /genres/{genre_id} | 
[**list_genres**](GenresApi.md#list_genres) | **GET** /genres | 
[**update_genre**](GenresApi.md#update_genre) | **PUT** /genres/{genre_id} | 



## create_genre

> models::Genre create_genre(new_genre)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_genre** | [**NewGenre**](NewGenre.md) |  | [required] |

### Return type

[**models::Genre**](Genre.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_genre

> i32 delete_genre(genre_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **uuid::Uuid** | ID of the genre to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genre_by_id

> models::Genre get_genre_by_id(genre_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **uuid::Uuid** | ID of the genre to retrieve | [required] |

### Return type

[**models::Genre**](Genre.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_genres

> Vec<models::Genre> list_genres(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i64** | Max number of genres to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::Genre>**](Genre.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_genre

> models::Genre update_genre(genre_id, new_genre)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_id** | **uuid::Uuid** | ID of the genre to update | [required] |
**new_genre** | [**NewGenre**](NewGenre.md) |  | [required] |

### Return type

[**models::Genre**](Genre.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

