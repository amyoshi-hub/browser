#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <iostream>
#include "cpp_parse/html_parser.hpp"

void glfw_error_callback(int error, const char* description) {
    std::cerr << "GLFW Error " << error << ": " << description << std::endl;
}

int window(const std::string& text) {
    // GLFW初期化
    glfwSetErrorCallback(glfw_error_callback);
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }

    // OpenGLコンテキストの設定
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* window = glfwCreateWindow(1280, 720, "ImGui + GLFW + OpenGL", NULL, NULL);
    if (!window) {
        std::cerr << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }

    glfwMakeContextCurrent(window);
    glfwSwapInterval(1); // 垂直同期

    // ImGui初期化
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO(); (void)io;

    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init("#version 330");

    ImGui::StyleColorsDark();

    // メインループ
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        // ImGuiフレーム開始
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        // ImGuiのウィジェット
        ImGui::Begin("Hello, world!");
        ImGui::Text("%s", text.c_str());
        ImGui::End();

        // 描画
        ImGui::Render();
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

        glfwSwapBuffers(window);
    }

    // クリーンアップ
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();

    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}

int main(int argc, char* argv[]){
	
    if (argc < 3) {
        printf("Usage: %s <port> <protocol (tcp/udp)>\n", argv[0]);
        exit(1);
    }

    int PORT = atoi(argv[1]);

    // プロトコルを指定して処理を分岐
    int use_udp = 0;
    if (strcmp(argv[2], "udp") == 0) {
        use_udp = 1;
    } else if (strcmp(argv[2], "tcp") != 0) {
        printf("Invalid protocol. Use 'tcp' or 'udp'.\n");
        exit(1);
    }
    char *filename = argv[3];
    std::string parsedHtml = nana::parseHtml(PORT, use_udp, filename);
	window(parsedHtml);
}

