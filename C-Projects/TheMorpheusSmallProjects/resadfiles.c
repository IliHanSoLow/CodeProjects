#include <stdio.h>

FILE *helloWorld;
char buffer;

void printWord()
{
    buffer=fgetc(helloWorld);
    if (fgetc(helloWorld) != EOF)
    {
        printf("%s", buffer);
        printWord();
    }
}

int main()
{
    helloWorld = fopen(".\\helloWorld.c", "r");
    printWord();
    return 0;
}
