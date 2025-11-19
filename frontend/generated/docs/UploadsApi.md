# UploadsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**requestUploadSas**](#requestuploadsas) | **POST** /uploads/request-sas | |

# **requestUploadSas**
> UploadSasResponse requestUploadSas(uploadSasRequest)


### Example

```typescript
import {
    UploadsApi,
    Configuration,
    UploadSasRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new UploadsApi(configuration);

let uploadSasRequest: UploadSasRequest; //

const { status, data } = await apiInstance.requestUploadSas(
    uploadSasRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **uploadSasRequest** | **UploadSasRequest**|  | |


### Return type

**UploadSasResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | SAS token for upload |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

