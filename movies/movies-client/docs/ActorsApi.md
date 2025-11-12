# \ActorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_actor**](ActorsApi.md#create_actor) | **POST** /actors | 
[**delete_actor**](ActorsApi.md#delete_actor) | **DELETE** /actors/{actor_id} | 
[**get_actor_by_id**](ActorsApi.md#get_actor_by_id) | **GET** /actors/{actor_id} | 
[**get_all_actors**](ActorsApi.md#get_all_actors) | **GET** /actors | 
[**update_actor**](ActorsApi.md#update_actor) | **PUT** /actors/{actor_id} | 



## create_actor

> models::Actor create_actor(new_actor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_actor** | [**NewActor**](NewActor.md) |  | [required] |

### Return type

[**models::Actor**](Actor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_actor

> i32 delete_actor(actor_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **i32** | ID of the actor to delete | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_actor_by_id

> models::Actor get_actor_by_id(actor_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **i32** | ID of the actor to retrieve | [required] |

### Return type

[**models::Actor**](Actor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_actors

> Vec<models::Actor> get_all_actors()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Actor>**](Actor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_actor

> models::Actor update_actor(actor_id, new_actor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **i32** | ID of the actor to update | [required] |
**new_actor** | [**NewActor**](NewActor.md) |  | [required] |

### Return type

[**models::Actor**](Actor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

