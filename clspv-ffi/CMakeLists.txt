cmake_minimum_required(VERSION 3.13.4)

project(clspv_ffi)

add_subdirectory(clspv)

add_library(clspv_ffi STATIC
    ${CMAKE_CURRENT_SOURCE_DIR}/ffi.cpp
)

target_include_directories(clspv_ffi PRIVATE ${CLSPV_INCLUDE_DIRS})
target_link_libraries(clspv_ffi PRIVATE clspv_core)

install(
    TARGETS clspv_ffi
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
