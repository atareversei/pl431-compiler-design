/*
Develop a function that reads a line from the standard input with
time constraints.
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/time.h>
#include<sys/types.h>

char *trl(int timeout) {
  static char buf[512]; // the content must stay in memory.
  fd_set rfd;
  struct timeval tv;
  int ret;

  FD_ZERO(&rfd);
  FD_SET(STDIN_FILENO, &rfd);

  tv.tv_sec = timeout;
  tv.tv_usec = 0;

  // `1` means highest file descriptor being monitored + 1
  ret = select(1, &rfd, NULL, NULL, &tv);
  if (ret > 0 && FD_ISSET(STDIN_FILENO, &rfd)) {
    memset(buf, 0, 512);
    // 512 - '\0' character = 511
    ret = read(0, buf, 511);
    if (ret < 0) {
      return NULL;
    }
    ret--;
    buf[ret] = 0;

    return buf;
  }

  return NULL;
}