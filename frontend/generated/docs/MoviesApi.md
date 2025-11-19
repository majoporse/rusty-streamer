# MoviesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createMovie**](#createmovie) | **POST** /movies | |
|[**deleteMovie**](#deletemovie) | **DELETE** /movies/{movie_id} | |
|[**getAllMovies**](#getallmovies) | **GET** /movies | |
|[**getMovieById**](#getmoviebyid) | **GET** /movies/{movie_id} | |
|[**getMovieDetailsById**](#getmoviedetailsbyid) | **GET** /movies/{movie_id}/details | |
|[**searchMoviesByActor**](#searchmoviesbyactor) | **GET** /search/movies/person | |
|[**searchMoviesByTitle**](#searchmoviesbytitle) | **GET** /search/movies/title | |
|[**updateMovie**](#updatemovie) | **PUT** /movies/{movie_id} | |

# **createMovie**
> WrapperMovie createMovie(wrapperNewMovie)


### Example

```typescript
import {
    MoviesApi,
    Configuration,
    WrapperNewMovie
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let wrapperNewMovie: WrapperNewMovie; //

const { status, data } = await apiInstance.createMovie(
    wrapperNewMovie
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewMovie** | **WrapperNewMovie**|  | |


### Return type

**WrapperMovie**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Movie created successfully |  -  |
|**400** | Invalid input |  -  |
|**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteMovie**
> number deleteMovie()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let movieId: string; //ID of the movie to delete (default to undefined)

const { status, data } = await apiInstance.deleteMovie(
    movieId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **movieId** | [**string**] | ID of the movie to delete | defaults to undefined|


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
|**200** | Delete movie by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllMovies**
> Array<WrapperMovie> getAllMovies()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let limit: number; //Max number of movies to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.getAllMovies(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] | Max number of movies to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperMovie>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get all movies |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMovieById**
> WrapperMovie getMovieById()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let movieId: string; //ID of the movie to retrieve (default to undefined)

const { status, data } = await apiInstance.getMovieById(
    movieId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **movieId** | [**string**] | ID of the movie to retrieve | defaults to undefined|


### Return type

**WrapperMovie**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get movie by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMovieDetailsById**
> WrapperMovieDetail getMovieDetailsById()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let movieId: string; //ID of the movie to retrieve details for (default to undefined)

const { status, data } = await apiInstance.getMovieDetailsById(
    movieId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **movieId** | [**string**] | ID of the movie to retrieve details for | defaults to undefined|


### Return type

**WrapperMovieDetail**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get movie details by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchMoviesByActor**
> Array<WrapperMovie> searchMoviesByActor()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let actorName: string; //Actor name to search for movies (default to undefined)
let limit: number; //Max number of movies to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.searchMoviesByActor(
    actorName,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **actorName** | [**string**] | Actor name to search for movies | defaults to undefined|
| **limit** | [**number**] | Max number of movies to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperMovie>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Search movies by actor |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchMoviesByTitle**
> Array<WrapperMovie> searchMoviesByTitle()


### Example

```typescript
import {
    MoviesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let titleName: string; //Title query string to search for movies (default to undefined)
let limit: number; //Max number of movies to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.searchMoviesByTitle(
    titleName,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **titleName** | [**string**] | Title query string to search for movies | defaults to undefined|
| **limit** | [**number**] | Max number of movies to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperMovie>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Search movies by title |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateMovie**
> WrapperMovie updateMovie(wrapperNewMovie)


### Example

```typescript
import {
    MoviesApi,
    Configuration,
    WrapperNewMovie
} from './api';

const configuration = new Configuration();
const apiInstance = new MoviesApi(configuration);

let movieId: string; //ID of the movie to update (default to undefined)
let wrapperNewMovie: WrapperNewMovie; //

const { status, data } = await apiInstance.updateMovie(
    movieId,
    wrapperNewMovie
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewMovie** | **WrapperNewMovie**|  | |
| **movieId** | [**string**] | ID of the movie to update | defaults to undefined|


### Return type

**WrapperMovie**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update movie by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

