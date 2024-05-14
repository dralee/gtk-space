#!/bin/bash
gcc `pkg-config --cflags --libs gtk+-2.0` -o hello.out main.c

