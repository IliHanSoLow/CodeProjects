#include <windows.h>

int main(void)
{
    MessageBoxW(NULL, L"Hello World", L"Hi", MB_ICONEXCLAMATION | MB_OKCANCEL);
    return EXIT_SUCCESS;
}