# WatchHistoryApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createWatchHistory**](#createwatchhistory) | **POST** /watch-history | |
|[**deleteWatchHistory**](#deletewatchhistory) | **DELETE** /watch-history/{id} | |
|[**getWatchHistoryById**](#getwatchhistorybyid) | **GET** /watch-history/{id} | |
|[**listWatchHistoryByUser**](#listwatchhistorybyuser) | **GET** /users/{user_id}/watch-history | |
|[**updateWatchHistory**](#updatewatchhistory) | **PUT** /watch-history/{id} | |

# **createWatchHistory**
> WatchHistory createWatchHistory(newWatchHistory)


### Example

```typescript
import {
    WatchHistoryApi,
    Configuration,
    NewWatchHistory
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchHistoryApi(configuration);

let newWatchHistory: NewWatchHistory; //

const { status, data } = await apiInstance.createWatchHistory(
    newWatchHistory
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newWatchHistory** | **NewWatchHistory**|  | |


### Return type

**WatchHistory**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new watch history entry |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWatchHistory**
> number deleteWatchHistory()


### Example

```typescript
import {
    WatchHistoryApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchHistoryApi(configuration);

let id: number; //ID of the watch history entry to delete (default to undefined)

const { status, data } = await apiInstance.deleteWatchHistory(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] | ID of the watch history entry to delete | defaults to undefined|


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
|**200** | Delete watch history entry |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWatchHistoryById**
> WatchHistory getWatchHistoryById()


### Example

```typescript
import {
    WatchHistoryApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchHistoryApi(configuration);

let id: number; //ID of the watch history entry (default to undefined)

const { status, data } = await apiInstance.getWatchHistoryById(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] | ID of the watch history entry | defaults to undefined|


### Return type

**WatchHistory**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get watch history by ID |  -  |
|**404** | Not found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listWatchHistoryByUser**
> Array<WatchHistory> listWatchHistoryByUser()


### Example

```typescript
import {
    WatchHistoryApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchHistoryApi(configuration);

let userId: string; //UUID of the user whose history to retrieve (default to undefined)
let limit: number; //Max number of items to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.listWatchHistoryByUser(
    userId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] | UUID of the user whose history to retrieve | defaults to undefined|
| **limit** | [**number**] | Max number of items to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WatchHistory>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List user watch history |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateWatchHistory**
> WatchHistory updateWatchHistory(updateWatchHistory)


### Example

```typescript
import {
    WatchHistoryApi,
    Configuration,
    UpdateWatchHistory
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchHistoryApi(configuration);

let id: number; //ID of the watch history entry to update (default to undefined)
let updateWatchHistory: UpdateWatchHistory; //

const { status, data } = await apiInstance.updateWatchHistory(
    id,
    updateWatchHistory
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateWatchHistory** | **UpdateWatchHistory**|  | |
| **id** | [**number**] | ID of the watch history entry to update | defaults to undefined|


### Return type

**WatchHistory**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update watch history entry |  -  |
|**404** | Not found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

