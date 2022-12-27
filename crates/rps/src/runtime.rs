//! Runtime API wrappers.

use std::{ffi::CStr, fmt};

use bitflags::bitflags;

use rps_sys as ffi;

bitflags! {
    /// Bitflags for resource (view) access attributes.
    ///
    /// If specified for a node parameter, it indicates the required resource layout and synchronizations before entering
    /// and after exiting the node.
    pub struct AccessFlags: i32 {
        /// Accessible as an indirect argument buffer.
        const INDIRECT_ARGS = ffi::RpsAccessFlagBits_RPS_ACCESS_INDIRECT_ARGS_BIT;

        /// Accessible as an index buffer.
        const INDEX_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_INDEX_BUFFER_BIT;

        /// Accessible as a vertex buffer.
        const VERTEX_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_VERTEX_BUFFER_BIT;

        /// Accessible as a constant buffer.
        const CONSTANT_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_CONSTANT_BUFFER_BIT;

        /// Accessible as a shader resource (readonly) view.
        const SHADER_RESOURCE = ffi::RpsAccessFlagBits_RPS_ACCESS_SHADER_RESOURCE_BIT;

        /// Accessible as a unordered access (shader readwrite) view.
        const UNORDERED_ACCESS = ffi::RpsAccessFlagBits_RPS_ACCESS_UNORDERED_ACCESS_BIT;

        /// Accessible as a shading rate image in a Variable Rate Shading (VRS) pass.
        const SHADING_RATE = ffi::RpsAccessFlagBits_RPS_ACCESS_SHADING_RATE_BIT;

        /// Accessible as a render target view.
        const RENDER_TARGET = ffi::RpsAccessFlagBits_RPS_ACCESS_RENDER_TARGET_BIT;

        /// Accessible as a readonly depth view.
        const DEPTH_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_READ_BIT;

        /// Accessible as a writable depth view.
        const DEPTH_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_WRITE_BIT;

        /// Accessible as a readonly stencil view.
        const STENCIL_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL_READ_BIT;

        /// Accessible as a writable stencil view.
        const STENCIL_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL_WRITE_BIT;

        /// Accessible for write as a stream out buffer.
        const STREAM_OUT = ffi::RpsAccessFlagBits_RPS_ACCESS_STREAM_OUT_BIT;

        /// Accessible as a copy source.
        const COPY_SRC = ffi::RpsAccessFlagBits_RPS_ACCESS_COPY_SRC_BIT;

        /// Accessible as a copy target.
        const COPY_DEST = ffi::RpsAccessFlagBits_RPS_ACCESS_COPY_DEST_BIT;

        /// Accessible as a resolve source.
        const RESOLVE_SRC = ffi::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_SRC_BIT;

        /// Accessible as a resolve target.
        const RESOLVE_DEST = ffi::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_DEST_BIT;

        /// Accessible for write (build) as a raytracing acceleration structure.
        const RAYTRACING_AS_BUILD = ffi::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_BUILD_BIT;

        /// Accessible for read as a raytracing acceleration structure.
        const RAYTRACING_AS_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_READ_BIT;

        /// Accessible as a present source.
        const PRESENT = ffi::RpsAccessFlagBits_RPS_ACCESS_PRESENT_BIT;

        /// Accessible for reads by the CPU.
        const CPU_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_CPU_READ_BIT;

        /// Accessible for writes by the CPU.
        const CPU_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_CPU_WRITE_BIT;

        // Additional decorator flags not for standalone use but instead for combination with basic access flags.

        /// Access can be used by a render pass attachment (as render target or depth stencil). Used to distinguish clear-only
        /// accesses (which may use special clear commands) and render target / depth stencil view accesses.
        const RENDER_PASS = ffi::RpsAccessFlagBits_RPS_ACCESS_RENDER_PASS;

        /// Initial state when entering the node. This allows a view to have a different state at entering and exiting,
        /// in case the node implementation needs to perform a transition but does not want to transition it back to the
        /// original state. Not implemented yet.
        const BEFORE = ffi::RpsAccessFlagBits_RPS_ACCESS_BEFORE_BIT;

        /// Final state when exiting the node. This allows a view to have a different state at entering and exiting,
        /// in case the node implementation needs to perform a transition but does not want to transition it back to the
        /// original state. Not implemented yet.
        const AFTER = ffi::RpsAccessFlagBits_RPS_ACCESS_AFTER_BIT;

        /// View is cleared before the current access. Usually used together with other basic access flags.
        const CLEAR = ffi::RpsAccessFlagBits_RPS_ACCESS_CLEAR_BIT;

        /// Access does not read existing data so it can be discarded.
        const DISCARD_OLD_DATA = ffi::RpsAccessFlagBits_RPS_ACCESS_DISCARD_OLD_DATA_BIT;

        /// Access does not care about the ordering with regard to other accesses which also have the
        /// RPS_ACCESS_RELAXED_ORDER_BIT flag.
        const RELAXED_ORDER = ffi::RpsAccessFlagBits_RPS_ACCESS_RELAXED_ORDER_BIT;

        /// Access does not need a resource view to be created, (e.g. via `ID3D12GraphicsCommandList::CopyResource`).
        const NO_VIEW = ffi::RpsAccessFlagBits_RPS_ACCESS_NO_VIEW_BIT;

        // Aliases

        /// Accessible as a predication buffer.
        const PREDICATION = ffi::RpsAccessFlagBits_RPS_ACCESS_PREDICATION_BIT;

        /// Depth read write access.
        const DEPTH = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH;

        /// Stencil read write access.
        const STENCIL = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL;

        /// Depth / Stencil read access.
        const DEPTH_STENCIL_READ= ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_READ;

        /// Depth / Stencil write access.
        const DEPTH_STENCIL_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_WRITE;

        /// Depth / Stencil read write access.
        const DEPTH_STENCIL = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL;

        /// Bitwise OR of all possible GPU writeable access flags.
        const ALL_GPU_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_WRITE;

        /// Bitwise OR of all possible GPU readonly access flags.
        const ALL_GPU_READONLY = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_READONLY;

        /// Bitwise OR of all possible GPU access flags.
        const ALL_GPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU;

        /// Bitwise OR of all possible CPU access flags.
        const ALL_CPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_CPU;

        /// Bitwise OR of all GPU / CPU access, excluding decorator flags such as RPS_ACCESS_RELAXED_ORDER_BIT and RPS_ACCESS_NO_VIEW_BIT.
        const ALL_ACCESS_MASK = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_ACCESS_MASK;
    }
}

bitflags! {
    /// Bitflags for shader stages.
    pub struct ShaderStage: u32 {
        /// Vertex shader stage.
        const VS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_VS;
        /// Pixel shader stage.
        const PS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_PS;
        /// Geometry shader stage.
        const GS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_GS;
        /// Compute shader stage.
        const CS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_CS;
        /// Hull shader stage.
        const HS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_HS;
        /// Domain shader stage.
        const DS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_DS;
        /// Raytracing shader stage.
        const RAYTRACING = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_RAYTRACING;
        /// Amplification shader stage.
        const AS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_AS;
        /// Mesh shader stage.
        const MS = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_MS;
        /// All shader stages.
        const ALL = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_ALL;
    }
}

/// Graphics resource and argument data usage semantics.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Semantic {
    /// No semantics.
    Unspecified = ffi::RpsSemantic_RPS_SEMANTIC_UNSPECIFIED,

    // Shaders:
    /// Reserved for future use.
    VertexShader = ffi::RpsSemantic_RPS_SEMANTIC_VERTEX_SHADER,
    /// Reserved for future use.
    PixelShader = ffi::RpsSemantic_RPS_SEMANTIC_PIXEL_SHADER,
    /// Reserved for future use.
    GeometryShader = ffi::RpsSemantic_RPS_SEMANTIC_GEOMETRY_SHADER,
    /// Reserved for future use.
    ComputeShader = ffi::RpsSemantic_RPS_SEMANTIC_COMPUTE_SHADER,
    /// Reserved for future use.
    HullShader = ffi::RpsSemantic_RPS_SEMANTIC_HULL_SHADER,
    /// Reserved for future use.
    DomainShader = ffi::RpsSemantic_RPS_SEMANTIC_DOMAIN_SHADER,
    /// Reserved for future use.
    RaytracingPipeline = ffi::RpsSemantic_RPS_SEMANTIC_RAYTRACING_PIPELINE,
    /// Reserved for future use.
    AmplificationShader = ffi::RpsSemantic_RPS_SEMANTIC_AMPLIFICATION_SHADER,
    /// Reserved for future use.
    MeshShader = ffi::RpsSemantic_RPS_SEMANTIC_MESH_SHADER,

    // States:
    /// Reserved for future use.
    VertexLayout = ffi::RpsSemantic_RPS_SEMANTIC_VERTEX_LAYOUT,
    /// Reserved for future use.
    StreamOutLayout = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_LAYOUT,
    /// Reserved for future use.
    StreamOutDesc = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_DESC,
    /// Reserved for future use.
    BlendState = ffi::RpsSemantic_RPS_SEMANTIC_BLEND_STATE,
    /// Reserved for future use.
    RenderTargetBlend = ffi::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET_BLEND,
    /// Reserved for future use.
    DepthStencilState = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_STATE,
    /// Reserved for future use.
    RasterizerState = ffi::RpsSemantic_RPS_SEMANTIC_RASTERIZER_STATE,

    /// Usage as a viewport.
    /// The data type must be [Viewport].
    Viewport = ffi::RpsSemantic_RPS_SEMANTIC_VIEWPORT,

    /// Usage as a scissor rectangle.
    /// The data type must be [Rect].
    Scissor = ffi::RpsSemantic_RPS_SEMANTIC_SCISSOR,

    /// Usage as primitive topology.
    /// The data must be one of the values specified by [PrimitiveTopology].
    PrimitiveTopology = ffi::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_TOPOLOGY,

    /// Reserved for future use.
    PatchControlPoints = ffi::RpsSemantic_RPS_SEMANTIC_PATCH_CONTROL_POINTS,

    /// Reserved for future use.
    PrimitiveStripCutIndex = ffi::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_STRIP_CUT_INDEX,

    /// Reserved for future use.
    BlendFactor = ffi::RpsSemantic_RPS_SEMANTIC_BLEND_FACTOR,

    /// Reserved for future use.
    StencilRef = ffi::RpsSemantic_RPS_SEMANTIC_STENCIL_REF,

    /// Reserved for future use.
    DepthBounds = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_BOUNDS,

    /// Reserved for future use.
    SampleLocation = ffi::RpsSemantic_RPS_SEMANTIC_SAMPLE_LOCATION,

    /// Reserved for future use.
    ShadingRate = ffi::RpsSemantic_RPS_SEMANTIC_SHADING_RATE,

    /// Usage as a color clear value. The data type must be float[4].
    ColorClearValue = ffi::RpsSemantic_RPS_SEMANTIC_COLOR_CLEAR_VALUE,

    /// Usage as a depth clear value. The data type must be float.
    DepthClearValue = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_CLEAR_VALUE,

    /// Usage as a stencil clear value. The data type must be uint32_t, only the lower 8 bit will be used.
    StencilClearValue = ffi::RpsSemantic_RPS_SEMANTIC_STENCIL_CLEAR_VALUE,

    // Resource bindings:
    /// Bound as a vertex buffer. The semantic index indicates the vertex buffer binding slot.
    VertexBuffer = ffi::RpsSemantic_RPS_SEMANTIC_VERTEX_BUFFER,

    /// Bound as an index buffer.
    IndexBuffer = ffi::RpsSemantic_RPS_SEMANTIC_INDEX_BUFFER,

    /// Bound as an indirect argument buffer.
    IndirectArgs = ffi::RpsSemantic_RPS_SEMANTIC_INDIRECT_ARGS,

    /// Bound as an indirect count buffer.
    StreamOutBuffer = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_BUFFER,

    /// Bound for write as a stream out buffer. The semantic index indicates the stream out buffer binding slot.
    IndirectCount = ffi::RpsSemantic_RPS_SEMANTIC_INDIRECT_COUNT,

    /// Bound as a render target view. The semantic index indicates the render target slot.
    RenderTarget = ffi::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET,

    /// Bound as a depth stencil view.
    DepthStencilTarget = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_TARGET,

    /// Bound as a shading rate image in a Variable Rate Shading (VRS) pass.
    ShadingRateImage = ffi::RpsSemantic_RPS_SEMANTIC_SHADING_RATE_IMAGE,

    /// Bound as a resolve target. The semantic index indicates the render
    /// target slot of the resolve source.
    ResolveTarget = ffi::RpsSemantic_RPS_SEMANTIC_RESOLVE_TARGET,

    /// User defined resource view binding. This is intended for shader resource views and unordered access views where
    /// resources are bound to programmable shaders instead of fixed function binding points.
    UserResourceBinding = ffi::RpsSemantic_RPS_SEMANTIC_USER_RESOURCE_BINDING,
}

impl Semantic {
    /// Number of defined semantics.
    pub const COUNT: usize = ffi::RpsSemantic_RPS_SEMANTIC_COUNT as usize;

    /// Start of the dynamic state semantic enumeration values.
    pub const DYNAMIC_STATE_BEGIN: Self = Self::Viewport;

    /// Start of the resource binding enumeration values.
    pub const RESOURCE_BINDING_BEGIN: Self = Self::VertexBuffer;
}

/// Supported RPS formats.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Format {
    /// Unknown format.
    Unknown = ffi::RpsFormat_RPS_FORMAT_UNKNOWN,
    /// 4-channel RGBA format with each channel being a typeless 32-bit value.
    R32G32B32A32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_TYPELESS,
    /// 4-channel RGBA format with each channel being a 32-bit IEEE 754 floating point value.
    R32G32B32A32Float = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_FLOAT,
    /// 4-channel RGBA format with each channel being a 32-bit unsigned integer.
    R32G32B32A32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_UINT,
    /// 4-channel RGBA format with each channel being a 32-bit signed integer.
    R32G32B32A32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_SINT,
    /// 3-channel RGB format with each channel being a typeless 32-bit value.
    R32G32B32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32B32_TYPELESS,
    /// 3-channel RGB format with each channel being a 32-bit IEEE 754 floating point value.
    R32G32B32Float = ffi::RpsFormat_RPS_FORMAT_R32G32B32_FLOAT,
    /// 3-channel RGB format with each channel being a 32-bit unsigned integer.
    R32G32B32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32B32_UINT,
    /// 3-channel RGB format with each channel being a 32-bit signed integer.
    R32G32B32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32B32_SINT,

    /// 4-channel RGBA format with each channel being a typeless 16-bit value.
    R16G16B16A16Typeless = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_TYPELESS,
    /// 4-channel RGBA format with each channel being a 16-bit floating point value.
    R16G16B16A16Float = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_FLOAT,
    /// 4-channel RGBA format with each channel being a normalized, 16-bit unsigned integer.
    R16G16B16A16Unorm = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UNORM,
    /// 4-channel RGBA format with each channel being a 16-bit unsigned integer.
    R16G16B16A16Uint = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UINT,
    /// 4-channel RGBA format with each channel being a normalized, 16-bit signed integer.
    R16G16B16A16Snorm = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SNORM,

    /// 4-channel RGBA format with each channel being a 16-bit signed integer.
    R16G16B16A16Sint = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SINT,
    /// 2-channel RG format with each channel being a typeless 32-bit value.
    R32G32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32_TYPELESS,
    /// 2-channel RG format with each channel being a 32-bit IEEE 754 floating point value.
    R32G32Float = ffi::RpsFormat_RPS_FORMAT_R32G32_FLOAT,
    /// 2-channel RG format with each channel being a 32-bit unsigned integer.
    R32G32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32_UINT,
    /// 2-channel RG format with each channel being a 32-bit signed integer.
    R32G32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32_SINT,

    /// 2-channel RG format with the first channel being a typeless 32-bit value, the second channel a typeless 8-bit
    /// value and 24 unused bits at the end.
    R32G8X24Typeless = ffi::RpsFormat_RPS_FORMAT_R32G8X24_TYPELESS,

    /// 2-channel RG format with the first channel being a 32-bit depth value, the second one a 8-bit unsigned integer
    /// value and 24 unused bits at the end.
    D32FloatS8X24Uint = ffi::RpsFormat_RPS_FORMAT_D32_FLOAT_S8X24_UINT,

    /// Single channel R format with the channel being a typeless 32-bit IEEE 754 floating point value and additional
    /// sets of 8 and 24 unused bits afterwards.
    R32FloatX8X24Typeless = ffi::RpsFormat_RPS_FORMAT_R32_FLOAT_X8X24_TYPELESS,

    /// Single channel R format with 32 unused bits, the channel being an 8-bit unsigned integer value and 24 unused
    /// bits at the end.
    X32TypelessG8X24Uint = ffi::RpsFormat_RPS_FORMAT_X32_TYPELESS_G8X24_UINT,

    /// 4-channel RGBA format with the RGB channels being typeless 10-bit values and the A channel being a typeless
    /// 2-bit value.
    R10G10B10A2Typeless = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_TYPELESS,

    /// 4-channel RGBA format with the RGB channels being 10-bit normalized, unsigned integer values and the A channel
    /// being a 2-bit normalized, unsigned integer value.
    R10G10B10A2Unorm = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UNORM,

    /// 4-channel RGBA format with the RGB channels being 10-bit unsigned integer values and the A channel being a 2-bit
    /// unsigned integer value.
    R10G10B10A2Uint = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UINT,

    /// 3-channel RGB format with the RG channels being 11-bit floating point values and the B channel being a 10-bit
    /// floating point value.
    R11G11B10Float = ffi::RpsFormat_RPS_FORMAT_R11G11B10_FLOAT,

    /// 4-channel RGBA format with all channels being typeless 8-bit values.
    R8G8B8A8Typeless = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_TYPELESS,

    /// 4-channel RGBA format with all channels being normalized 8-bit unsigned integers.
    R8G8B8A8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM,
    /// 4-channel RGBA format with all channels being normalized 8-bit unsigned integer SRGB values.
    R8G8B8A8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM_SRGB,
    /// 4-channel RGBA format with all channels being 8-bit unsigned integers.
    R8G8B8A8Uint = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UINT,
    /// 4-channel RGBA format with all channels being normalized, 8-bit signed integers.
    R8G8B8A8Snorm = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SNORM,
    /// 4-channel RGBA format with all channels being 8-bit signed integers.
    R8G8B8A8Sint = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SINT,

    /// 2-channel RG format with each channel being a typeless 16-bit value.
    R16G16Typeless = ffi::RpsFormat_RPS_FORMAT_R16G16_TYPELESS,
    /// 2-channel RG format with each channel being a 16-bit IEEE 754 floating point value.
    R16G16Float = ffi::RpsFormat_RPS_FORMAT_R16G16_FLOAT,
    /// 2-channel RG format with each channel being a normalized, 16-bit unsigned integer.
    R16G16Unorm = ffi::RpsFormat_RPS_FORMAT_R16G16_UNORM,
    /// 2-channel RG format with each channel being a 16-bit unsigned integer.
    R16G16Uint = ffi::RpsFormat_RPS_FORMAT_R16G16_UINT,
    /// 2-channel RG format with each channel being a normalized, 16-bit signed integer value.
    R16G16Snorm = ffi::RpsFormat_RPS_FORMAT_R16G16_SNORM,
    /// 2-channel RG format with each channel being a 16-bit signed integer.
    R16G16Sint = ffi::RpsFormat_RPS_FORMAT_R16G16_SINT,

    /// Single channel R format with the channel being a typeless 32-bit value.
    R32Typeless = ffi::RpsFormat_RPS_FORMAT_R32_TYPELESS,
    /// Single channel R format with the channel being a 32-bit IEEE 754 floating point depth value.
    D32Float = ffi::RpsFormat_RPS_FORMAT_D32_FLOAT,
    /// Single channel R format with the channel being a 32-bit IEEE 754 floating point value.
    R32Float = ffi::RpsFormat_RPS_FORMAT_R32_FLOAT,
    /// Single channel R format with the channel being a 32-bit unsigned integer.
    R32Uint = ffi::RpsFormat_RPS_FORMAT_R32_UINT,
    /// Single channel R format with the channel being a 32-bit signed integer.
    R32Sint = ffi::RpsFormat_RPS_FORMAT_R32_SINT,

    /// 2-channel RG format with the first channel being a typeless 24-bit value and the second one a typeless 8-bit
    /// value.
    R24G8Typeless = ffi::RpsFormat_RPS_FORMAT_R24G8_TYPELESS,

    /// 2-channel RG format with the first channel being a normalized, 24-bit unsigned integer depth value and the
    /// second one an 8-bit unsigned integer stencil value.
    D24UnormS8Uint = ffi::RpsFormat_RPS_FORMAT_D24_UNORM_S8_UINT,

    /// 2-channel RG format with the first channel being a normalized, 24-bit unsigned integer value and the second one
    /// a typeless 8-bit value.
    R24UnormX8Typeless = ffi::RpsFormat_RPS_FORMAT_R24_UNORM_X8_TYPELESS,

    /// Single channel R format with 24 unused bits with the channel being an 8-bit unsigned integer.
    X24TypelessG8Uint = ffi::RpsFormat_RPS_FORMAT_X24_TYPELESS_G8_UINT,

    /// 2-channel RG format with each channel being a typeless 8-bit value.
    R8G8Typeless = ffi::RpsFormat_RPS_FORMAT_R8G8_TYPELESS,
    /// 2-channel RG format with each channel being a normalized, 8-bit unsigned integer.
    R8G8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8_UNORM,
    /// 2-channel RG format with each channel being a 8-bit unsigned integer.
    R8G8Uint = ffi::RpsFormat_RPS_FORMAT_R8G8_UINT,
    /// 2-channel RG format with each channel being a normalized, 8-bit signed integer.
    R8G8Snorm = ffi::RpsFormat_RPS_FORMAT_R8G8_SNORM,
    /// 2-channel RG format with each channel being a 8-bit signed integer.
    R8G8Sint = ffi::RpsFormat_RPS_FORMAT_R8G8_SINT,

    /// Single channel R format with the channel being a typeless 16-bit value.
    R16Typeless = ffi::RpsFormat_RPS_FORMAT_R16_TYPELESS,

    /// Single channel R format with the channel being a 16-bit IEEE 754 floating point value.
    R16Float = ffi::RpsFormat_RPS_FORMAT_R16_FLOAT,

    /// Single channel R format with the channel being a 16-bit IEEE 754 floating point depth value.
    D16Unorm = ffi::RpsFormat_RPS_FORMAT_D16_UNORM,

    /// Single channel R format with the channel being a 16-bit unsigned integer.
    R16Unorm = ffi::RpsFormat_RPS_FORMAT_R16_UNORM,

    /// Single channel R format with the channel being a 16-bit signed integer.
    R16Uint = ffi::RpsFormat_RPS_FORMAT_R16_UINT,

    /// Single channel R format with the channel being a normalized, 16-bit signed integer.
    R16Snorm = ffi::RpsFormat_RPS_FORMAT_R16_SNORM,

    /// Single channel R format with the channel being a 16-bit signed integer.
    R16Sint = ffi::RpsFormat_RPS_FORMAT_R16_SINT,

    /// Single channel R format with the channel being a typeless 8-bit value.
    R8Typeless = ffi::RpsFormat_RPS_FORMAT_R8_TYPELESS,
    /// Single channel R format with the channel being a normalized, 8-bit unsigned integer.
    R8Unorm = ffi::RpsFormat_RPS_FORMAT_R8_UNORM,
    /// Single channel R format with the channel being a 8-bit signed integer.
    R8Uint = ffi::RpsFormat_RPS_FORMAT_R8_UINT,
    /// Single channel R format with the channel being a normalized, 8-bit signed integer.
    R8Snorm = ffi::RpsFormat_RPS_FORMAT_R8_SNORM,
    /// Single channel R format with the channel being a 8-bit signed integer.
    R8Sint = ffi::RpsFormat_RPS_FORMAT_R8_SINT,
    /// Single channel A format with the channel being a normalized, 8-bit unsigned integer.
    A8Unorm = ffi::RpsFormat_RPS_FORMAT_A8_UNORM,
    /// Single channel R format with the channel being a 1-bit unsigned integer.
    R1Unorm = ffi::RpsFormat_RPS_FORMAT_R1_UNORM,

    /// 4-channel RGB format with the first three channels being a 9-bit mantissa. Together with the 5-bit exponent that
    /// is shared for all three channels they form three 9-bit mantissa + 5-bit exponent floating point value.
    R9G9B9E5Sharedexp = ffi::RpsFormat_RPS_FORMAT_R9G9B9E5_SHAREDEXP,

    /// 4-channel RGB format with each channel being a normalized, 8-bit unsigned integer. Each block of 32 bits
    /// describes the RGB values for a pair of pixels that always share one R and B value but have separate G values.
    R8G8B8G8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8_B8G8_UNORM,

    /// 4-channel RGB format with each channel being a normalized, 8-bit unsigned integer. Each block of 32 bits
    /// describes the RGB values for a pair of pixels that always share one R and B value but have separate G values.
    G8R8G8B8Unorm = ffi::RpsFormat_RPS_FORMAT_G8R8_G8B8_UNORM,

    /// 4-channel block compressed format with the first channel being a typeless 5-bit value, the second one a
    /// typeless, 6-bit value, the third one a typeless, 5-bit value and the last one a typeless, 0-bit or 1-bit value.
    BC1Typeless = ffi::RpsFormat_RPS_FORMAT_BC1_TYPELESS,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer, the second
    /// one a normalized, 6-bit unsigned integer, the third one a normalized, 5-bit unsigned integer and the last one a
    /// normalized, 0-bit or 1-bit unsigned integer.
    BC1Unorm = ffi::RpsFormat_RPS_FORMAT_BC1_UNORM,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer SRGB value,
    /// the second one a normalized, 6-bit unsigned integer SRGB value, the third one a normalized, 5-bit unsigned
    /// integer SRGB valu eand the last one a normalized, 0-bit or 1-bit unsigned integer SRGB value.
    BC1UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC1_UNORM_SRGB,

    /// 4-channel block compressed format with the first channel being a typeless 5-bit value, the second one a
    /// typeless, 6-bit value, the third one a typeless, 5-bit value and the last one a typeless, 4-bit value.
    BC2Typeless = ffi::RpsFormat_RPS_FORMAT_BC2_TYPELESS,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer, the second
    /// one a normalized, 6-bit unsigned integer, the third one a normalized, 5-bit unsigned integer and the last one a
    /// normalized, 4-bit unsigned integer.
    BC2Unorm = ffi::RpsFormat_RPS_FORMAT_BC2_UNORM,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer SRGB value,
    /// the second one a normalized, 6-bit unsigned integer SRGB value, the third one a normalized, 5-bit unsigned
    /// integer SRGB value and the last one a normalized, 4-bit unsigned integer SRGB value.
    BC2UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC2_UNORM_SRGB,

    /// 4-channel block compressed format with the first channel being a typeless 5-bit value, the second one a
    /// typeless, 6-bit value, the third one a typeless, 5-bit value and the last one a typeless, 8-bit value.
    BC3Typeless = ffi::RpsFormat_RPS_FORMAT_BC3_TYPELESS,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer, the second
    /// one a normalized, 6-bit unsigned integer, the third one a normalized, 5-bit unsigned integer and the last one a
    /// normalized, 8-bit unsigned integer.
    BC3Unorm = ffi::RpsFormat_RPS_FORMAT_BC3_UNORM,

    /// 4-channel block compressed format with the first channel being a normalized, 5-bit unsigned integer SRGB value,
    /// the second one a normalized, 6-bit unsigned integer SRGB value, the third one a normalized, 5-bit unsigned
    /// integer SRGB value and the last one a normalized, 0-bit or 1-bit unsigned integer SRGB value.
    BC3UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC3_UNORM_SRGB,

    /// Single channel block compressed format with the channel being a typeless 8-bit value.
    BC4Typeless = ffi::RpsFormat_RPS_FORMAT_BC4_TYPELESS,

    /// Single channel block compressed format with the channel being a normalized, 8-bit signed integer value.
    BC4Unorm = ffi::RpsFormat_RPS_FORMAT_BC4_UNORM,

    /// Single channel block compressed format with the channel being a normalized, 8-bit signed integer value.
    BC4Snorm = ffi::RpsFormat_RPS_FORMAT_BC4_SNORM,

    /// 2-channel block compressed format with each channel being a typeless 8-bit value.
    BC5Typeless = ffi::RpsFormat_RPS_FORMAT_BC5_TYPELESS,

    /// 2-channel block compressed format with each channel being a normalized, 8-bit unsigned integer value.
    BC5Unorm = ffi::RpsFormat_RPS_FORMAT_BC5_UNORM,

    /// 2-channel block compressed format with each channel being a normalized, 8-bit signed integer value.
    BC5Snorm = ffi::RpsFormat_RPS_FORMAT_BC5_SNORM,

    /// 3-channel BGR format with the first channel being a normalized, 5-bit unsigned integer, the second one a
    /// normalized, 6-bit unsigned integer and the third one a normalized, 5-bit unsigned integer.
    B5G6R5Unorm = ffi::RpsFormat_RPS_FORMAT_B5G6R5_UNORM,

    /// 4-channel BGRA format with the first three channels being a normalized, 5-bit unsigned integer and the last one
    /// a normalized, 1-bit unsigned integer.
    B5G5R5A1Unorm = ffi::RpsFormat_RPS_FORMAT_B5G5R5A1_UNORM,

    /// 4-channel BGRA format with each channel being a normalized, 8-bit unsigned integer.
    B8G8R8A8Unorm = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM,

    /// 3-channel BGR format with each channel being a normalized, 8-bit unsigned integer value and 8 unused bits at the
    /// end.
    B8G8R8X8Unorm = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM,

    /// 4-channel RGB 2.8-biased fixed-point format with the first three channels being a normalized, 10-bit
    /// unsigned integer and the last one a normalized 2-bit unsigned integer.
    R10G10B10XrBiasA2Unorm = ffi::RpsFormat_RPS_FORMAT_R10G10B10_XR_BIAS_A2_UNORM,

    /// 4-channel BGRA format with each channel being a typeless 8-bit value.
    B8G8R8A8Typeless = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_TYPELESS,
    /// 4-channel BGRA format with each channel being a normalized, 8-bit unsigned integer SRGB value.
    B8G8R8A8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM_SRGB,
    /// 3-channel BGR format with each channel being a typeless 8-bit value and 8 unused bits at the end.
    B8G8R8X8Typeless = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_TYPELESS,
    /// 3-channel BGR format with each channel being a normalized, 8-bit unsigned integer and 8 unused bits a the end.
    B8G8R8X8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM_SRGB,

    /// 3-channel block compressed HDR format with each channel being a typeless 16-bit value.
    BC6HTypeless = ffi::RpsFormat_RPS_FORMAT_BC6H_TYPELESS,

    /// 3-channel block compressed HDR format with each channel being a 16-bit unsigned "half" floating point value.
    BC6HUF16 = ffi::RpsFormat_RPS_FORMAT_BC6H_UF16,

    /// 3-channel block compressed HDR format with each channel being a 16-bit signed "half" floating point value.
    BC6HSF16 = ffi::RpsFormat_RPS_FORMAT_BC6H_SF16,

    /// 3-channel or 4-channel block compressed format with the first three channels being a typeless, 4-7-bit value and
    /// the last one an optional, typeless 0-8-bit value.
    BC7Typeless = ffi::RpsFormat_RPS_FORMAT_BC7_TYPELESS,

    /// 3-channel or 4-channel block compressed format with the first three channels being an normalized, 4-7-bit
    /// unsigned integer and the last one an optional, normalized, 0-8-bit unsigned integer.
    BC7Unorm = ffi::RpsFormat_RPS_FORMAT_BC7_UNORM,

    /// 3-channel or 4-channel block compressed format with the first three channels being an normalized, 4-7-bit
    /// unsigned integer and the last one an optional, normalized, 0-8-bit unsigned integer .
    BC7UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC7_UNORM_SRGB,

    /// 4-channel video resource format with each channel being a 8-bit value.
    AYUV = ffi::RpsFormat_RPS_FORMAT_AYUV,

    /// 4-channel video resource format with each of the first three channels being a
    /// 10-bit value and the last one a 2-bit value.
    Y410 = ffi::RpsFormat_RPS_FORMAT_Y410,

    /// 4-channel video resource format with each channel being a 16-bit value.
    Y416 = ffi::RpsFormat_RPS_FORMAT_Y416,
    /// 2-channel video resource format with each channel being a 8-bit value.
    NV12 = ffi::RpsFormat_RPS_FORMAT_NV12,
    /// 2-channel video resource format with each channel being a 16-bit value.
    P010 = ffi::RpsFormat_RPS_FORMAT_P010,
    /// 2-channel video resource format with each channel being a 8-bit value.
    P016 = ffi::RpsFormat_RPS_FORMAT_P016,
    /// Video resource format with opaque layout.
    Opaque420 = ffi::RpsFormat_RPS_FORMAT_420_OPAQUE,
    /// 4-channel video resource format with each channel being a 8-bit value.
    YUY2 = ffi::RpsFormat_RPS_FORMAT_YUY2,
    /// 4-channel video resource format with each channel being a 16-bit value.
    Y210 = ffi::RpsFormat_RPS_FORMAT_Y210,
    /// 4-channel video resource format with each channel being a 16-bit value.
    Y216 = ffi::RpsFormat_RPS_FORMAT_Y216,
    /// 2-channel video resource format with each channel being a 8-bit value.
    NV11 = ffi::RpsFormat_RPS_FORMAT_NV11,
    /// 4-bit palletized video resource format.
    AI44 = ffi::RpsFormat_RPS_FORMAT_AI44,
    /// 4-bit palletized video resource format.
    IA44 = ffi::RpsFormat_RPS_FORMAT_IA44,
    /// RGB video resource format with 8-bit palletization.
    P8 = ffi::RpsFormat_RPS_FORMAT_P8,
    /// RGB video resource format with 8-bit palletization.
    A8P8 = ffi::RpsFormat_RPS_FORMAT_A8P8,

    /// 4-channels BGRA format with each channel being a normalized 4-bit unsigned integer.
    B4G4R4A4Unorm = ffi::RpsFormat_RPS_FORMAT_B4G4R4A4_UNORM,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = unsafe {
            let ptr = ffi::rpsFormatGetName(*self as ffi::RpsFormat);
            CStr::from_ptr(ptr)
        };

        write!(f, "{:?}", str)
    }
}

impl Format {
    /// Number of formats available.
    pub const COUNT: usize = ffi::RpsFormat_RPS_FORMAT_COUNT as usize;

    /// Returns whether a format is block compressed.
    #[inline]
    pub fn is_block_compressed(self) -> bool {
        unsafe { ffi::rpsFormatIsBlockCompressed(self as ffi::RpsFormat) != 0 }
    }

    /// Returns whether a format has a depth or a stencil component.
    #[inline]
    pub fn has_depth_stencil(self) -> bool {
        unsafe { ffi::rpsFormatHasDepthStencil(self as ffi::RpsFormat) != 0 }
    }

    /// Returns whether a format has a depth component.
    #[inline]
    pub fn has_depth(self) -> bool {
        unsafe { ffi::rpsFormatHasDepth(self as ffi::RpsFormat) != 0 }
    }

    /// Returns whether a format has a stencil component.
    #[inline]
    pub fn has_stencil(self) -> bool {
        unsafe { ffi::rpsFormatHasStencil(self as ffi::RpsFormat) != 0 }
    }

    /// Returns whether a format has only a depth component and no stencil component.
    #[inline]
    pub fn is_depth_only(self) -> bool {
        unsafe { ffi::rpsFormatIsDepthOnly(self as ffi::RpsFormat) != 0 }
    }

    /// Returns the single element byte size for a format.
    ///
    /// For most formats one element is one pixel. This is different for block compressed formats, e.g.
    /// RPS_FORMAT_BC1_UNORM. The byte size of one block will be returned for these instead.
    ///
    /// # Returns
    /// 0 if the format does not support element wise usage, size of bytes of a single element otherwise.
    #[inline]
    pub fn element_bytes(self) -> u32 {
        unsafe { ffi::rpsGetFormatElementBytes(self as ffi::RpsFormat) }
    }
}
