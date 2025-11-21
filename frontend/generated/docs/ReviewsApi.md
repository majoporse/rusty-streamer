# ReviewsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createReview**](#createreview) | **POST** /reviews | |
|[**deleteReview**](#deletereview) | **DELETE** /reviews/{review_id} | |
|[**getAllReviews**](#getallreviews) | **GET** /reviews | |
|[**getReviewById**](#getreviewbyid) | **GET** /reviews/{review_id} | |
|[**getReviewsByMovieId**](#getreviewsbymovieid) | **GET** /reviews/movie/{movie_id} | |
|[**getReviewsByUserId**](#getreviewsbyuserid) | **GET** /reviews/user/{user_id} | |
|[**updateReview**](#updatereview) | **PUT** /reviews/{review_id} | |

# **createReview**
> WrapperReview createReview(wrapperNewReview)


### Example

```typescript
import {
    ReviewsApi,
    Configuration,
    WrapperNewReview
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let wrapperNewReview: WrapperNewReview; //

const { status, data } = await apiInstance.createReview(
    wrapperNewReview
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewReview** | **WrapperNewReview**|  | |


### Return type

**WrapperReview**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Create a new review |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteReview**
> number deleteReview()


### Example

```typescript
import {
    ReviewsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let reviewId: number; //ID of the review to delete (default to undefined)

const { status, data } = await apiInstance.deleteReview(
    reviewId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **reviewId** | [**number**] | ID of the review to delete | defaults to undefined|


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
|**200** | Delete review by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllReviews**
> Array<WrapperReview> getAllReviews()


### Example

```typescript
import {
    ReviewsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let limit: number; //Max number of reviews to return (optional) (default to undefined)
let offset: number; //Pagination offset (optional) (default to undefined)

const { status, data } = await apiInstance.getAllReviews(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] | Max number of reviews to return | (optional) defaults to undefined|
| **offset** | [**number**] | Pagination offset | (optional) defaults to undefined|


### Return type

**Array<WrapperReview>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List all reviews |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getReviewById**
> WrapperReview getReviewById()


### Example

```typescript
import {
    ReviewsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let reviewId: number; //ID of the review to retrieve (default to undefined)

const { status, data } = await apiInstance.getReviewById(
    reviewId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **reviewId** | [**number**] | ID of the review to retrieve | defaults to undefined|


### Return type

**WrapperReview**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get review by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getReviewsByMovieId**
> Array<WrapperReview> getReviewsByMovieId()


### Example

```typescript
import {
    ReviewsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let movieId: number; //ID of the movie to get reviews for (default to undefined)
let limit: number; //Max number of reviews to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.getReviewsByMovieId(
    movieId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **movieId** | [**number**] | ID of the movie to get reviews for | defaults to undefined|
| **limit** | [**number**] | Max number of reviews to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperReview>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get reviews by movie ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getReviewsByUserId**
> Array<WrapperReview> getReviewsByUserId()


### Example

```typescript
import {
    ReviewsApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let userId: string; //ID of the user to get reviews for (default to undefined)
let limit: number; //Max number of reviews to return (default to undefined)
let offset: number; //Pagination offset (default to undefined)

const { status, data } = await apiInstance.getReviewsByUserId(
    userId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userId** | [**string**] | ID of the user to get reviews for | defaults to undefined|
| **limit** | [**number**] | Max number of reviews to return | defaults to undefined|
| **offset** | [**number**] | Pagination offset | defaults to undefined|


### Return type

**Array<WrapperReview>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Get reviews by user ID |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateReview**
> WrapperReview updateReview(wrapperNewReview)


### Example

```typescript
import {
    ReviewsApi,
    Configuration,
    WrapperNewReview
} from './api';

const configuration = new Configuration();
const apiInstance = new ReviewsApi(configuration);

let reviewId: number; //ID of the review to update (default to undefined)
let wrapperNewReview: WrapperNewReview; //

const { status, data } = await apiInstance.updateReview(
    reviewId,
    wrapperNewReview
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **wrapperNewReview** | **WrapperNewReview**|  | |
| **reviewId** | [**number**] | ID of the review to update | defaults to undefined|


### Return type

**WrapperReview**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Update review by ID |  -  |
|**404** | Not Found |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

