#ifndef HTML_PARSER_H
#define HTML_PARSER_H

#include <string>
#include "client.h"
#include "html_parser.hpp"

std::string extractSrcFileName(const std::string& html);

namespace nana {
    std::string parseHtml(char* IP, int port, int use_udp, const char* filename);
}
#endif

