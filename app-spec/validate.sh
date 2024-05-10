#!/bin/bash

db_status=$(systemctl status postgresql)
if [[ $db_status != *"active (running)"* ]]; then
  echo "PostgreSQL is not running."
  exit 1
else
  echo "PostgreSQL is running."
fi

server_status=$(systemctl status gmt-server)
if [[ $server_status != *"active (running)"* ]]; then
  echo "GMT Server is not running."
  exit 1
else
  echo "GMT Server is running."
fi

api_status=$(systemctl status gmt-api)
if [[ $api_status != *"active (running)"* ]]; then
  echo "GMT API is not running."
  exit 1
else
  echo "GMT API is running."
fi