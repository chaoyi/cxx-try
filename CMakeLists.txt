# adapted from https://github.com/david-cattermole/cxx-demo-example/
cmake_minimum_required(VERSION 3.20)

project(crash)
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 1)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")

include(FetchContent)
include(GenerateExportHeader)
include(GNUInstallDirs)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG origin/master # Optionally specify a version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_VISIBILITY_INLINES_HIDDEN 1)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_BUILD_TYPE "${CMAKE_BUILD_TYPE}" CACHE STRING "Choose the type of build" FORCE)
if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()
if (WIN32 AND CMAKE_BUILD_TYPE MATCHES "Debug")
  message(FATAL_ERROR "cxxbridge has issue with debug build on windows")
endif()

find_program(CXXBRIDGE cxxbridge REQUIRED PATHS $ENV{HOME}/.cargo/bin)
message(STATUS "Using cxxbridge: ${CXXBRIDGE}")

set(cxxbridge_in ${CMAKE_CURRENT_SOURCE_DIR}/rust/src/lib.rs)
set(cxxbridge_args "--cxx-impl-annotations" "CRASH_EXPORT"  "--include" "rust/cxx.h" "--include" "crash/export.h")
set(cxxbridge_shim_header_out ${CMAKE_CURRENT_BINARY_DIR}/include/rust/cxx.h)
set(cxxbridge_header_out ${CMAKE_CURRENT_BINARY_DIR}/include/crash/cxxbridge.h)
set(cxxbridge_source_out ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge.cpp)
set(export_header_out ${CMAKE_CURRENT_BINARY_DIR}/include/crash/export.h)
set(main_header_out ${CMAKE_CURRENT_BINARY_DIR}/include/crash.h)

add_custom_command(
  OUTPUT ${cxxbridge_shim_header_out} ${cxxbridge_header_out} ${cxxbridge_source_out}
  COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_CURRENT_BINARY_DIR}/include/rust"
  COMMAND ${CXXBRIDGE} --header --output ${cxxbridge_shim_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --header --output ${cxxbridge_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --output ${cxxbridge_source_out}
  COMMENT "Generating CXX Bridge: ${cxxbridge_header_out} ${cxxbridge_source_out} ${cxxbridge_shim_header_out}"
  DEPENDS ${cxxbridge_in})


set(main_header_content "#include \"rust/cxx.h\"")
set(main_header_content "${main_header_content}\n#include \"crash/cxxbridge.h\"")
set(main_header_content "${main_header_content}\n#include \"crash/export.h\"")
configure_file(crash.h.in ${main_header_out} @ONLY)

set(crash_public_h
  ${cxxbridge_shim_header_out}
  ${cxxbridge_header_out}
  ${export_header_out}
  ${main_header_out})
add_library(crash SHARED ${cxxbridge_source_out} ${crash_public_h})
generate_export_header(crash EXPORT_FILE_NAME ${export_header_out})
target_include_directories(crash PUBLIC ${CMAKE_CURRENT_BINARY_DIR}/include)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/rust/Cargo.toml)
target_link_libraries(crash PUBLIC crash-rs)

add_executable(my_program main.cpp)
target_link_libraries(my_program crash)

install(TARGETS crash
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
install(DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/include/
  DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
  FILES_MATCHING PATTERN "*.h")
