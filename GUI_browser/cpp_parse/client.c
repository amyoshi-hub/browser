#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include "client.h"

#define SERVER_IP "127.0.0.1"

char* client(const char* IP, int PORT, int use_udp, const char *filename) {
    int sock;
    struct sockaddr_in server_addr;
    char request[256];
    char *buffer = malloc(4096);  // 動的メモリ割り当て
    if (!buffer) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    snprintf(request, sizeof(request), "GET /%s HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n", filename);

    // ソケット作成
    if (use_udp) {
        sock = socket(AF_INET, SOCK_DGRAM, 0);  // UDP ソケット
    } else {
        sock = socket(AF_INET, SOCK_STREAM, 0);  // TCP ソケット
    }

    if (sock < 0) {
        perror("socket");
        free(buffer);
        exit(EXIT_FAILURE);
    }

    // サーバーアドレス設定
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);

    if (inet_pton(AF_INET, IP, &server_addr.sin_addr) <= 0) {
        perror("inet_pton");
        close(sock);
        free(buffer);
        exit(EXIT_FAILURE);
    }

    // サーバーへ接続 (UDP の場合は不要)
    if (!use_udp && connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("connect");
        close(sock);
        free(buffer);
        exit(EXIT_FAILURE);
    }

    // HTTPリクエスト送信 (UDP の場合は送信先を指定して送信)
    if (use_udp) {
        if (sendto(sock, request, strlen(request), 0, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
            perror("sendto");
        }
    } else {
        if (send(sock, request, strlen(request), 0) < 0) {
            perror("send");
        }
    }

    // レスポンス受信
    int received;
    while ((received = recv(sock, buffer, 4096 - 1, 0)) > 0) {
        buffer[received] = '\0';  // null 終端を追加
        printf("%s", buffer);     // バッファの内容を出力
    }

    if (received < 0) {
        perror("recv");
    }

    // ソケットクリーンアップ
    close(sock);

    return buffer;  // 呼び出し側で free する必要あり
}

