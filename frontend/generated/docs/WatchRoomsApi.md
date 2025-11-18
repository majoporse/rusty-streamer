# WatchRoomsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**connectRoom**](#connectroom) | **GET** /watch-rooms/{room_id}/connect | |
|[**createWatchRoom**](#createwatchroom) | **POST** /watch-rooms | |
|[**deleteWatchRoom**](#deletewatchroom) | **DELETE** /watch-rooms/{room_id} | |
|[**getWatchRoomById**](#getwatchroombyid) | **GET** /watch-rooms/{room_id} | |
|[**listRoomsByHost**](#listroomsbyhost) | **GET** /users/{host_user_id}/watch-rooms | |
|[**updateWatchRoom**](#updatewatchroom) | **PUT** /watch-rooms/{room_id} | |

# **connectRoom**
> connectRoom()


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let roomId: string; //UUID of the room to connect to (default to undefined)

const { status, data } = await apiInstance.connectRoom(
    roomId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room to connect to | defaults to undefined|


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Connect to a watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createWatchRoom**
> WatchRoom createWatchRoom(newWatchRoom)


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration,
    NewWatchRoom
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let newWatchRoom: NewWatchRoom; //

const { status, data } = await apiInstance.createWatchRoom(
    newWatchRoom
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newWatchRoom** | **NewWatchRoom**|  | |


### Return type

**WatchRoom**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWatchRoom**
> number deleteWatchRoom()


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let roomId: string; //UUID of the room to delete (default to undefined)

const { status, data } = await apiInstance.deleteWatchRoom(
    roomId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room to delete | defaults to undefined|


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
|**200** | Delete a watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWatchRoomById**
> WatchRoom getWatchRoomById()


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let roomId: string; //UUID of the watch room to retrieve (default to undefined)

const { status, data } = await apiInstance.getWatchRoomById(
    roomId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the watch room to retrieve | defaults to undefined|


### Return type

**WatchRoom**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get watch room by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listRoomsByHost**
> Array<WatchRoom> listRoomsByHost()


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let hostUserId: string; //UUID of the user hosting the rooms (default to undefined)
let limit: number; //Max number of rooms to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.listRoomsByHost(
    hostUserId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **hostUserId** | [**string**] | UUID of the user hosting the rooms | defaults to undefined|
| **limit** | [**number**] | Max number of rooms to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WatchRoom>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List rooms hosted by user |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateWatchRoom**
> WatchRoom updateWatchRoom(updateWatchRoom)


### Example

```typescript
import {
    WatchRoomsApi,
    Configuration,
    UpdateWatchRoom
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomsApi(configuration);

let roomId: string; //UUID of the room to update (default to undefined)
let updateWatchRoom: UpdateWatchRoom; //

const { status, data } = await apiInstance.updateWatchRoom(
    roomId,
    updateWatchRoom
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateWatchRoom** | **UpdateWatchRoom**|  | |
| **roomId** | [**string**] | UUID of the room to update | defaults to undefined|


### Return type

**WatchRoom**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update an existing watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

