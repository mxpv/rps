//! Runtime API wrappers.

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
        const RPS_ACCESS_ALL_GPU_READONLY = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_READONLY;

        /// Bitwise OR of all possible GPU access flags.
        const RPS_ACCESS_ALL_GPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU;

        /// Bitwise OR of all possible CPU access flags.
        const RPS_ACCESS_ALL_CPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_CPU;

        /// Bitwise OR of all GPU / CPU access, excluding decorator flags such as RPS_ACCESS_RELAXED_ORDER_BIT and RPS_ACCESS_NO_VIEW_BIT.
        const RPS_ACCESS_ALL_ACCESS_MASK = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_ACCESS_MASK;
    }
}
