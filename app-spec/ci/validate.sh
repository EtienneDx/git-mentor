#!/bin/bash

TIMEOUT=10
RETRY_COUNT=3

# Function to check the status of a service
check_service_status() {
  service_name=$1
  status=$(systemctl status $service_name)
  if [[ $status != *"active (running)"* ]]; then
    echo "$service_name is not running."
    exit 1
  else
    echo "$service_name is running."
  fi
}

# Function to retry a command with a timeout
retry_with_timeout() {
  command=$1
  timeout=$2
  retry_count=$3
  count=0
  while [[ $count -lt $retry_count ]]; do
    eval $command && return 0
    count=$((count+1))
    sleep 1
  done
  echo "Command '$command' failed after $retry_count retries."
  exit 1
}

# Check PostgreSQL status with retry and timeout
retry_with_timeout "check_service_status postgresql" $TIMEOUT $RETRY_COUNT

# Check GMT Server status with retry and timeout
retry_with_timeout "check_service_status gmt-server" $TIMEOUT $RETRY_COUNT

# Check GMT API status with retry and timeout
retry_with_timeout "check_service_status gmt-api" $TIMEOUT $RETRY_COUNT