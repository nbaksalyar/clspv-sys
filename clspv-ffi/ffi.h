#include <stdint.h>

extern "C"
{
    struct clspv_result;

    // Allocates memory to hold the result of compilation.
    // `clspv_result_free` *must* be called after you have used the compilation result.
    clspv_result *clspv_result_alloc();

    // Frees memory allocated for the compilation result.
    void clspv_result_free(clspv_result *res);

    // Returns the compiler output.
    //
    // ## Arguments
    // - `output` should be a valid pointer to a variable that will be populated with the address of the output value bytes.
    // - `output_size` should be a valid pointer to a `size_t` integer that will hold the output value size.
    void clspv_result_get_output(clspv_result *res, uint32_t **output, size_t *output_size);

    // Returns the compiler output log.
    //
    // ## Arguments
    // - `output` should be a valid pointer to a variable that will be populated with the address of the output log.
    // - `output_size` should be a valid pointer to a `size_t` integer that will hold the output value size in bytes.
    void clspv_result_get_output_log(clspv_result *res, const char **output, size_t *output_size);

    // Compile from a source string.
    //
    // For use with clBuildProgram. The input program is passed as `program`.
    // Command line options to clspv are passed as `options`.
    int compile_from_source_string(const char *program,
                                   const char *options,
                                   clspv_result *result);
}
