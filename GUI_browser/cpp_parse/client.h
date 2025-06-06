#ifndef CLIENT_H_
#define CLIENT_H_

#ifdef __cplusplus
extern "C" {
#endif

char* client(const char *IP, int port, int use_udp, const char* filename);
int img_receive(const char*filename, const char* IP, int port);
void send_filename(const char *filename, const char* IP, int port);

#ifdef __cplusplus
}
#endif

#endif
