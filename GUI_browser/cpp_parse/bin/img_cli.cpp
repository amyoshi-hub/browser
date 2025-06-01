#include <iostream>
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

int main() {
    std::string html = R"(<img src="example.png" alt="Example Image">)";
    
    std::string filename = extractSrcFileName(html);
    if (!filename.empty()) {
        std::cout << "Found src file: " << filename << std::endl;
    } else {
        std::cout << "No src attribute found." << std::endl;
    }
    
    return 0;
}

