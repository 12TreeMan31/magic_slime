#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/udp.h>

#define BUFLEN 1024

int main()
{
    int sfd = socket(AF_INET, SOCK_DGRAM, 0);
    int opt = 1;
    setsockopt(sfd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    struct sockaddr_in sfd_data = {
        .sin_addr.s_addr = INADDR_ANY,
        .sin_family = AF_INET,
        .sin_port = htons(8888),
    };
    bind(sfd, (struct sockaddr *)&sfd_data, sizeof(sfd_data));

    char buf[1024] = {0};
    struct sockaddr_in peer_info = {0};
    socklen_t plen = sizeof(peer_info);
    printf("Started Server\n");

    while (true)
    {
        recvfrom(sfd, buf, BUFLEN, 0, (struct sockaddr *)&peer_info, &plen);
        printf("%s", buf);
        // snprintf(buf, BUFLEN, "{\"CONNECT\":{\"ip\":[0,0,0,0], \"port\":%hu}}", ntohs(peer_info.sin_port));
        snprintf(buf, BUFLEN, "%hu", ntohs(peer_info.sin_port));
        sendto(sfd, buf, BUFLEN, 0, (struct sockaddr *)&peer_info, plen);
    }
}