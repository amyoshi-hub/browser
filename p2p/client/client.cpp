#include "client.hpp"
#include <cstring>
#include <iostream>
#include <arpa/inet.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

#define MAX_BUFFER 1024

void sender(const std::string &ip, int port, int ip_v) {
    int sockfd;
    char buffer[MAX_BUFFER];
    struct sockaddr_in server_addr4;
    struct sockaddr_in6 server_addr6;

    // ソケット作成
    if (ip_v == 0) { // IPv4
        if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
            perror("Socket creation failed");
            exit(EXIT_FAILURE);
        }

        memset(&server_addr4, 0, sizeof(server_addr4));
        server_addr4.sin_family = AF_INET;
        server_addr4.sin_port = htons(port);

        if (inet_pton(AF_INET, ip.c_str(), &server_addr4.sin_addr) <= 0) {
            perror("Invalid IPv4 address/ Address not supported");
            close(sockfd);
            exit(EXIT_FAILURE);
        }
    } else if (ip_v == 1) { // IPv6
        if ((sockfd = socket(AF_INET6, SOCK_DGRAM, 0)) < 0) {
            perror("Socket creation failed");
            exit(EXIT_FAILURE);
        }

        memset(&server_addr6, 0, sizeof(server_addr6));
        server_addr6.sin6_family = AF_INET6;
        server_addr6.sin6_port = htons(port);

        if (inet_pton(AF_INET6, ip.c_str(), &server_addr6.sin6_addr) <= 0) {
            perror("Invalid IPv6 address/ Address not supported");
            close(sockfd);
            exit(EXIT_FAILURE);
        }
    } else {
        std::cerr << "Invalid IP version specified. Use 0 for IPv4, 1 for IPv6." << std::endl;
        exit(EXIT_FAILURE);
    }

    // メッセージ送信ループ
    while (true) {
        std::cout << "Enter message: ";
        std::cin.getline(buffer, MAX_BUFFER);

        if (ip_v == 0) {
            sendto(sockfd, buffer, strlen(buffer), MSG_CONFIRM,
                   (const struct sockaddr *)&server_addr4, sizeof(server_addr4));
        } else if (ip_v == 1) {
            sendto(sockfd, buffer, strlen(buffer), MSG_CONFIRM,
                   (const struct sockaddr *)&server_addr6, sizeof(server_addr6));
        }

        std::cout << "Message sent" << std::endl;
    }

    close(sockfd);
}
