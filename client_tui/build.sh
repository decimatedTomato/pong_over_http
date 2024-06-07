#!/bin/bash

set -xe

gcc -o bin/pong_client_tui src/*.c -Iinclude -Wall -Werror -pedantic