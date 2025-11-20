import { BlockBlobClient } from "@azure/storage-blob";

export async function uploadAzureSas(
  file: File,
  sasUrl: string,
  onProgress?: (loadedBytes: number, totalBytes: number, percent: number) => void
) {
  const blobClient = new BlockBlobClient(sasUrl);

  const blockSize = 8 * 1024 * 1024; // 8 MB
  const concurrency = 5; // parallel uploads
  const a = new Blob([file]);

  await blobClient.uploadData(a, {
    blockSize,
    concurrency,
    onProgress: (progress) => {
      const loaded = progress.loadedBytes || 0;
      const total = file.size || 0;
      const percent = total > 0 ? Math.round((loaded / total) * 100) : 0;
      if (onProgress) onProgress(loaded, total, percent);
    },
    blobHTTPHeaders: {
      blobContentType: "video/mp4",
    },
  });

  return {
    status: "ok",
  };
}
