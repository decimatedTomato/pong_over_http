#ifdef __linux__
    #include <sys/socket.h>
#elif _WIN32
    #include <winsock2.h>
#endif

