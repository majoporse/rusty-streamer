# \PeopleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_person**](PeopleApi.md#create_person) | **POST** /people | 
[**delete_person**](PeopleApi.md#delete_person) | **DELETE** /people/{person_id} | 
[**get_all_people**](PeopleApi.md#get_all_people) | **GET** /people | 
[**get_person_by_id**](PeopleApi.md#get_person_by_id) | **GET** /people/{person_id} | 
[**get_person_by_movie_id**](PeopleApi.md#get_person_by_movie_id) | **GET** /search/people/movie/{movie_id} | 
[**get_person_by_name**](PeopleApi.md#get_person_by_name) | **GET** /search/people/name/{name} | 
[**update_person**](PeopleApi.md#update_person) | **PUT** /people/{person_id} | 



## create_person

> models::Person create_person(new_person)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_person** | [**NewPerson**](NewPerson.md) |  | [required] |

### Return type

[**models::Person**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_person

> i32 delete_person(person_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **uuid::Uuid** | ID of the person to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_people

> Vec<models::Person> get_all_people(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i64** | Max number of people to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::Person>**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_by_id

> models::Person get_person_by_id(person_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **uuid::Uuid** | ID of the person to retrieve | [required] |

### Return type

[**models::Person**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_by_movie_id

> Vec<models::Person> get_person_by_movie_id(movie_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | **uuid::Uuid** | ID of the movie to get persons for | [required] |

### Return type

[**Vec<models::Person>**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_by_name

> Vec<models::Person> get_person_by_name(name, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the person to search for | [required] |
**limit** | **i64** | Max number of people to return | [required] |
**offset** | **i64** | Pagination offset | [required] |

### Return type

[**Vec<models::Person>**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_person

> models::Person update_person(person_id, new_person)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_id** | **uuid::Uuid** | ID of the person to update | [required] |
**new_person** | [**NewPerson**](NewPerson.md) |  | [required] |

### Return type

[**models::Person**](Person.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

