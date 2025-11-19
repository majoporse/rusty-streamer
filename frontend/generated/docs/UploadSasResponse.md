# UploadSasResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blob_url** | **string** | canonical blob URL (without SAS) for later reference | [default to undefined]
**expires_at** | **string** | optional expiration timestamp for the upload URL | [optional] [default to undefined]
**upload_url** | **string** | full URL suitable for uploading (may include SAS token) | [default to undefined]

## Example

```typescript
import { UploadSasResponse } from './api';

const instance: UploadSasResponse = {
    blob_url,
    expires_at,
    upload_url,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
