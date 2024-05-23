#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <ncurses.h>
#include <unistd.h>
#include <pthread.h>
#include <string.h>

#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/udp.h>
#include <arpa/inet.h>

void *chatwin(void *sock)
{
    int fd = *((int *)sock);
    char buf[1024] = {0};
    int count = 0;

    WINDOW *chat = newwin(LINES - 2, COLS, 0, 0);
    while (true)
    {
        memset(buf, 0, 1024);
        recv(fd, buf, 1024, 0);
        mvwprintw(chat, count, 0, buf);
        count++;
        wrefresh(chat);
    }
    return NULL;
}

int main(int argc, char *argv[])
{
    initscr();
    cbreak();
    noecho();
    notimeout(stdscr, false);
    keypad(stdscr, true);

    // Create a socket
    int fd = socket(AF_INET, SOCK_DGRAM, 0);
    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    struct sockaddr_in info = {
        .sin_family = AF_INET,
        .sin_port = htons(atoi(argv[1]))};
    inet_pton(AF_INET, "0.0.0.0", &info.sin_addr);

    /*connect(fd, &info, sizeof(info));
    struct sockaddr_in info2 = {
        .sin_family = AF_INET,
        .sin_port = htons(atoi(argv[2]))};
    inet_pton(AF_INET, "127.0.0.1", &info.sin_addr);*/
    bind(fd, (struct sockaddr *)&info, sizeof(info));

    pthread_t id;
    pthread_create(&id, NULL, chatwin, &fd);

    while (true)
    {
        mvprintw(LINES - 1, 0, "> ");
        refresh();
        sleep(1);
    }
}