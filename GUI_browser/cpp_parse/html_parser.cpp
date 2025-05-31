// html_parser.cpp
#include "html_parser.hpp"
#include "html_element.hpp"
#include "html_analysys.hpp"
#include <iostream>
#include <fstream>
#include <sstream>

namespace nana {
    std::string parseHtml(char *IP, int port, int use_udp, const char* filename) {
        std::string str = client(IP, port, use_udp, filename);
        HtmlSaxParser parser;
        DocumentHtmlSaxParserHandler handler;
        std::istringstream is(str);
        
        // パース
        parser.parse(is, handler);
        auto docUptr = handler.result();
        
        // パース結果を文字列化
        std::ostringstream result;
        for (HtmlDocument::const_iterator i = docUptr->begin(); i != docUptr->end(); ++i) {
            result << **i << "#" << (*i)->tagName() << std::endl;
        }
        return result.str();
    }
}

