#include <iostream>
#include <thread>
#include <cstring>
#include <arpa/inet.h>
#include <unistd.h>
#include "../lib/lib.hpp"
#include <iostream>
#include <string>
#include "../client/client.hpp"
#include "../server/listener.hpp"
#define MAX_BUFFER 1024

void main() {
    int choice;
    std::cout << "Select mode: \n1. Listener\n2. Sender\n";
    std::cin >> choice;
    std::cin.ignore();
    int ip_v;
    std::cout << "wich ip version? 1:ipv4, 2:ipv6";
    std::cin >> ip_v;

    if (choice == 1) {
        int port;
        std::cout << "Enter port to listen on: ";
        std::cin >> port;
        listener(port, ip_v);
    } else if (choice == 2) {
        std::string ip;
        int port;
        std::cout << "Enter receiver IP: ";
        std::cin >> ip;
        std::cout << "Enter receiver port: ";
        std::cin >> port;
        std::cin.ignore(); // 入力バッファをクリア
        sender(ip, port, ip_v);
    } else {
        std::cout << "Invalid choice" << std::endl;
    }
}

