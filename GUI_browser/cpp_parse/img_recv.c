#include "client.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define MAX_BUFFER_SIZE 1024
#define CHUNK_SIZE 1024
#define END_CHUNK_NUM 0xFFFFFFFF

typedef struct {
    uint32_t chunk_num;
    char data[CHUNK_SIZE];
} Chunk;

int img_receive(const char* IP, int port) {
    int sockfd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t addr_len = sizeof(client_addr);
    Chunk chunk;
    FILE *output_file;
    uint32_t expected_chunk_num = 0;

    if (!port) {
        fprintf(stderr, "Port is required\n");
        return -1;
    }

    if (inet_pton(AF_INET, IP, &server_addr.sin_addr) <= 0) {
        perror("inet_pton");
        close(sockfd);
        exit(EXIT_FAILURE);
    }


    if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
        perror("socket");
        return -1;
    }

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(port);

    if (bind(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind");
        close(sockfd);
        return -1;
    }

    printf("Waiting for incoming data...\n");

    output_file = fopen("received_image.jpg", "wb");
    if (!output_file) {
        perror("fopen");
        close(sockfd);
        return -1;
    }

    while (1) {
        ssize_t len = recvfrom(sockfd, &chunk, sizeof(chunk), 0, (struct sockaddr *)&client_addr, &addr_len);
        if (len < 0) {
            perror("recvfrom");
            break;
        }
        if (len < sizeof(uint32_t)) {
            fprintf(stderr, "Received packet too small\n");
            continue;
        }

        uint32_t chunk_num = ntohl(chunk.chunk_num);

        if (chunk_num == END_CHUNK_NUM) {
            break;
        }

        if (chunk_num == expected_chunk_num) {
            fwrite(chunk.data, 1, len - sizeof(uint32_t), output_file);
            expected_chunk_num++;
            printf("Received chunk %u\n", chunk_num);
        } else {
            printf("Out-of-order chunk %u received, expected %u\n", chunk_num, expected_chunk_num);
        }
    }

    fclose(output_file);
    close(sockfd);

    printf("File received and saved as received_image.jpg\n");
    return 0;
}

