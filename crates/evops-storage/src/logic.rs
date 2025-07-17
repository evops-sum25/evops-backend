use std::pin::Pin;

use bytes::Bytes;
use evops_models::{ApiError, ApiResult};
use futures::{Stream, StreamExt as _};
use minio::s3::types::S3Api as _;

impl crate::Storage {
    fn event_image_filename(id: evops_models::EventImageId) -> String {
        format!("{id}.webp")
    }

    pub async fn upload_event_image(
        &self,
        id: evops_models::EventImageId,
        image: evops_models::EventImage,
    ) -> ApiResult<()> {
        self.client
            .put_object(
                Self::BUCKET_EVENT_IMAGES,
                Self::event_image_filename(id),
                image.encode_as_webp()?.into(),
            )
            .send()
            .await
            .map_err(|e| ApiError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn stream_event_image(
        &self,
        id: evops_models::EventImageId,
    ) -> ApiResult<Pin<Box<dyn Stream<Item = ApiResult<Bytes>> + Send>>> {
        let response = {
            self.client
                .get_object(Self::BUCKET_EVENT_IMAGES, Self::event_image_filename(id))
                .send()
                .await
                .map_err(|e| ApiError::Storage(e.to_string()))?
        };
        let stream = {
            response
                .content
                .to_stream()
                .await
                .map_err(|e| ApiError::Storage(e.to_string()))?
                .0
                .map(|chunk| -> ApiResult<Bytes> {
                    chunk.map_err(|e| ApiError::Storage(e.to_string()))
                })
                .boxed()
        };
        Ok(stream)
    }

    pub async fn delete_event_image(&self, id: evops_models::EventImageId) -> ApiResult<()> {
        self.client
            .delete_object(Self::BUCKET_EVENT_IMAGES, Self::event_image_filename(id))
            .send()
            .await
            .map_err(|e| ApiError::Storage(e.to_string()))?;
        Ok(())
    }
}
