# PeopleApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createPerson**](#createperson) | **POST** /people | |
|[**deletePerson**](#deleteperson) | **DELETE** /people/{person_id} | |
|[**getAllPeople**](#getallpeople) | **GET** /people | |
|[**getPersonById**](#getpersonbyid) | **GET** /people/{person_id} | |
|[**getPersonByMovieId**](#getpersonbymovieid) | **GET** /people/movie/{movie_id} | |
|[**getPersonByName**](#getpersonbyname) | **GET** /search/people/name/{name} | |
|[**updatePerson**](#updateperson) | **PUT** /people/{person_id} | |

# **createPerson**
> WrapperPerson createPerson(wrapperNewPerson)


### Example

```typescript
import {
    PeopleApi,
    Configuration,
    WrapperNewPerson
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let wrapperNewPerson: WrapperNewPerson; //

const { status, data } = await apiInstance.createPerson(
    wrapperNewPerson
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewPerson** | **WrapperNewPerson**|  | |


### Return type

**WrapperPerson**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new person |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deletePerson**
> number deletePerson()


### Example

```typescript
import {
    PeopleApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let personId: string; //ID of the person to delete (default to undefined)

const { status, data } = await apiInstance.deletePerson(
    personId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **personId** | [**string**] | ID of the person to delete | defaults to undefined|


### Return type

**number**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Delete person by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllPeople**
> Array<WrapperPerson> getAllPeople()


### Example

```typescript
import {
    PeopleApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

const { status, data } = await apiInstance.getAllPeople();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<WrapperPerson>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List all people |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPersonById**
> WrapperPerson getPersonById()


### Example

```typescript
import {
    PeopleApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let personId: string; //ID of the person to retrieve (default to undefined)

const { status, data } = await apiInstance.getPersonById(
    personId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **personId** | [**string**] | ID of the person to retrieve | defaults to undefined|


### Return type

**WrapperPerson**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get person by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPersonByMovieId**
> Array<WrapperPerson> getPersonByMovieId()


### Example

```typescript
import {
    PeopleApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let movieId: string; //ID of the movie to get people for (default to undefined)

const { status, data } = await apiInstance.getPersonByMovieId(
    movieId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **movieId** | [**string**] | ID of the movie to get people for | defaults to undefined|


### Return type

**Array<WrapperPerson>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get people by movie ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPersonByName**
> Array<WrapperPerson> getPersonByName()


### Example

```typescript
import {
    PeopleApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let name: string; //Name of the person to search for (default to undefined)
let limit: number; //Max number of people to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.getPersonByName(
    name,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] | Name of the person to search for | defaults to undefined|
| **limit** | [**number**] | Max number of people to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperPerson>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get persons by name |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updatePerson**
> WrapperPerson updatePerson(wrapperNewPerson)


### Example

```typescript
import {
    PeopleApi,
    Configuration,
    WrapperNewPerson
} from './api';

const configuration = new Configuration();
const apiInstance = new PeopleApi(configuration);

let personId: string; //ID of the person to update (default to undefined)
let wrapperNewPerson: WrapperNewPerson; //

const { status, data } = await apiInstance.updatePerson(
    personId,
    wrapperNewPerson
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewPerson** | **WrapperNewPerson**|  | |
| **personId** | [**string**] | ID of the person to update | defaults to undefined|


### Return type

**WrapperPerson**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update person by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

