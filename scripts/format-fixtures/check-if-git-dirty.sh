#!/bin/bash

if [ -z "$(git status --porcelain)" ]; then 
  # Working directory clean
  echo "Fixtures formatted correctly"
  exit 0
else 
  # Uncommitted changes
  echo "Test fixtures are not formatted correctly."
  echo "Please run 'cargo format-fixtures'"
  exit 1
fi