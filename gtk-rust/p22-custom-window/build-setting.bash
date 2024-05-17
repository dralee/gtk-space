#!/bin/bash
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp org.gtk_rs.CustomWindow1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
ls ~/.local/share/glib-2.0/schemas/