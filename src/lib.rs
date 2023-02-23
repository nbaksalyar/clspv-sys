use std::ffi::{c_char, c_int, c_void, CStr, CString};

/// Opaque result structure.
#[allow(non_camel_case_types)]
type clspv_result = c_void;

extern "C" {
    /// Allocates memory to hold the result of compilation.
    /// `clspv_result_free` *must* be called after you have used the compilation result.
    fn clspv_result_alloc() -> *mut clspv_result;

    /// Frees memory allocated for the compilation result.
    fn clspv_result_free(res: *mut clspv_result);

    /// Returns the compiler output.
    ///
    /// ## Arguments
    /// - `output` should be a valid pointer to a variable that will be populated with the address of the output value bytes.
    /// - `output_size` should be a valid pointer to a `size_t` integer that will hold the output value size.
    fn clspv_result_get_output(
        res: *const clspv_result,
        output: *mut *const u32,
        output_size: *mut usize,
    );

    /// Returns the compiler output log.
    ///
    /// ## Arguments
    /// - `output` should be a valid pointer to a variable that will be populated with the address of the output log.
    /// - `output_size` should be a valid pointer to a `size_t` integer that will hold the output value size in bytes.
    fn clspv_result_get_output_log(
        res: *const clspv_result,
        output: *mut *const c_char,
        output_size: *mut usize,
    );

    /// Compile from a source string.
    ///
    /// For use with clBuildProgram. The input program is passed as `program`.
    /// Command line options to clspv are passed as `options`.
    fn compile_from_source_string(
        program: *const c_char,
        options: *const c_char,
        result: *mut clspv_result,
    ) -> c_int;
}

/// Output from the compiler.
pub struct Output {
    /// Exit code returned by the compiler.
    pub ret_code: c_int,
    /// Compiler output (SPIR-V object code, LLVM IR, etc.)
    pub output: Vec<u32>,
    /// Compiler output log.
    pub log: String,
}

/// Compiles the OpenCL C `program` into a specified target.
pub fn compile_from_source(program: &str, options: &str) -> Output {
    let c_program = CString::new(program).unwrap();
    let c_options = CString::new(options).unwrap();

    unsafe {
        let result = clspv_result_alloc();

        let ret_code = compile_from_source_string(c_program.as_ptr(), c_options.as_ptr(), result);

        let mut out_ptr: *const u32 = std::ptr::null();
        let mut out_size = 0usize;

        let mut out_log_ptr: *const c_char = std::ptr::null();
        let mut out_log_size = 0usize;

        clspv_result_get_output(result, &mut out_ptr, &mut out_size);
        clspv_result_get_output_log(result, &mut out_log_ptr, &mut out_log_size);

        let output = std::slice::from_raw_parts(out_ptr, out_size).to_owned();
        let log = CStr::from_ptr(out_log_ptr)
            .to_owned()
            .into_string()
            .unwrap();

        clspv_result_free(result);

        Output {
            ret_code,
            output,
            log,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::compile_from_source;

    #[test]
    fn smoke_test() {
        let output = compile_from_source("bla bla bla", "");
        assert_eq!(output.ret_code, -1);
    }
}
