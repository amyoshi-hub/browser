#ifndef CLIENT_H_
#define CLIENT_H_

#ifdef __cplusplus
extern "C" {
#endif

char* client(int port, int use_udp, const char* filename);

#ifdef __cplusplus
}
#endif

#endif
