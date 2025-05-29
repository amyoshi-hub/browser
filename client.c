/*
GUIと連携してUDP（OSAIのほうと標準httpを切り替えれるように）
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 8080

int main() {
    int sock;
    struct sockaddr_in server_addr;
    //char request[] = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    char request[] = "GET /index.html HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    char buffer[4096];

    // ソケット作成
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // サーバーアドレス設定
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(SERVER_PORT);
    inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

    // サーバーへ接続
    if (connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("connect");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // HTTPリクエスト送信
    send(sock, request, strlen(request), 0);

    // レスポンス受信
    int received;
    while ((received = recv(sock, buffer, sizeof(buffer) - 1, 0)) > 0) {
        buffer[received] = '\0';
        printf("%s", buffer);
    }

    // ソケット閉じる
    close(sock);
    return 0;
}
