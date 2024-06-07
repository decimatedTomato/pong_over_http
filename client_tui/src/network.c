#ifdef __linux__
    #include <sys/socket.h>
#elif _WIN32
    #include <winsock2.h>
#endif

#include "network.h"

#define PORT_NUMBER 42069
#define IP_ADDR_TEMP "127.0.0.1"

struct http_response
{
    struct parsed_url *request_uri;
    char *body;
    char *status_code;
    int status_code_int;
    char *status_text;
    char *request_headers;
    char *response_headers;
};

bool connect_to_game() {
    // connect
    return true;
}

extern gamestate_t temp; // TODO delete

gamestate_t get_gamestate(user_input_t input) {
    // get gamestate
    if (input == INP_DOWN) {
        temp.p1_pos.y += 10;
    } else if (input == INP_UP) {
        temp.p1_pos.y -= 10;
    }
    return temp;
}

void disconnect_from_game() {
    // disconnect
}