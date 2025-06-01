#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include "client.h"

void send_filename(const char *filename, const char* IP, int port) {
    int sock;
    struct sockaddr_in server_addr;

    // ソケット作成
    if ((sock = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
        perror("Socket creation failed");
        exit(1);
    }

    // サーバーアドレス設定
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);

    if (inet_pton(AF_INET, IP, &server_addr.sin_addr) <= 0) {
        perror("Invalid address");
        close(sock);
        exit(1);
    }

    // ファイル名を送信
    if (sendto(sock, filename, strlen(filename), 0,
               (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("Send failed");
        close(sock);
        exit(1);
    }

    printf("Sent filename: %s\n", filename);

    close(sock);
}
