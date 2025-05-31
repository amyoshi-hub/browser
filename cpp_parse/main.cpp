#include <iostream>
#include <fstream>
#include <sstream>
//
#include "html_element.hpp"
#include "html_analysys.hpp"


using namespace std;

int main(int argc, char *argv[]){
	string str("<html><form><div>サンプル</div></form></html>");
	nana::HtmlSaxParser parser;
	nana::DocumentHtmlSaxParserHandler handler;
	istringstream is(str);
	
	//パース
	parser.parse(is, handler);
	unique_ptr<nana::HtmlDocument> docUptr = handler.result();
	
	//パース結果の出力（(*i)は(const HtmlPart*)）
	for(nana::HtmlDocument::const_iterator i = docUptr->begin(); i != docUptr->end(); ++i){
	  cout << **i << "#" << (*i)->tagName() << endl;
	}
	return 0;
}