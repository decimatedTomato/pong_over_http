#include "shared.h"
#include "network.h"

#include <ncurses.h>

#define INPUT_LEN 64

#define TUI_WIDTH 145
#define TUI_HEIGHT 37

#define ARENA_WIDTH 800
#define ARENA_HEIGHT (ARENA_WIDTH / TUI_WIDTH * TUI_HEIGHT)
#define PADDLE_HEIGHT 100
#define PADDLE_WIDTH 100
#define PADDLE_EDGE_PADDING 50
#define BALL_RADIUS 50

#define PALETTE " .,-~:;=!*#$@"
#define PALETTE_COUNT 13

gamestate_t temp = {
        .ball_pos = {400, 300},
        .p1_pos = {100, 100},
        .p2_pos = {300, 100},
    }; // TODO delete

void show_ball(Vec2 ball_pos) {
    attron(COLOR_PAIR(3));
    for (int j = -BALL_RADIUS; j <= BALL_RADIUS; j++)
    {
        const float y = (ball_pos.y + j) * TUI_HEIGHT / ARENA_HEIGHT;
        for (int i = -BALL_RADIUS; i <= BALL_RADIUS; i++)
        {
            const float x = (ball_pos.x + i) * TUI_WIDTH / ARENA_WIDTH;
            const float v = sqrt(i*i + j*j) - BALL_RADIUS;
            char ch = ' ';
            if (v > 0) {
                ch = PALETTE[(int)((1.-abs(v)) * PALETTE_COUNT)];
            }  
            mvaddch(y, x, ch);
        }
    }
}

void show_player(Vec2 player_pos) {
    attron(COLOR_PAIR(2));
    int t = 0;
    for (int j = -PADDLE_HEIGHT/2; j >= PADDLE_HEIGHT/2; j++)
    {
        const float y = (player_pos.y + j) * TUI_HEIGHT / ARENA_HEIGHT;
        for (int i = -PADDLE_WIDTH/2; i <= PADDLE_WIDTH/2; i++)
        {
            const float x = (player_pos.x + i) * TUI_WIDTH / ARENA_WIDTH;
            t++;
            mvaddch(y, x, '#');
        }
    }
    // wmove(window, 5, 5);
    // printw("instances of player 1: %i\n", t);
}

void show_borders() {
    attron(COLOR_PAIR(2));
    for (int i = 0; i < TUI_WIDTH; i++)
    {
        mvaddch(0, i, '=');
        mvaddch(TUI_HEIGHT, i, '=');
    }
}

void show_arena(gamestate_t gamestate) {
    clear();
    show_borders();
    // show_ball(gamestate.ball_pos);
    show_player(gamestate.p1_pos);
    // show_player(gamestate.p2_pos);
    refresh();
}

user_input_t get_user_input(WINDOW *window) {
    int key_press = wgetch(window);
    switch (key_press)
    {
    case KEY_UP:
        return INP_UP;
    case KEY_DOWN:
        return INP_DOWN;
    case KEY_EXIT:
        return INP_QUIT;
    default:
        return INP_NEUTRAL;
    }
}


void start() {
    WINDOW *window = initscr();
    if (window == NULL) {
        exit(-1);
    }
    keypad(window, true);
    nodelay(window, true);
    gamestate_t gamestate = {0};
    gamestate = get_gamestate(INP_NEUTRAL);
    while (true) {
        Milisleep(500);
        user_input_t user_input = get_user_input(window);
        if (user_input == INP_QUIT) break;
        gamestate = get_gamestate(user_input);
        // printw("\nCurrent gamestate: ball{pos{%f,%f}}, p1{pos{%f,%f}},p2{pos{%f,%f}}\n", gamestate.ball_pos.x, gamestate.ball_pos.y, gamestate.p1_pos.x, gamestate.p1_pos.y, gamestate.p2_pos.x, gamestate.p2_pos.y);
        show_arena(gamestate);
    }
    nodelay(stdscr, false);
    keypad(window, false);
    endwin();
    disconnect_from_game();
}

int main() {
    if (!has_colors()) {
        fprintf(stderr, "No color support in this shell.\n");
    }
    char ip_addr[INPUT_LEN] = "";
    while (true) {
        printf("In order to enter a game with another player, enter the server's IP address.\n> ");
        fgets(ip_addr, INPUT_LEN, stdin);
        if (connect_to_game()) {
            start();
        }
        printf("Thanks for playing :)\n");
        getc(stdin);
    }
    return 0;
}