use bytes::Bytes;
use minio::s3::types::S3Api as _;
use tap::Conv as _;

impl crate::Storage {
    fn event_image_filename(id: evops_models::EventImageId) -> String {
        format!("{id}.webp")
    }

    pub async fn upload_event_image(
        &self,
        id: evops_models::EventImageId,
        image: evops_models::EventImage,
    ) -> eyre::Result<(), minio::s3::error::Error> {
        self.client
            .put_object(
                Self::BUCKET_EVENT_IMAGES,
                Self::event_image_filename(id),
                image.into_inner().into_bytes().conv::<Bytes>().into(),
            )
            .send()
            .await?;
        Ok(())
    }
}
