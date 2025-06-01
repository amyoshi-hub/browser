#include "../imgui/imgui.h"
#include "../imgui/backends/imgui_impl_glfw.h"
#include "../imgui/backends/imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <iostream>
#include <string>
#include <cstring>
#include <cstdlib>
#include "../cpp_parse/html_parser.hpp"
#define STB_IMAGE_IMPLEMENTATION
#include "../glfw-3.4/stb_img.h"
#include "guilib.hpp"



// グローバル変数
char url_buffer[256] = "";  // 検索バー入力用
char IP_buffer[256] = "";   // IP入力用
std::string display_text;   // 表示用テキスト
			    //
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

// 検索バーの描画
void renderSearchBar(int PORT) {
    ImGui::Begin("Search Bar");

    // URL入力フィールド
    ImGui::InputText("Enter file", url_buffer, IM_ARRAYSIZE(url_buffer));
    ImGui::InputText("Enter IP", IP_buffer, IM_ARRAYSIZE(IP_buffer));

    // Goボタンの処理
    if (ImGui::Button("Go")) {
        if (strlen(url_buffer) > 0) {
            // 入力されたURLをパースして表示テキストを更新
            display_text = nana::parseHtml(IP_buffer, PORT, 0, url_buffer);  // サンプルPORTとプロトコル
        }
    }
    ImGui::End();
}

// メインウィンドウ関数
int window(const char* IP, int PORT, const std::string& initial_text) {
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

    std::string img_file_str = extractSrcFileName(display_text);
    const char* img_file_cstr = img_file_str.c_str();
    GLuint texture;
    if(img_file_cstr){
    	GLuint texture = LoadTexture(img_file_cstr);
	img_receive(img_file_cstr, IP, 1234);
    }

    // メインループ
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        // ImGuiフレーム開始
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

	if(texture){
		RenderImageOverlay(texture);
	}

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

