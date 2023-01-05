#!/bin/bash

if [[ ${YOGA_FIXTURE_DIR+dummy} == "" ]]; then
  echo "Error: YOGA_FIXTURE_DIR not set";
  echo "Please set the YOGA_FIXTURE_DIR environment variable to the absolute path of the gentest/fixtures dir in a local copy of the yoga repo";
  exit 1;
fi

if [ ! -d "$YOGA_FIXTURE_DIR" ]; then
  echo "YOGA_FIXTURE_DIR directory $YOGA_FIXTURE_DIR does not exist."
  exit 1;
fi

YOGA_TEST_LIST=$(rg '<div id="([a-z_]*)".*' --replace '$1' -o --no-line-number --no-filename $YOGA_FIXTURE_DIR | grep -v '^$' | sed -E 's/^(column_row|row|column)_gap/gap_\1_gap/' | sort)

TAFFY_FIXTURE_DIR=$(SELF=$(dirname "$0") && bash -c "cd \"$SELF\" && cd ../test_fixtures && pwd")
TAFFY_TEST_LIST=$(cd $TAFFY_FIXTURE_DIR && fd -x echo {.} | grep -v '^grid' | sed -E 's/^x(.*)/\1--disabled/' | sort)

diff -U 1000000 --color <(echo "$YOGA_TEST_LIST") <(echo "$TAFFY_TEST_LIST")