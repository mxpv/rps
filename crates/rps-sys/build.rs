use std::env;
use std::path::PathBuf;

use bindgen::callbacks;

#[cfg(all(not(target_os = "windows"), any(feature = "d3d11", feature = "d3d12")))]
compile_error!("D3D is not supported on this platform");

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[allow(unused_mut)]
    let mut bindgen = bindgen::builder()
        .clang_arg("-I./RenderPipelineShaders/include")
        .header("RenderPipelineShaders/include/rps/rps.h");

    #[cfg(feature = "d3d12")]
    {
        bindgen = bindgen
            .header("RenderPipelineShaders/include/rps/runtime/d3d_common/rps_d3d_common.h")
            .header("RenderPipelineShaders/include/rps/runtime/d3d12/rps_d3d12_runtime.h");
    }

    #[cfg(feature = "d3d11")]
    {
        bindgen = bindgen
            .header("RenderPipelineShaders/include/rps/runtime/d3d_common/rps_d3d_common.h")
            .header("RenderPipelineShaders/include/rps/runtime/d3d11/rps_d3d11_runtime.h");
    }

    #[cfg(feature = "vk")]
    {
        bindgen = bindgen
            .clang_arg("-I./Vulkan-Headers/include")
            .header("RenderPipelineShaders/include/rps/runtime/vk/rps_vk_runtime.h")
            .header_contents("fixup_vk.h", "#define RPS_IMPL_OPAQUE_HANDLE(x, y, z)")
            .blocklist_type("Vk.*")
            .raw_line("type VkDevice = u64;")
            .raw_line("type VkPhysicalDevice = u64;")
            .raw_line("type VkImage = u64;")
            .raw_line("type VkImageView = u64;")
            .raw_line("type VkImageLayout = u64;")
            .raw_line("type VkBuffer = u64;")
            .raw_line("type VkBufferView = u64;")
            .raw_line("type VkDeviceMemory = u64;")
            .raw_line("type VkRenderPass = u64;")
            .raw_line("type VkFormat = u64;");
    }

    bindgen
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .derive_debug(true)
        .derive_default(true)
        .allowlist_function("rps.*")
        .allowlist_function("PFN_rps.*")
        .allowlist_type("Rps.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(RustDoc))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings file");

    let mut build = cc::Build::new();

    #[cfg(feature = "d3d12")]
    build.define("RPS_D3D12_RUNTIME", "1");

    #[cfg(feature = "d3d11")]
    build.define("RPS_D3D11_RUNTIME", "1");

    #[cfg(feature = "vk")]
    build.define("RPS_VK_RUNTIME", "1");

    build
        .cpp(true)
        .flag("-std=c++14")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-unused-private-field")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-function")
        .include("RenderPipelineShaders/include")
        .include("RenderPipelineShaders/src")
        // Core
        .file("RenderPipelineShaders/src/core/rps_core.cpp")
        .file("RenderPipelineShaders/src/core/rps_device.cpp")
        .file("RenderPipelineShaders/src/core/rps_graph.cpp")
        .file("RenderPipelineShaders/src/core/rps_result.cpp")
        // Frontend
        .file("RenderPipelineShaders/src/frontend/rps_builder.cpp")
        // Runtime
        .file("RenderPipelineShaders/src/runtime/common/rps_access.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_format.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_null_runtime_backend.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_null_runtime_device.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_render_graph.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_render_graph_builder.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_render_graph_diagnostics.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_runtime_backend.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_runtime_device.cpp")
        .file("RenderPipelineShaders/src/runtime/common/rps_subprogram.cpp");

    #[cfg(feature = "d3d12")]
    {
        build
            .file("RenderPipelineShaders/src/runtime/d3d12/rps_d3d12_built_in_nodes.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d12/rps_d3d12_runtime_backend.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d12/rps_d3d12_runtime_backend_debug.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d12/rps_d3d12_runtime_backend_views.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d12/rps_d3d12_runtime_device.cpp");
    }

    #[cfg(feature = "d3d11")]
    {
        build
            .file("RenderPipelineShaders/src/runtime/d3d11/rps_d3d11_built_in_nodes.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d11/rps_d3d11_runtime_backend.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d11/rps_d3d11_runtime_backend_views.cpp")
            .file("RenderPipelineShaders/src/runtime/d3d11/rps_d3d11_runtime_device.cpp");
    }

    #[cfg(feature = "vk")]
    {
        build
            .include("Vulkan-Headers/include")
            .file("RenderPipelineShaders/src/runtime/vk/rps_vk_built_in_nodes.cpp")
            .file("RenderPipelineShaders/src/runtime/vk/rps_vk_formats.cpp")
            .file("RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_backend.cpp")
            .file("RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_device.cpp");
    }

    build.compile("rps");
}

/// Removes `cpp` specific comments for better rustdoc output.
#[derive(Debug)]
struct RustDoc;

impl callbacks::ParseCallbacks for RustDoc {
    fn process_comment(&self, comment: &str) -> Option<String> {
        let mut comment = comment.trim();

        if comment.starts_with("< ") {
            comment = comment.strip_prefix("< ").unwrap_or_default();
        }

        if comment.starts_with("@brief ") {
            comment = comment.strip_prefix("@brief ").unwrap_or_default();
        }

        let comment = comment
            .replace("float[4]", "`float[4]`")
            .replace("<c><i>", "[")
            .replace("</i></c>", "]");

        Some(comment)
    }
}
