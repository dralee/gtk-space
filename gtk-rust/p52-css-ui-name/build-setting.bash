#!/bin/bash
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp org.dralee.TodoListCssName.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
ls ~/.local/share/glib-2.0/schemas/