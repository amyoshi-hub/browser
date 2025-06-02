cmake_minimum_required(VERSION 3.10)
project(ImGuiExample)

# C++のバージョン設定
set(CMAKE_CXX_STANDARD 11)

# OSごとの設定
if(WIN32)
    message(STATUS "Configuring for Windows")
    set(GLFW_BUILD_WAYLAND OFF CACHE BOOL "Disable Wayland support")
    set(GLFW_BUILD_X11 OFF CACHE BOOL "Disable X11 support")
    set(GLFW_BUILD_NULL OFF CACHE BOOL "Disable NULL support")
else()
    message(STATUS "Configuring for Linux")
    set(GLFW_BUILD_WAYLAND OFF CACHE BOOL "Enable Wayland support")
    set(GLFW_BUILD_X11 ON CACHE BOOL "Enable X11 support")
    set(GLFW_BUILD_NULL OFF CACHE BOOL "Disable NULL support")
endif()

# HtmlParserライブラリの作成
add_library(HtmlParser STATIC
    cpp_parse/html_parser.cpp
    cpp_parse/html_analysys.cpp
    cpp_parse/html_element.cpp
    cpp_parse/client.c
    cpp_parse/img_recv.c
    cpp_parse/img_req.c
)

# ImGuiの設定
set(IMGUI_DIR imgui)
set(IMGUI_SOURCES
    ${IMGUI_DIR}/imgui.cpp
    ${IMGUI_DIR}/imgui_draw.cpp
    ${IMGUI_DIR}/imgui_widgets.cpp
    ${IMGUI_DIR}/imgui_tables.cpp
    ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
    ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp
)

# Gladライブラリの設定
add_library(glad_lib STATIC glad/src/glad.c)
target_include_directories(glad_lib PUBLIC glad/include)

# 実行ファイルの作成
add_executable(ImGuiExample main.cpp ${IMGUI_SOURCES})

# 必要なディレクトリをインクルード
target_include_directories(ImGuiExample PUBLIC ${IMGUI_DIR} ${IMGUI_DIR}/backends)

# GUIライブラリの作成
add_library(GUI STATIC
    GUI/lib.cpp
)

# GUIがHtmlParserに依存していることを指定
target_link_libraries(GUI PRIVATE HtmlParser)

# glfwライブラリのディレクトリを指定
add_subdirectory(glfw-3.4)

# 実行ファイルにライブラリをリンク
if(WIN32)
    target_link_libraries(ImGuiExample PRIVATE HtmlParser GUI glad_lib glfw)
else()
    target_link_libraries(ImGuiExample PRIVATE HtmlParser GUI glad_lib glfw GL dl pthread)
endif()

