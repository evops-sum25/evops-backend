#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

mod impls;

tonic::include_proto!("evops.api.v1");

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("api-descriptor");
