# WatchRoomMessagesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createWatchRoomMessage**](#createwatchroommessage) | **POST** /watch-room-messages | |
|[**deleteMessagesByRoom**](#deletemessagesbyroom) | **DELETE** /watch-rooms/{room_id}/messages | |
|[**deleteWatchRoomMessage**](#deletewatchroommessage) | **DELETE** /watch-room-messages/{message_id} | |
|[**getWatchRoomMessageById**](#getwatchroommessagebyid) | **GET** /watch-room-messages/{message_id} | |
|[**listMessagesByRoom**](#listmessagesbyroom) | **GET** /watch-rooms/{room_id}/messages | |

# **createWatchRoomMessage**
> WatchRoomMessage createWatchRoomMessage(newWatchRoomMessage)


### Example

```typescript
import {
    WatchRoomMessagesApi,
    Configuration,
    NewWatchRoomMessage
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomMessagesApi(configuration);

let newWatchRoomMessage: NewWatchRoomMessage; //

const { status, data } = await apiInstance.createWatchRoomMessage(
    newWatchRoomMessage
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **newWatchRoomMessage** | **NewWatchRoomMessage**|  | |


### Return type

**WatchRoomMessage**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new message in a watch room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteMessagesByRoom**
> number deleteMessagesByRoom()


### Example

```typescript
import {
    WatchRoomMessagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomMessagesApi(configuration);

let roomId: string; //UUID of the room whose messages to delete (default to undefined)

const { status, data } = await apiInstance.deleteMessagesByRoom(
    roomId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room whose messages to delete | defaults to undefined|


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
|**200** | Delete all messages from a room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteWatchRoomMessage**
> number deleteWatchRoomMessage()


### Example

```typescript
import {
    WatchRoomMessagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomMessagesApi(configuration);

let messageId: number; //ID of the message to delete (default to undefined)

const { status, data } = await apiInstance.deleteWatchRoomMessage(
    messageId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **messageId** | [**number**] | ID of the message to delete | defaults to undefined|


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
|**200** | Delete a message |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWatchRoomMessageById**
> WatchRoomMessage getWatchRoomMessageById()


### Example

```typescript
import {
    WatchRoomMessagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomMessagesApi(configuration);

let messageId: number; //ID of the message to retrieve (default to undefined)

const { status, data } = await apiInstance.getWatchRoomMessageById(
    messageId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **messageId** | [**number**] | ID of the message to retrieve | defaults to undefined|


### Return type

**WatchRoomMessage**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get a watch room message by ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listMessagesByRoom**
> Array<WatchRoomMessage> listMessagesByRoom()


### Example

```typescript
import {
    WatchRoomMessagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WatchRoomMessagesApi(configuration);

let roomId: string; //UUID of the room whose messages to retrieve (default to undefined)
let limit: number; //Max number of messages (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.listMessagesByRoom(
    roomId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **roomId** | [**string**] | UUID of the room whose messages to retrieve | defaults to undefined|
| **limit** | [**number**] | Max number of messages | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WatchRoomMessage>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List messages by room |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

