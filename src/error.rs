use thiserror::Error;

#[derive(Error, Debug)]
pub enum General {
    #[error("could not load metadata: {0}")]
    LoadMetadata(rexiv2::Rexiv2Error),
    #[error("not embedded thumbnail found")]
    NoThumbnail,
    #[error("could not store thumbnail: {0}")]
    StoreThumbnail(rexiv2::Rexiv2Error),
    #[error("could not get thumbnail extension: {0}")]
    GetExtension(rexiv2::Rexiv2Error),
    #[error("could not store metadata to thumbnail: {0}")]
    StoreMetadata(rexiv2::Rexiv2Error),
}
