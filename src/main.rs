extern crate gtk;
extern crate gio;
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use gtk::{
	Builder, ApplicationWindow, Button, Entry
};

use std::env::args;

extern crate ftp_lib;

struct FtpClientApplication {
	ftp_connection: ftp_lib::FtpConnection
}

impl FtpClientApplication {
	fn new(application: &gtk::Application){
		println!("Major: {}, Minor: {}",
			 gtk::get_major_version(),
			 gtk::get_minor_version()
		);
		
		let glade_src = include_str!("ftp_client.glade");
		let builder = Builder::new_from_string(glade_src);
		
		// Get an instance to and set up the components
		let window: ApplicationWindow = builder
			.get_object("window")
			.expect("Couldn't get window");
		window.set_application(Some(application));
		
		let connect_button: Button = builder
			.get_object("connect_button")
			.expect("Couldn't get connect_button");
		
		let ip_entry: Entry = builder
			.get_object("ip_entry")
			.expect("Couldn't get ip_entry");
		
		let username_entry: Entry = builder
			.get_object("username_entry")
			.expect("Couldn't get username_entry");
		
		let password_entry: Entry = builder
			.get_object("password_entry")
			.expect("Couldn't get password_entry");
		
		connect_button.connect_clicked(clone!(@weak window,
						      @weak ip_entry,
						      @weak username_entry,
						      @weak password_entry => move |_| {
							      // Fetch the input in the three fields
							      let socket_addr: String = ip_entry.get_text()
								      .expect("Failed to fetch IP from input.")
								      .to_string();
							      let username: String = username_entry.get_text()
								      .expect("Failed to fetch username from input.")
								      .to_string();
							      let password: String = password_entry.get_text()
								      .expect("Failed to fetch password from input.")
								      .to_string();
							      // Connect to FTP server and store it
							      
						      }));
		
		window.show_all();	
	}
}

fn main() {
	let application = gtk::Application::new(
		Some("com.github.NicksWorld.FtpClient"),
		Default::default(),
	).expect("Initialization failed");

	application.connect_activate(|app| {
		FtpClientApplication::new(app);
	});

	application.run(&args().collect::<Vec<_>>());
}
