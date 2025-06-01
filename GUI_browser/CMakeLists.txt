cmake_minimum_required(VERSION 3.10)
project(ImGuiExample)

set(CMAKE_CXX_STANDARD 11)

add_library(HtmlParser STATIC
    cpp_parse/html_parser.cpp
    cpp_parse/html_analysys.cpp
    cpp_parse/html_element.cpp
    cpp_parse/client.c
    cpp_parse/img_recv.c
)

add_library(GUI STATIC
    GUI/lib.cpp
)

set(IMGUI_DIR imgui)
set(IMGUI_SOURCES
    ${IMGUI_DIR}/imgui.cpp
    ${IMGUI_DIR}/imgui_draw.cpp
    ${IMGUI_DIR}/imgui_widgets.cpp
    ${IMGUI_DIR}/imgui_tables.cpp
    ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
    ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp
)

add_library(glad glad/src/glad.c)
target_include_directories(glad PUBLIC glad/include)

add_subdirectory(glfw-3.4)
target_include_directories(glad PUBLIC glfw-3.4/include)

add_executable(ImGuiExample main.cpp ${IMGUI_SOURCES})
target_include_directories(ImGuiExample PUBLIC ${IMGUI_DIR} ${IMGUI_DIR}/backends GUI)

target_link_libraries(ImGuiExample PRIVATE HtmlParser GUI glfw glad GL dl pthread)

