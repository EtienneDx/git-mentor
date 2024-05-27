#!/bin/bash

systemctl stop gmt-server
systemctl stop gmt-api

systemctl disable gmt-server
systemctl disable gmt-api