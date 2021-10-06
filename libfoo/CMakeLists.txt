cmake_minimum_required(VERSION 3.1)

project(foo)

set(CMAKE_CXX_STANDARD 17)

add_library(foo
    src/foo.cpp
)

target_include_directories(foo PUBLIC include/)

get_target_property(INCL_DIR_TO_INSTALL foo INTERFACE_INCLUDE_DIRECTORIES)
install(TARGETS foo)
install(DIRECTORY ${INCL_DIR_TO_INSTALL} TYPE INCLUDE) 
