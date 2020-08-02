use crate::gpu::vk;
use crate::gpu::Protected;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{
    GrVkAlloc, GrVkBackendMemory, GrVkDrawableInfo, GrVkImageInfo, GrVkYcbcrConversionInfo,
};
use std::ffi::CStr;
use std::os::raw;
use std::ptr;

pub type GraphicsBackendMemory = GrVkBackendMemory;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Alloc {
    pub memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub flags: AllocFlag,
    pub backend_memory: GraphicsBackendMemory,
    uses_system_heap: bool,
}
unsafe impl Send for Alloc {}
unsafe impl Sync for Alloc {}

impl NativeTransmutable<GrVkAlloc> for Alloc {}
#[test]
fn test_vk_alloc_layout() {
    Alloc::test_layout()
}

impl Default for Alloc {
    fn default() -> Self {
        Self {
            memory: vk::NULL_HANDLE.into(),
            offset: 0,
            size: 0,
            flags: AllocFlag::empty(),
            backend_memory: 0,
            uses_system_heap: false,
        }
    }
}

impl PartialEq for Alloc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_GrVkAlloc_Equals(self.native(), other.native()) }
    }
}

bitflags! {
    pub struct AllocFlag : u32 {
        const NONCOHERENT = sb::GrVkAlloc_Flag_kNoncoherent_Flag as _;
        const MAPPABLE = sb::GrVkAlloc_Flag_kMappable_Flag as _;
    }
}

impl Alloc {
    /// # Safety
    /// The memory's lifetime is expected to outlive the lifetime of the returned object.
    pub unsafe fn from_device_memory(
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: AllocFlag,
    ) -> Alloc {
        Alloc::construct(|alloc| {
            sb::C_GrVkAlloc_Construct(alloc, memory, offset, size, flags.bits())
        })
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct YcbcrConversionInfo {
    pub format: vk::Format,
    pub external_format: u64,
    pub ycrbcr_model: vk::SamplerYcbcrModelConversion,
    pub ycbcr_range: vk::SamplerYcbcrRange,
    pub x_chroma_offset: vk::ChromaLocation,
    pub y_chroma_offset: vk::ChromaLocation,
    pub chroma_filter: vk::Filter,
    pub force_explicit_reconstruction: vk::Bool32,
    pub format_features: vk::FormatFeatureFlags,
}

impl NativeTransmutable<GrVkYcbcrConversionInfo> for YcbcrConversionInfo {}
#[test]
fn test_ycbcr_conversion_info_layout() {
    YcbcrConversionInfo::test_layout()
}

impl PartialEq for YcbcrConversionInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_GrVkYcbcrConversionInfo_Equals(self.native(), other.native()) }
    }
}

impl Default for YcbcrConversionInfo {
    fn default() -> Self {
        YcbcrConversionInfo {
            format: vk::Format::UNDEFINED,
            external_format: 0,
            ycrbcr_model: vk::SamplerYcbcrModelConversion::RGB_IDENTITY,
            ycbcr_range: vk::SamplerYcbcrRange::ITU_FULL,
            x_chroma_offset: vk::ChromaLocation::COSITED_EVEN,
            y_chroma_offset: vk::ChromaLocation::COSITED_EVEN,
            chroma_filter: vk::Filter::NEAREST,
            force_explicit_reconstruction: 0,
            format_features: 0,
        }
    }
}

impl YcbcrConversionInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_format(
        format: vk::Format,
        external_format: u64,
        ycrbcr_model: vk::SamplerYcbcrModelConversion,
        ycbcr_range: vk::SamplerYcbcrRange,
        x_chroma_offset: vk::ChromaLocation,
        y_chroma_offset: vk::ChromaLocation,
        chroma_filter: vk::Filter,
        force_explicit_reconstruction: vk::Bool32,
        format_features: vk::FormatFeatureFlags,
    ) -> YcbcrConversionInfo {
        debug_assert!(ycrbcr_model != vk::SamplerYcbcrModelConversion::RGB_IDENTITY);
        debug_assert!((format != vk::Format::UNDEFINED) ^ (external_format != 0));
        YcbcrConversionInfo {
            format,
            external_format,
            ycrbcr_model,
            ycbcr_range,
            x_chroma_offset,
            y_chroma_offset,
            chroma_filter,
            force_explicit_reconstruction,
            format_features,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ycrbcr_model: vk::SamplerYcbcrModelConversion,
        ycbcr_range: vk::SamplerYcbcrRange,
        x_chroma_offset: vk::ChromaLocation,
        y_chroma_offset: vk::ChromaLocation,
        chroma_filter: vk::Filter,
        force_explicit_reconstruction: vk::Bool32,
        external_format: u64,
        external_format_features: vk::FormatFeatureFlags,
    ) -> YcbcrConversionInfo {
        Self::new_with_format(
            vk::Format::UNDEFINED,
            external_format,
            ycrbcr_model,
            ycbcr_range,
            x_chroma_offset,
            y_chroma_offset,
            chroma_filter,
            force_explicit_reconstruction,
            external_format_features,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.ycrbcr_model != vk::SamplerYcbcrModelConversion::RGB_IDENTITY
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ImageInfo {
    pub image: vk::Image,
    pub alloc: Alloc,
    pub tiling: vk::ImageTiling,
    pub layout: vk::ImageLayout,
    pub format: vk::Format,
    pub level_count: u32,
    pub current_queue_family: u32,
    pub protected: Protected,
    pub ycbcr_conversion_info: YcbcrConversionInfo,
}
unsafe impl Send for ImageInfo {}
unsafe impl Sync for ImageInfo {}

impl NativeTransmutable<GrVkImageInfo> for ImageInfo {}
#[test]
fn test_image_info_layout() {
    ImageInfo::test_layout()
}

impl Default for ImageInfo {
    fn default() -> Self {
        ImageInfo {
            image: vk::NULL_HANDLE.into(),
            alloc: Alloc::default(),
            tiling: vk::ImageTiling::OPTIMAL,
            layout: vk::ImageLayout::UNDEFINED,
            format: vk::Format::UNDEFINED,
            level_count: 0,
            current_queue_family: vk::QUEUE_FAMILY_IGNORED,
            protected: Protected::No,
            ycbcr_conversion_info: Default::default(),
        }
    }
}

impl ImageInfo {
    /// # Safety
    /// The Vulkan `image` and `alloc` must outlive the lifetime of the ImageInfo returned.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn new(
        image: vk::Image,
        alloc: Alloc,
        tiling: vk::ImageTiling,
        layout: vk::ImageLayout,
        format: vk::Format,
        level_count: u32,
        current_queue_family: impl Into<Option<u32>>,
        ycbcr_conversion_info: impl Into<Option<YcbcrConversionInfo>>,
        protected: impl Into<Option<Protected>>, // added in m77
    ) -> ImageInfo {
        let current_queue_family = current_queue_family
            .into()
            .unwrap_or(vk::QUEUE_FAMILY_IGNORED);
        let ycbcr_conversion_info = ycbcr_conversion_info.into().unwrap_or_default();
        let protected = protected.into().unwrap_or(Protected::No);
        Self {
            image,
            alloc,
            tiling,
            layout,
            format,
            level_count,
            current_queue_family,
            protected,
            ycbcr_conversion_info,
        }
    }

    /// # Safety
    /// The Vulkan `image` and `alloc` must outlive the lifetime of the ImageInfo returned.
    #[deprecated(since = "0.19.0", note = "use new()")]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn from_image(
        image: vk::Image,
        alloc: Alloc,
        tiling: vk::ImageTiling,
        layout: vk::ImageLayout,
        format: vk::Format,
        level_count: u32,
        current_queue_family: impl Into<Option<u32>>,
        ycbcr_conversion_info: impl Into<Option<YcbcrConversionInfo>>,
        protected: impl Into<Option<Protected>>, // added in m77
    ) -> ImageInfo {
        Self::new(
            image,
            alloc,
            tiling,
            layout,
            format,
            level_count,
            current_queue_family,
            ycbcr_conversion_info,
            protected,
        )
    }

    /// # Safety
    /// The Vulkan `info.image` and `info.alloc` must outlive the lifetime of the ImageInfo returned.
    pub unsafe fn from_info(info: &ImageInfo, layout: vk::ImageLayout) -> ImageInfo {
        Self::new(
            info.image,
            info.alloc,
            info.tiling,
            layout,
            info.format,
            info.level_count,
            info.current_queue_family,
            info.ycbcr_conversion_info,
            info.protected,
        )
    }
}

// TODO: Tried to use CStr here, but &CStr needs a lifetime parameter
//       which would make the whole GetProc trait generic.
#[derive(Copy, Clone, Debug)]
pub enum GetProcOf {
    Instance(vk::Instance, *const raw::c_char),
    Device(vk::Device, *const raw::c_char),
}

impl GetProcOf {
    /// # Safety
    /// The referred raw `name` strings must outlive the returned CStr reference.
    pub unsafe fn name(&self) -> &CStr {
        match *self {
            GetProcOf::Instance(_, name) => CStr::from_ptr(name),
            GetProcOf::Device(_, name) => CStr::from_ptr(name),
        }
    }
}

// TODO: Really would like to see a fn() signature here, but I'm always running
//       into a conflict between extern "C" and extern "system".
pub type GetProcResult = *const raw::c_void;

// GetProc is a trait alias for Fn...
pub trait GetProc: Fn(GetProcOf) -> GetProcResult {}
impl<T> GetProc for T where T: Fn(GetProcOf) -> GetProcResult {}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DrawableInfo {
    pub secondary_command_buffer: vk::CommandBuffer,
    pub color_attachment_index: u32,
    pub compatible_render_pass: vk::RenderPass,
    pub format: vk::Format,
    pub draw_bounds: *mut vk::Rect2D,
    pub image: vk::Image,
}
unsafe impl Send for DrawableInfo {}
unsafe impl Sync for DrawableInfo {}

impl Default for DrawableInfo {
    fn default() -> Self {
        DrawableInfo {
            secondary_command_buffer: vk::NULL_HANDLE.into(),
            color_attachment_index: 0,
            compatible_render_pass: vk::NULL_HANDLE.into(),
            format: vk::Format::UNDEFINED,
            draw_bounds: ptr::null_mut(),
            image: vk::NULL_HANDLE.into(),
        }
    }
}

impl NativeTransmutable<GrVkDrawableInfo> for DrawableInfo {}
#[test]
fn test_drawable_info_layout() {
    DrawableInfo::test_layout()
}
