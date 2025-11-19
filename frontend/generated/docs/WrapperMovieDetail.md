# WrapperMovieDetail


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**description** | **string** |  | [optional] [default to undefined]
**duration_minutes** | **number** |  | [optional] [default to undefined]
**genres** | [**Array&lt;WrapperGenre&gt;**](WrapperGenre.md) |  | [default to undefined]
**id** | **string** |  | [default to undefined]
**mpaa_rating** | **string** |  | [optional] [default to undefined]
**people** | [**Array&lt;MovieCrewDetail&gt;**](MovieCrewDetail.md) |  | [default to undefined]
**poster_url** | **string** |  | [optional] [default to undefined]
**release_date** | **string** |  | [optional] [default to undefined]
**reviews** | [**Array&lt;WrapperReview&gt;**](WrapperReview.md) |  | [default to undefined]
**title** | **string** |  | [default to undefined]
**updated_at** | **string** |  | [default to undefined]
**video_url** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { WrapperMovieDetail } from './api';

const instance: WrapperMovieDetail = {
    created_at,
    description,
    duration_minutes,
    genres,
    id,
    mpaa_rating,
    people,
    poster_url,
    release_date,
    reviews,
    title,
    updated_at,
    video_url,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
