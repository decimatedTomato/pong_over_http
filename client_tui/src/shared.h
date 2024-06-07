#pragma once

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <math.h>

#ifdef __linux__
    #include <unistd.h>
    #define Milisleep(X) usleep((X)*1000)
#elif _WIN32
#endif

typedef enum {
    INP_NEUTRAL = 0,
    INP_UP,
    INP_DOWN,
    INP_QUIT,
} user_input_t;

typedef struct {
    float x;
    float y;
} Vec2;

typedef struct {
    Vec2 p1_pos;
    Vec2 p2_pos;
    Vec2 ball_pos;
} gamestate_t;