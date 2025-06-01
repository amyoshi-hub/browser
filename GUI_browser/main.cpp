#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <iostream>
#include <string>
#include <cstring>
#include <cstdlib>
#include "../cpp_parse/html_parser.hpp"
#include "GUI/guilib.hpp"

int main(int argc, char* argv[]) {
    if (argc < 4) {
        std::cerr << "Usage: " << argv[0] << " <IP> <port> <protocol (tcp/udp)> <filename>" << std::endl;
        return 1;
    }

    char *IP = argv[1];
    int PORT = std::atoi(argv[2]);
    int use_udp = (std::strcmp(argv[3], "udp") == 0) ? 1 : 0;

    const char* filename = argv[4];
    std::string parsedHtml = nana::parseHtml(IP, PORT, use_udp, filename);

    std::cout << "cui/gui [1:2 mode]: ";
    int mode;
    std::cin >> mode;

    if (mode == 1) {
        std::cout << parsedHtml << std::endl;
    } else {
        window(IP, PORT, parsedHtml);
    }

    return 0;
}

