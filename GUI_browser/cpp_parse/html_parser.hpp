#ifndef HTML_PARSER_H
#define HTML_PARSER_H

#include <string>
#include "client.h"

namespace nana {
    std::string parseHtml(char* IP, int port, int use_udp, const char* filename);
}
#endif

