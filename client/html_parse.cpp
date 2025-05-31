#include <iostream>
#include <string>
#include <vector>

class HTMLNode {
public:
    std::string tagName;
    std::string textContent;
    std::vector<HTMLNode*> children;

    HTMLNode(const std::string& tag) : tagName(tag) {}

    // 子ノードを追加する
    void addChild(HTMLNode* child) {
        children.push_back(child);
    }

    // ノードの情報を表示する（デバッグ用）
    void printTree(int indent = 0) const {
        for (int i = 0; i < indent; ++i) std::cout << "  ";
        std::cout << "<" << tagName << ">" << std::endl;
        if (!textContent.empty()) {
            for (int i = 0; i < indent + 1; ++i) std::cout << "  ";
            std::cout << textContent << std::endl;
        }
        for (const auto& child : children) {
            child->printTree(indent + 1);
        }
        for (int i = 0; i < indent; ++i) std::cout << "  ";
        std::cout << "</" << tagName << ">" << std::endl;
    }
};

class HTMLParser {
public:
    HTMLNode* parse(const std::string& html) {
        HTMLNode* root = new HTMLNode("root");
        HTMLNode* currentNode = root;

        size_t i = 0;
        while (i < html.size()) {
            if (html[i] == '<') {
                // タグの開始
                size_t tagEnd = html.find('>', i);
                std::string tag = html.substr(i + 1, tagEnd - i - 1);
		std::cout << tag << std::endl;
                HTMLNode* newNode = new HTMLNode(tag);

                currentNode->addChild(newNode);
                currentNode = newNode;
                i = tagEnd + 1;
            } else if (html[i] == '>') {
                // タグの閉じ
                currentNode = root;  // とりあえず最上位に戻す
                i++;
            } else {
                // テキストコンテンツ
                size_t textEnd = html.find('<', i);
                currentNode->textContent = html.substr(i, textEnd - i);
                i = textEnd;
            }
        }

        return root;
    }
};

int main() {

    extern "C"{
    
    }

    HTMLParser parser;
    std::string html = "<html><body><h1>Hello, world!</h1><p>Welcome to HTML parsing!</p></body></html>";

    HTMLNode* root = parser.parse(html);

    // パースしたツリーを表示
    root->printTree();

    // メモリ解放（ここでは簡単化のため）
    delete root;
    return 0;
}

