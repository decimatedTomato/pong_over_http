#!/bin/bash

set -xe

gcc -o bin/pong_client_tui src/*.c -Iinclude -lncurses -lm -Wall -Werror -pedantic