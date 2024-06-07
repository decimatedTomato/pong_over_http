#pragma once
#include "shared.h"

bool connect_to_game();
gamestate_t get_gamestate(user_input_t input);
void disconnect_from_game();