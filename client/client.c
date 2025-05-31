#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define SERVER_IP "127.0.0.1"

char* client(int PORT, int use_udp) {
    int sock;
    struct sockaddr_in server_addr;
    char request[] = "GET /index.html HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    static char buffer[4096];  // staticを使って関数外でも使えるようにする

    // ソケット作成
    if (use_udp) {
        sock = socket(AF_INET, SOCK_DGRAM, 0);  // UDP ソケット
    } else {
        sock = socket(AF_INET, SOCK_STREAM, 0);  // TCP ソケット
    }

    if (sock < 0) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // サーバーアドレス設定
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr);

    // サーバーへ接続 (UDP の場合は不要)
    if (!use_udp && connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("connect");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // HTTPリクエスト送信 (UDP の場合は送信先を指定して送信)
    if (use_udp) {
        sendto(sock, request, strlen(request), 0, (struct sockaddr *)&server_addr, sizeof(server_addr));
    } else {
        send(sock, request, strlen(request), 0);
    }

    // レスポンス受信
    int received;
    while ((received = recv(sock, buffer, sizeof(buffer) - 1, 0)) > 0) {
        buffer[received] = '\0';  // null 終端を追加
        printf("%s", buffer);     // バッファの内容を出力
    }

    // ソケット閉じる
    close(sock);
    return buffer;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        printf("Usage: %s <port> <protocol (tcp/udp)>\n", argv[0]);
        exit(1);
    }

    int PORT = atoi(argv[1]);

    // プロトコルを指定して処理を分岐
    int use_udp = 0;
    if (strcmp(argv[2], "udp") == 0) {
        use_udp = 1;
    } else if (strcmp(argv[2], "tcp") != 0) {
        printf("Invalid protocol. Use 'tcp' or 'udp'.\n");
        exit(1);
    }

    // クライアント実行
    printf("Using %s protocol:\n", use_udp ? "UDP" : "TCP");
    printf("%s", client(PORT, use_udp));

    return 0;
}
