#include <stdint.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/udp.h>
#include <strings.h>
#include <stdio.h>
#include <errno.h>

/* TODO: Add proper error handling */

int socket_bind(int fd, const uint8_t ip[4], const uint16_t port)
{
    uint32_t ip_converted = ip[3] << 24 | ip[2] << 16 | ip[1] << 8 | ip[0];
    struct sockaddr_in info = {
        .sin_addr.s_addr = ip_converted,
        .sin_family = AF_INET,
        .sin_port = htons(port)};
    if (bind(fd, (struct sockaddr *)&info, sizeof(info)) < 0)
    {
        perror("Could not bind");
        return -1;
    }
    return 0;
}

int socket_create()
{
    int fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (fd < 0)
        return fd;
    int opt = 1;
    // https://www.gnu.org/software/libc/manual/html_node/Socket_002dLevel-Options.html#Socket_002dLevel-Options
    int rc = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));
    if (rc < 0)
    {
        shutdown(fd, SHUT_RDWR);
        return rc;
    }
    return fd;
}