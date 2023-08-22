#include <stdio.h>
int input = 2;
int main(){
    int output;
    asm(
        "movl %1, %%ebx;"
        "shll $1, %%ebx;"
        "movl %%ebx, %0;"
        :"=r" (output)
        :"r" (input)
         :"%ebx");
    printf("%d",output);
}
