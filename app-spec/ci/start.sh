#!/bin/bash

systemctl enable postgresql
systemctl enable gmt-server
systemctl enable gmt-api

systemctl start postgresql
systemctl start gmt-server
systemctl start gmt-api