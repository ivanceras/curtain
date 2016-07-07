#!/bin/sh
echo "Starting nginx...." &&\
service nginx start &\
echo "Starting postgresql.." &&\
service postgresql start &&\
echo "Launching curtain.." &&\
/home/curtain/iron_curtain
