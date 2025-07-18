# evops-storage

## Overview

Provides object storage operations for event images using MinIO. Handles image uploads, streaming downloads, and deletions.

### Key Files

- `lib.rs`: Client Initialization
    Sets up the storage client and ensures required buckets exist.
    - Creates MinIO/S3 client with authentication
    - Checks storage connection
    - Auto-creates `event-images` bucket if missing

- `logic.rs`: Business Logic
Implements CRUD operations for event images. Uses WebP format for storage.
Key methods:
    - `upload_event_image` - Encodes image as WebP and uploads to storage
    - `stream_event_image` - Streams image bytes chunk-by-chunk
    - `delete_event_image` - Removes image from storage
