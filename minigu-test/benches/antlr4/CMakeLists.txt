cmake_minimum_required(VERSION 3.15)
project(gql-antlr4)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_INTERPROCEDURAL_OPTIMIZATION OFF)
set(CMAKE_BUILD_TYPE Release)

set(ANTLR4_JAR_LOCATION ${PROJECT_SOURCE_DIR}/antlr-4.13.2-complete.jar)

find_package(antlr4-generator REQUIRED)
find_package(antlr4-runtime REQUIRED)

antlr4_generate(
   gql_antlr4_parser
   ${PROJECT_SOURCE_DIR}/src/GQL.g4
   BOTH
   FALSE
   FALSE
)

function(bundle_static_library tgt_name bundled_tgt_name)
    list(APPEND static_libs ${tgt_name})

    function(_recursively_collect_dependencies input_target)
        set(_input_link_libraries LINK_LIBRARIES)
        get_target_property(_input_type ${input_target} TYPE)

        if(${_input_type} STREQUAL "INTERFACE_LIBRARY")
            set(_input_link_libraries INTERFACE_LINK_LIBRARIES)
        endif()

        get_target_property(public_dependencies ${input_target} ${_input_link_libraries})

        foreach(dependency IN LISTS public_dependencies)
            if(TARGET ${dependency})
                get_target_property(alias ${dependency} ALIASED_TARGET)

                if(TARGET ${alias})
                    set(dependency ${alias})
                endif()

                get_target_property(_type ${dependency} TYPE)

                if(${_type} STREQUAL "STATIC_LIBRARY")
                    list(APPEND static_libs ${dependency})
                endif()

                get_property(library_already_added
                    GLOBAL PROPERTY _${tgt_name}_static_bundle_${dependency})

                if(NOT library_already_added)
                    set_property(GLOBAL PROPERTY _${tgt_name}_static_bundle_${dependency} ON)
                    _recursively_collect_dependencies(${dependency})
                endif()
            endif()
        endforeach()

        set(static_libs ${static_libs} PARENT_SCOPE)
    endfunction()

    _recursively_collect_dependencies(${tgt_name})

    list(REMOVE_DUPLICATES static_libs)

    set(bundled_tgt_full_name
        ${CMAKE_BINARY_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${bundled_tgt_name}${CMAKE_STATIC_LIBRARY_SUFFIX})

    if(CMAKE_CXX_COMPILER_ID MATCHES "^(Clang|GNU)$")
        if(APPLE)
            find_program(lib_tool libtool)

            foreach(tgt IN LISTS static_libs)
                list(APPEND static_libs_files $<TARGET_FILE:${tgt}>)
            endforeach()

            add_custom_command(
                COMMAND ${lib_tool} -static -o ${bundled_tgt_full_name} ${static_libs_files}
                OUTPUT ${bundled_tgt_full_name}
                COMMENT "Bundling ${bundled_tgt_name}"
                VERBATIM)
        else()
            file(WRITE ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar.in
                "CREATE ${bundled_tgt_full_name}\n")

            foreach(tgt IN LISTS static_libs)
                file(APPEND ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar.in
                    "ADDLIB $<TARGET_FILE:${tgt}>\n")
            endforeach()

            file(APPEND ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar.in "SAVE\n")
            file(APPEND ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar.in "END\n")

            file(GENERATE
                OUTPUT ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar
                INPUT ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar.in)

            set(ar_tool ${CMAKE_AR})

            if(CMAKE_INTERPROCEDURAL_OPTIMIZATION)
                set(ar_tool ${CMAKE_CXX_COMPILER_AR})
            endif()

            add_custom_command(
                COMMAND ${ar_tool} -M < ${CMAKE_BINARY_DIR}/${bundled_tgt_name}.ar
                OUTPUT ${bundled_tgt_full_name}
                COMMENT "Bundling ${bundled_tgt_name}"
                VERBATIM)
        endif()
    elseif(MSVC)
        find_program(lib_tool lib)

        foreach(tgt IN LISTS static_libs)
            list(APPEND static_libs_full_names $<TARGET_FILE:${tgt}>)
        endforeach()

        add_custom_command(
            COMMAND ${lib_tool} /NOLOGO /OUT:${bundled_tgt_full_name} ${static_libs_full_names}
            OUTPUT ${bundled_tgt_full_name}
            COMMENT "Bundling ${bundled_tgt_name}"
            VERBATIM)
    else()
        message(FATAL_ERROR "Unknown bundle scenario!")
    endif()

    add_custom_target(${bundled_tgt_name} ALL DEPENDS ${bundled_tgt_full_name})
    add_dependencies(${bundled_tgt_name} ${tgt_name})
endfunction()

add_library(gql_antlr4 STATIC src/parser.cpp ${ANTLR4_SRC_FILES_gql_antlr4_parser})
target_include_directories(gql_antlr4 PRIVATE ${ANTLR4_INCLUDE_DIR_gql_antlr4_parser})
target_link_libraries(gql_antlr4 PRIVATE antlr4_static)
bundle_static_library(gql_antlr4 gql_antlr4_bundled)