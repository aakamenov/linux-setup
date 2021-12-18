#!/bin/bash

# Forked from: https://github.com/PrayagS/polybar-spotify

# see man zscroll for documentation of the following parameters
zscroll -l 25 \
        --delay 0.1 \
        --scroll-padding " | " \
        --match-command "`dirname $0`/get_player_status.sh --status" \
        --match-text "Playing" "--scroll 1" \
        --match-text "Paused" "--scroll 0" \
        --update-check true "`dirname $0`/get_player_status.sh" &

wait
