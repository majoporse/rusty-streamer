# WatchlistApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createWatchlistItem**](#createwatchlistitem) | **POST** /watchlist | |
|[**deleteWatchlistItem**](#deletewatchlistitem) | **DELETE** /users/{user_id}/watchlist/{content_id} | |
|[**getWatchlistByUser**](#getwatchlistbyuser) | **GET** /users/{user_id}/watchlist | |

# **createWatchlistItem**
> Watchlist createWatchlistItem(newWatchlist)


### Example

```typescript
import {
    WatchlistApi,
    Configuration,
    NewWatchlist
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchlistApi(configuration);

let newWatchlist: NewWatchlist; //

const { status, data } = await apiInstance.createWatchlistItem(
    newWatchlist
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newWatchlist** | **NewWatchlist**|  | |


### Return type

**Watchlist**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Add a new item to the watchlist |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWatchlistItem**
> number deleteWatchlistItem()


### Example

```typescript
import {
    WatchlistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchlistApi(configuration);

let userId: string; //UUID of the user (default to undefined)
let contentId: string; //UUID of the content item (default to undefined)

const { status, data } = await apiInstance.deleteWatchlistItem(
    userId,
    contentId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] | UUID of the user | defaults to undefined|
| **contentId** | [**string**] | UUID of the content item | defaults to undefined|


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
|**200** | Delete item from watchlist |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWatchlistByUser**
> Array<Watchlist> getWatchlistByUser()


### Example

```typescript
import {
    WatchlistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchlistApi(configuration);

let userId: string; //UUID of the user whose watchlist to retrieve (default to undefined)
let limit: number; //Max number of items to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.getWatchlistByUser(
    userId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] | UUID of the user whose watchlist to retrieve | defaults to undefined|
| **limit** | [**number**] | Max number of items to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<Watchlist>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List user watchlist |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

