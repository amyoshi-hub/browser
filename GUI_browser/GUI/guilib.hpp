#ifndef GUILIB_H_
#define GUILIB_H_

#include <string>
#include <GL/gl.h>
#include "../cpp_parse/html_parser.hpp"
#include "../cpp_parse/client.h"

extern char url_buffer[256];
extern char IP_buffer[256];
extern std::string display_text;

int window(const char* IP, int PORT, const std::string& initial_text);
void renderSearchBar(int PORT);
void RenderImageOverlay(GLuint texture);
GLuint LoadTexture(const char* filename);
void glfw_error_callback(int error, const char* description);

#endif
