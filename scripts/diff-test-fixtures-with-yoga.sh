#!/bin/bash

# Pass this as a parameter
YOGA_FIXTURE_DIR=$1
YOGA_TEST_LIST=$(rg '<div id="([a-z_]*)".*' --replace '$1' -o --no-line-number --no-filename $1 | grep -v '^$' | sed -E 's/^(row|column)_gap/gap_\1_gap/' | sort)

TAFFY_FIXTURE_DIR=$(SELF=$(dirname "$0") && bash -c "cd \"$SELF\" && cd ../test_fixtures && pwd")
TAFFY_TEST_LIST=$(cd $TAFFY_FIXTURE_DIR && fd -x echo {.} | grep -v '^grid' | sort)

# DIFF=$(diff <(echo "$YOGA_TEST_LIST") <(echo "$TAFFY_TEST_LIST"))
# echo "$DIFF"

diff -U 1000000 --color <(echo "$YOGA_TEST_LIST") <(echo "$TAFFY_TEST_LIST")