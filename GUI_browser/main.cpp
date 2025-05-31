#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <iostream>
#include <string>
#include <cstring>
#include <cstdlib>
#include "cpp_parse/html_parser.hpp"
#define STB_IMAGE_IMPLEMENTATION
#include "glfw-3.4/stb_img.h"


// エラーコールバック
void glfw_error_callback(int error, const char* description) {
    std::cerr << "GLFW Error " << error << ": " << description << std::endl;
}

GLuint LoadTexture(const char* filename) {
    int width, height, channels;
    unsigned char* data = stbi_load(filename, &width, &height, &channels, 4);
    if (!data) {
        std::cerr << "Failed to load image: " << filename << std::endl;
        return 0;
    }

    GLuint texture;
    glGenTextures(1, &texture);
    glBindTexture(GL_TEXTURE_2D, texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    stbi_image_free(data);
    return texture;
}

void RenderImageOverlay(GLuint texture) {
    ImGui::Begin("Image Overlay");

    // テクスチャをImGuiウィンドウに描画
    ImVec2 imageSize(300, 300); // 表示サイズ
    ImGui::Image((ImTextureID)(uintptr_t)texture, imageSize);


    ImGui::End();
}

// グローバル変数
char url_buffer[256] = "";  // 検索バー入力用バッファ
std::string display_text = "";  // 表示用テキスト

// 検索バーの描画
void renderSearchBar(int PORT) {
    ImGui::Begin("Search Bar");

    // URL入力フィールド
    ImGui::InputText("Enter file", url_buffer, IM_ARRAYSIZE(url_buffer));

    // Goボタンの処理
    if (ImGui::Button("Go")) {
        if (strlen(url_buffer) > 0) {
            // 入力されたURLをパースして表示テキストを更新
            display_text = nana::parseHtml(PORT, 0, url_buffer);  // サンプルPORTとプロトコル
        }
    }

    ImGui::End();
}

// メインウィンドウ関数
int window(int PORT, const std::string& initial_text) {
    // GLFW初期化
    glfwSetErrorCallback(glfw_error_callback);
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }

    // OpenGLコンテキスト設定
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    // ウィンドウ作成
    GLFWwindow* window = glfwCreateWindow(1280, 720, "ImGui + GLFW + OpenGL", nullptr, nullptr);
    if (!window) {
        std::cerr << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }

    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);  // 垂直同期

    // ImGui初期化
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO(); (void)io;

    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init("#version 330");

    ImGui::StyleColorsDark();

    // 初期表示テキスト
    display_text = initial_text;
    //GLFWwindow* window = glfwCreateWindow(800, 600, "Image Example", NULL, NULL);

    GLuint texture = LoadTexture("test.jpg");
    if (texture == 0) {
        glfwTerminate();
        return -1;
    }
    // メインループ
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        // ImGuiフレーム開始
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

	RenderImageOverlay(texture);

        // 検索バー
        renderSearchBar(PORT);

        // 表示テキストのウィンドウ
        ImGui::Begin("HTML Viewer");
        ImGui::TextWrapped("%s", display_text.c_str());
        ImGui::End();

        // 描画
        ImGui::Render();
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

        glfwSwapBuffers(window);
    }

    // 終了処理
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}

int main(int argc, char* argv[]) {
    if (argc < 3) {
        std::cerr << "Usage: " << argv[0] << " <port> <protocol (tcp/udp)> <filename>" << std::endl;
        return 1;
    }

    int PORT = std::atoi(argv[1]);
    int use_udp = (std::strcmp(argv[2], "udp") == 0) ? 1 : 0;

    const char* filename = argv[3];
    std::string parsedHtml = nana::parseHtml(PORT, use_udp, filename);

    std::cout << "cui/gui [1:2 mode]: ";
    int mode;
    std::cin >> mode;

    if (mode == 1) {
        std::cout << parsedHtml << std::endl;
    } else {
        window(PORT, parsedHtml);
    }

    return 0;
}

