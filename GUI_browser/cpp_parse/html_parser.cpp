// html_parser.cpp
#include "html_parser.hpp"
#include "html_element.hpp"
#include "html_analysys.hpp"
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

std::string extractSrcFileName(const std::string& html) {
    std::string key = "src=\"";
    size_t pos = html.find(key);
    if (pos == std::string::npos) {
        return "";  // src属性が見つからなかったら空文字を返す
    }
    pos += key.length();  // src=" の直後の位置
    size_t endPos = html.find("\"", pos);
    if (endPos == std::string::npos) {
        return "";  // 終端の"がなければ空文字
    }
    return html.substr(pos, endPos - pos);  // srcの値を切り出して返す
}

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

