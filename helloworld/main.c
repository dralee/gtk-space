#include <gtk/gtk.h>

static void on_active(GtkApplication *app){
	// create a new window
	GtkWidget *window = gtk_application_window_new(app);
	// create a new button
	GtkWidget *button = gtk_button_new_with_label("Hello, World!");
	// When the button is clicked, close the window passed as an argument
	g_signal_connect_swapped(button, "clicked", G_CALLBACK(gtk_window_close), window);
	gtk_window_set_child(GTK_WINDOW(window), button);
	gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char *argv[]){
	// Create a new application
	GtkApplication *app = gtk_application_new("top.dralee.GtkApplication", G_APPLICATION_FLAGS_NONE);
	g_signal_connect(app, "activate", G_CALLBACK(on_active), NULL);
	return g_application_run(G_APPLICATION(app), argc, argv);
}
