# \MoviesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_movie**](MoviesApi.md#create_movie) | **POST** /movies | 
[**delete_movie**](MoviesApi.md#delete_movie) | **DELETE** /movies/{movie_id} | 
[**get_all_movies**](MoviesApi.md#get_all_movies) | **GET** /movies | 
[**get_movie_by_id**](MoviesApi.md#get_movie_by_id) | **GET** /movies/{movie_id} | 
[**update_movie**](MoviesApi.md#update_movie) | **PUT** /movies/{movie_id} | 



## create_movie

> models::Movie create_movie(new_movie)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_movie** | [**NewMovie**](NewMovie.md) |  | [required] |

### Return type

[**models::Movie**](Movie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_movie

> i32 delete_movie(movie_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** | ID of the movie to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_movies

> Vec<models::Movie> get_all_movies()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Movie>**](Movie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_by_id

> models::Movie get_movie_by_id(movie_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** | ID of the movie to retrieve | [required] |

### Return type

[**models::Movie**](Movie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_movie

> models::Movie update_movie(movie_id, new_movie)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **i32** | ID of the movie to update | [required] |
**new_movie** | [**NewMovie**](NewMovie.md) |  | [required] |

### Return type

[**models::Movie**](Movie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

