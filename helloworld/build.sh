#!/bin/bash
gcc `pkg-config --cflags --libs gtk4` -o helloworld.out main.c

