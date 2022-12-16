use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .clang_arg("-I./RenderPipelineShaders/include")
        .header("RenderPipelineShaders/include/rps/rps.h")
        .rustfmt_bindings(true)
        .allowlist_function("rps.*")
        .allowlist_function("PFN_rps.*")
        .allowlist_type("Rps.*")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings file");

    let mut build = cc::Build::new();

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

    build.compile("rps");
}
