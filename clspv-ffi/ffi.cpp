// Provides a C ABI-compatible interface to clspv.

#include <string>
#include <vector>

#include "ffi.h"

#include "clspv/Compiler.h"

struct clspv_result
{
    std::string *output_log_str;
    std::vector<uint32_t> *output;
};

clspv_result *clspv_result_alloc()
{
    clspv_result *res = new clspv_result;
    res->output = new std::vector<uint32_t>();
    res->output_log_str = new std::string();
    return res;
}

void clspv_result_free(clspv_result *res)
{
    delete res->output_log_str;
    delete res->output;
    delete res;
}

void clspv_result_get_output(clspv_result *res, uint32_t **output, size_t *output_size)
{
    *output = &*res->output->begin();
    *output_size = res->output->size();
}

void clspv_result_get_output_log(clspv_result *res, const char **output, size_t *output_size)
{
    *output = &(res->output_log_str->c_str())[0];
    *output_size = res->output_log_str->size();
}

int compile_from_source_string(const char *program,
                               const char *options,
                               clspv_result *result)
{
    return clspv::CompileFromSourceString(std::string(program),
                                          std::string(),
                                          std::string(options),
                                          result->output,
                                          result->output_log_str);
}
