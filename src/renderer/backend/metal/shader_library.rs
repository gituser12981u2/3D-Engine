use log::{debug, info, trace};
use metal::{CompileOptions, Device, Function, Library, MTLDataType};
use std::{os::raw::c_void, path::Path};

use crate::RendererError;

/// Handles loading and management of Metal shader libraries
pub struct ShaderLibrary {
    library: Library,
}

/// Configuration options for shader loading
#[derive(Clone, Debug, Default)]
pub struct ShaderLoadOptions {
    /// Custom path to metallib file
    pub shader_path: Option<String>,
    /// Whether to use runtime compilation instead of pre-compiled shaders
    pub use_runtime_compilation: bool,
}

// Include pre-compiled shaders
const COMPILED_SHADERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));

// Include shader source code for runtime compilation
const VERTEX_SHADER_SOURCE: &str = include_str!("../../../metal_shaders/vertex_shader.metal");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../../../metal_shaders/fragment_shader.metal");
const SHADER_TYPES_HEADER: &str = include_str!("../../../metal_shaders/shader_types.h");

impl ShaderLibrary {
    /// Creates a new shader library instance
    pub fn new(device: &Device, options: &ShaderLoadOptions) -> Result<Self, RendererError> {
        debug!("Creating shader library");
        let library = Self::load_library(device, options)?;

        // Verify that required shader functions exist
        if library.get_function("vertex_main", None).is_err()
            || library.get_function("fragment_main", None).is_err()
        {
            return Err(RendererError::ShaderCompilationFailed(
                "Required shader functions not found in library".to_string(),
            ));
        }

        info!("Shader library created successfully");
        Ok(Self { library })
    }

    /// Gets vertex and fragment functions with the given configuration
    pub fn get_shader_functions(
        &self,
        is_instanced: bool,
        use_vertex_color: bool,
    ) -> Result<(Function, Function), RendererError> {
        debug!("Creating shader functions with instanced={is_instanced}, vertex_color={use_vertex_color}");

        // Create function constants for shader compilation
        // These constants are used to configure the shader behavior
        let function_constants = metal::FunctionConstantValues::new();

        // Set function constants for instancing and vertex color usage
        // These values correspond to the function_constant(0) and function_constant(1) in the shader code
        function_constants.set_constant_value_at_index(
            &is_instanced as *const bool as *const c_void,
            MTLDataType::Bool,
            0,
        );

        function_constants.set_constant_value_at_index(
            &use_vertex_color as *const bool as *const c_void,
            MTLDataType::Bool,
            1,
        );

        let vertex_function = self
            .library
            .get_function("vertex_main", Some(function_constants))
            .map_err(|_| RendererError::ShaderFunctionNotFound("vertex_main".to_string()))?;

        let fragment_function = self
            .library
            .get_function("fragment_main", None)
            .map_err(|_| RendererError::ShaderFunctionNotFound("fragment_main".to_string()))?;

        trace!("Shader functions created successfully");
        Ok((vertex_function, fragment_function))
    }

    /// Private method to load the shader library using the provided options
    pub fn load_library(
        device: &Device,
        options: &ShaderLoadOptions,
    ) -> Result<Library, RendererError> {
        // Try user-provided path first
        if let Some(path) = &options.shader_path {
            if Path::new(path).exists() {
                debug!("Loading shader library from custom path: {path}");
                return device.new_library_with_file(path).map_err(|e| {
                    RendererError::ShaderCompilationFailed(format!(
                        "Failed to load shader from path {path}: {e}"
                    ))
                });
            }
        }

        // Try environment variable path
        if let Ok(shader_lib_path) = std::env::var("METAL_SHADER_LIB") {
            if Path::new(&shader_lib_path).exists() {
                debug!("Loading shader library from environment path: {shader_lib_path}");
                return device.new_library_with_file(&shader_lib_path).map_err(|e| {
                    RendererError::ShaderCompilationFailed(format!(
                        "Failed to load shader from env path: {e}"
                    ))
                });
            }
        }

        // Use runtime compilation if requested
        if options.use_runtime_compilation {
            debug!("Compiling shaders at runtime");
            return Self::compile_shaders_at_runtime(device);
        }

        // Fall back to embedded pre-compiled shaders
        debug!("Loading embedded pre-compiled shaders");
        device.new_library_with_data(COMPILED_SHADERS).map_err(|e| {
            RendererError::ShaderCompilationFailed(format!("Failed to load embedded shaders: {e}"))
        })
    }

    /// Compile shaders at runtime from embedded source code
    fn compile_shaders_at_runtime(device: &Device) -> Result<Library, RendererError> {
        let options = CompileOptions::new();

        // Combine shader sources
        let combined_source = format!(
            "{}\n{}\n{}",
            SHADER_TYPES_HEADER, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE
        );

        device
            .new_library_with_source(&combined_source, &options)
            .map_err(|e| {
                RendererError::ShaderCompilationFailed(format!("Runtime compilation failed: {}", e))
            })
    }
}
