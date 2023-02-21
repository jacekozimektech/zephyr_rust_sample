# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(hello_world_rust)

# Add external Rust lib directory
set(zephyr_rust_lib_dir   ${CMAKE_CURRENT_SOURCE_DIR}/target/thumbv7m-none-eabi/release/)
set(zephyr_rust_lib_include_dir   ${CMAKE_CURRENT_SOURCE_DIR}/generated)

# Add the library
add_library(zephyr_rust_lib STATIC IMPORTED GLOBAL)

# Set the paths
set_target_properties(zephyr_rust_lib PROPERTIES IMPORTED_LOCATION             ${zephyr_rust_lib_dir}/libzephyr_rust_sample.a)
set_target_properties(zephyr_rust_lib PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${zephyr_rust_lib_include_dir})

# Link them!
target_link_libraries(app PUBLIC zephyr_rust_lib -Wl,--allow-multiple-definition)

target_sources(app PRIVATE src/main.c)
