cmake_minimum_required(VERSION 3.10)
project(ImGuiExample)

set(CMAKE_CXX_STANDARD 11)

# HtmlParser ライブラリの作成
add_library(HtmlParser STATIC
    cpp_parse/html_parser.cpp
    cpp_parse/html_analysys.cpp
    cpp_parse/html_element.cpp
    cpp_parse/client.c
    cpp_parse/img_recv.c
    cpp_parse/img_req.c
)

# GUI ライブラリの作成
add_library(GUI STATIC
    GUI/lib.cpp
)

# GUI が HtmlParser に依存していることを指定
target_link_libraries(GUI PRIVATE HtmlParser)

# ImGui ソースファイルの設定
set(IMGUI_DIR imgui)
set(IMGUI_SOURCES
    ${IMGUI_DIR}/imgui.cpp
    ${IMGUI_DIR}/imgui_draw.cpp
    ${IMGUI_DIR}/imgui_widgets.cpp
    ${IMGUI_DIR}/imgui_tables.cpp
    ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
    ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp
)

# Glad ライブラリの設定
add_library(glad glad/src/glad.c)
target_include_directories(glad PUBLIC glad/include)

# GLFW ライブラリの設定
add_subdirectory(glfw-3.4)
target_include_directories(glad PUBLIC glfw-3.4/include)

# 実行ファイルの作成
add_executable(ImGuiExample main.cpp ${IMGUI_SOURCES})
target_include_directories(ImGuiExample PUBLIC ${IMGUI_DIR} ${IMGUI_DIR}/backends GUI)

# 必要なライブラリをリンク
target_link_libraries(ImGuiExample PRIVATE HtmlParser GUI glfw glad GL dl pthread)

