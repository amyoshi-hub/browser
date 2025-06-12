#include "listener.hpp"
#include <cstring>
#include <iostream>
#include <arpa/inet.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

#define MAX_BUFFER 1024

void listener(int port, int ip_v) {
    int sockfd;
    char buffer[MAX_BUFFER];
    struct sockaddr_in server_addr4, client_addr4;
    struct sockaddr_in6 server_addr6, client_addr6;

    // ソケット作成
    if (ip_v == 0) { // IPv4
        if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
            perror("Socket creation failed");
            exit(EXIT_FAILURE);
        }

        memset(&server_addr4, 0, sizeof(server_addr4));
        server_addr4.sin_family = AF_INET;
        server_addr4.sin_addr.s_addr = INADDR_ANY;
        server_addr4.sin_port = htons(port);

        if (bind(sockfd, (const struct sockaddr *)&server_addr4, sizeof(server_addr4)) < 0) {
            perror("Bind failed");
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
        server_addr6.sin6_addr = in6addr_any;
        server_addr6.sin6_port = htons(port);

        if (bind(sockfd, (const struct sockaddr *)&server_addr6, sizeof(server_addr6)) < 0) {
            perror("Bind failed");
            close(sockfd);
            exit(EXIT_FAILURE);
        }
    } else {
        std::cerr << "Invalid IP version specified. Use 0 for IPv4, 1 for IPv6." << std::endl;
        exit(EXIT_FAILURE);
    }

    // メッセージ受信ループ
    while (true) {
        socklen_t len;
        int n;

        if (ip_v == 0) {
            len = sizeof(client_addr4);
            n = recvfrom(sockfd, buffer, MAX_BUFFER, MSG_WAITALL, 
                         (struct sockaddr *)&client_addr4, &len);
        } else if (ip_v == 1) {
            len = sizeof(client_addr6);
            n = recvfrom(sockfd, buffer, MAX_BUFFER, MSG_WAITALL, 
                         (struct sockaddr *)&client_addr6, &len);
        }

        if (n < 0) {
            perror("Receive failed");
            break;
        }

        buffer[n] = '\0'; // NULL文字で終端
        std::cout << "Received: " << buffer << std::endl;
    }

    close(sockfd);
}

