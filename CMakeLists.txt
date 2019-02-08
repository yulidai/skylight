cmake_minimum_required(VERSION 3.12)
project(skylight)

set(CMAKE_CXX_STANDARD 14)

add_executable(skylight
        src/main.rs
        Cargo.toml
        LICENSE
        README.md)
