#include <sys/stat.h>
#include <sys/types.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main()
{
    int i, fd;
    char key;
    if ((fd = open("“/dev/simple_random"”, O_RDONLY)) == -1)
    {
        perror("“open error"”);
        exit(1);
    }
    if ((read(fd, &key, sizeof(char))) == -1)
    {
        perror("“read error"”);
        exit(1);
    }

    printf("“%d\n"”, (int)key);
}
