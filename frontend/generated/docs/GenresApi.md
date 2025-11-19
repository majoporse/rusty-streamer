# GenresApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createGenre**](#creategenre) | **POST** /genres | |
|[**deleteGenre**](#deletegenre) | **DELETE** /genres/{genre_id} | |
|[**getGenreById**](#getgenrebyid) | **GET** /genres/{genre_id} | |
|[**listGenres**](#listgenres) | **GET** /genres | |
|[**updateGenre**](#updategenre) | **PUT** /genres/{genre_id} | |

# **createGenre**
> WrapperGenre createGenre(wrapperNewGenre)


### Example

```typescript
import {
    GenresApi,
    Configuration,
    WrapperNewGenre
} from './api';

const configuration = new Configuration();
const apiInstance = new GenresApi(configuration);

let wrapperNewGenre: WrapperNewGenre; //

const { status, data } = await apiInstance.createGenre(
    wrapperNewGenre
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewGenre** | **WrapperNewGenre**|  | |


### Return type

**WrapperGenre**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new genre |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteGenre**
> number deleteGenre()


### Example

```typescript
import {
    GenresApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new GenresApi(configuration);

let genreId: string; //ID of the genre to delete (default to undefined)

const { status, data } = await apiInstance.deleteGenre(
    genreId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **genreId** | [**string**] | ID of the genre to delete | defaults to undefined|


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
|**200** | Delete genre by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGenreById**
> WrapperGenre getGenreById()


### Example

```typescript
import {
    GenresApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new GenresApi(configuration);

let genreId: string; //ID of the genre to retrieve (default to undefined)

const { status, data } = await apiInstance.getGenreById(
    genreId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **genreId** | [**string**] | ID of the genre to retrieve | defaults to undefined|


### Return type

**WrapperGenre**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get genre by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listGenres**
> Array<WrapperGenre> listGenres()


### Example

```typescript
import {
    GenresApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new GenresApi(configuration);

let limit: number; //Max number of genres to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.listGenres(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] | Max number of genres to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperGenre>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List all genres |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateGenre**
> WrapperGenre updateGenre(wrapperNewGenre)


### Example

```typescript
import {
    GenresApi,
    Configuration,
    WrapperNewGenre
} from './api';

const configuration = new Configuration();
const apiInstance = new GenresApi(configuration);

let genreId: string; //ID of the genre to update (default to undefined)
let wrapperNewGenre: WrapperNewGenre; //

const { status, data } = await apiInstance.updateGenre(
    genreId,
    wrapperNewGenre
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewGenre** | **WrapperNewGenre**|  | |
| **genreId** | [**string**] | ID of the genre to update | defaults to undefined|


### Return type

**WrapperGenre**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update genre by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

