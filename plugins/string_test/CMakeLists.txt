# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2019-2022 Second State INC

add_library(GetString
  SHARED
  stringtest.cpp
)

target_compile_options(GetString
  PUBLIC
  -DWASMEDGE_PLUGIN
)

if(CMAKE_SYSTEM_NAME MATCHES "Darwin")
  target_link_options(GetString
    PUBLIC
    -Wl,-U,__ZN8WasmEdge6Plugin14PluginRegisterC1EPKNS0_6Plugin16PluginDescriptorE
    -Wl,-U,__ZN8WasmEdge6Plugin14PluginRegisterD1Ev
  )
endif()

target_include_directories(GetString
  PUBLIC
  $<TARGET_PROPERTY:wasmedgePlugin,INCLUDE_DIRECTORIES>
)

target_link_libraries(GetString
  PUBLIC
  wasmedgeCommon
  wasmedgeSystem
)

if(WASMEDGE_LINK_PLUGINS_STATIC)
  target_link_libraries(GetString
    PRIVATE
    wasmedgeCAPI
  )
else()
  target_link_libraries(GetString
    PRIVATE
    wasmedge_shared
  )
endif()

install(TARGETS GetString DESTINATION ${CMAKE_INSTALL_LIBDIR}/wasmedge)

message(STATUS "Build GetString Plugin")
find_package( PkgConfig REQUIRED)
pkg_search_module( TESSERACT REQUIRED tesseract )
pkg_search_module( LEPTONICA REQUIRED lept )

target_include_directories(GetString
  PUBLIC
  ${TESSERACT_INCLUDE_DIRS}
  ${LEPTONICA_INCLUDE_DIRS}
)

target_link_libraries(GetString
  PUBLIC
  ${TESSERACT_LIBRARIES}
  ${LEPTONICA_LIBRARIES}
)
