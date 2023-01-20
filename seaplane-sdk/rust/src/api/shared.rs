#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "compute_api_v1",
        feature = "locks_api_v1",
        feature = "metadata_api_v1",
        feature = "restrict_api_v1"
    )))
)]
pub mod v1;
