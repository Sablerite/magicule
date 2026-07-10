# --- Auto-detect library source files (everything except main.cpp) ---
file(GLOB_RECURSE NEUTRO_LIB_SOURCES CONFIGURE_DEPENDS
    ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/*.hpp
)
list(FILTER NEUTRO_LIB_SOURCES EXCLUDE REGEX "main\\.cpp$")

# --- Auto-detect C++20 module interface units ---
file(GLOB_RECURSE NEUTRO_MODULES CONFIGURE_DEPENDS
    ${CMAKE_CURRENT_SOURCE_DIR}/*.cppm
    ${CMAKE_CURRENT_SOURCE_DIR}/*.ixx
)

# ---- Static library (all project code, no main) ----
add_library(neutro STATIC ${NEUTRO_LIB_SOURCES})

if(NEUTRO_MODULES)
    target_sources(neutro
        FILE_SET CXX_MODULES
        FILES ${NEUTRO_MODULES}
    )
endif()

target_include_directories(neutro
    PUBLIC ${PROJECT_SOURCE_DIR}/include
)

# ---- Main executable ----
add_executable(neutro_main main.cpp)
target_link_libraries(neutro_main PRIVATE neutro)
target_include_directories(neutro_main
    PRIVATE ${PROJECT_SOURCE_DIR}/include
)