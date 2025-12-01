pub mod shader;
#[cfg(feature = "vulkan")]
pub mod vk;

use crate::error::*;
use crate::utils::*;

#[inline(always)]
fn to_err(e: impl std::error::Error + 'static) -> Error {
    (ErrorKind::GpuError, e).into()
}
