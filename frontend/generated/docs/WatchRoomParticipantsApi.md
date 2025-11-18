# WatchRoomParticipantsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createWatchRoomParticipant**](#createwatchroomparticipant) | **POST** /watch-room-participants | |
|[**deleteParticipantsByRoom**](#deleteparticipantsbyroom) | **DELETE** /watch-rooms/{room_id}/participants | |
|[**deleteWatchRoomParticipant**](#deletewatchroomparticipant) | **DELETE** /watch-rooms/{room_id}/participants/{user_id} | |
|[**getWatchRoomParticipantById**](#getwatchroomparticipantbyid) | **GET** /watch-room-participants/{participant_id} | |
|[**listParticipantsByRoom**](#listparticipantsbyroom) | **GET** /watch-rooms/{room_id}/participants | |

# **createWatchRoomParticipant**
> WatchRoomParticipant createWatchRoomParticipant(newWatchRoomParticipant)


### Example

```typescript
import {
    WatchRoomParticipantsApi,
    Configuration,
    NewWatchRoomParticipant
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomParticipantsApi(configuration);

let newWatchRoomParticipant: NewWatchRoomParticipant; //

const { status, data } = await apiInstance.createWatchRoomParticipant(
    newWatchRoomParticipant
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newWatchRoomParticipant** | **NewWatchRoomParticipant**|  | |


### Return type

**WatchRoomParticipant**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Add a new participant to a watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteParticipantsByRoom**
> number deleteParticipantsByRoom()


### Example

```typescript
import {
    WatchRoomParticipantsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomParticipantsApi(configuration);

let roomId: string; //UUID of the room whose participants to delete (default to undefined)

const { status, data } = await apiInstance.deleteParticipantsByRoom(
    roomId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room whose participants to delete | defaults to undefined|


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
|**200** | Delete all participants from a watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWatchRoomParticipant**
> number deleteWatchRoomParticipant()


### Example

```typescript
import {
    WatchRoomParticipantsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomParticipantsApi(configuration);

let roomId: string; //UUID of the room (default to undefined)
let userId: string; //UUID of the participant to remove (default to undefined)

const { status, data } = await apiInstance.deleteWatchRoomParticipant(
    roomId,
    userId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room | defaults to undefined|
| **userId** | [**string**] | UUID of the participant to remove | defaults to undefined|


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
|**200** | Delete a participant from a room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWatchRoomParticipantById**
> WatchRoomParticipant getWatchRoomParticipantById()


### Example

```typescript
import {
    WatchRoomParticipantsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomParticipantsApi(configuration);

let participantId: number; //ID of the participant to retrieve (default to undefined)

const { status, data } = await apiInstance.getWatchRoomParticipantById(
    participantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **participantId** | [**number**] | ID of the participant to retrieve | defaults to undefined|


### Return type

**WatchRoomParticipant**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get a watch room participant by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listParticipantsByRoom**
> Array<WatchRoomParticipant> listParticipantsByRoom()


### Example

```typescript
import {
    WatchRoomParticipantsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomParticipantsApi(configuration);

let roomId: string; //UUID of the room whose participants to retrieve (default to undefined)
let limit: number; //Max number of participants (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.listParticipantsByRoom(
    roomId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room whose participants to retrieve | defaults to undefined|
| **limit** | [**number**] | Max number of participants | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WatchRoomParticipant>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List participants by room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

