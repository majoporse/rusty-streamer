# \ReviewsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_review**](ReviewsApi.md#create_review) | **POST** /reviews | 
[**delete_review**](ReviewsApi.md#delete_review) | **DELETE** /reviews/{review_id} | 
[**get_all_reviews**](ReviewsApi.md#get_all_reviews) | **GET** /reviews | 
[**get_review_by_id**](ReviewsApi.md#get_review_by_id) | **GET** /reviews/{review_id} | 
[**update_review**](ReviewsApi.md#update_review) | **PUT** /reviews/{review_id} | 



## create_review

> models::Review create_review(new_review)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_review** | [**NewReview**](NewReview.md) |  | [required] |

### Return type

[**models::Review**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_review

> i32 delete_review(review_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **i32** | ID of the review to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_reviews

> Vec<models::Review> get_all_reviews()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Review>**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_review_by_id

> models::Review get_review_by_id(review_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **i32** | ID of the review to retrieve | [required] |

### Return type

[**models::Review**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_review

> models::Review update_review(review_id, new_review)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **i32** | ID of the review to update | [required] |
**new_review** | [**NewReview**](NewReview.md) |  | [required] |

### Return type

[**models::Review**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

