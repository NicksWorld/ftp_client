extern crate gtk;
extern crate gio;
#[macro_use] extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Dialog;
use gtk::Label;

use gio::subclass::application::ApplicationImplExt;
use gio::ApplicationFlags;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gtk::subclass::prelude::*;

use glib::clone;

use gtk::{
	Builder, ApplicationWindow, Button, Entry
};

use std::env::args;

extern crate ftp_lib;
extern crate tftp_lib;

struct FtpClientApplication {}

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

		let connect_button2: Button = builder
			.get_object("connect_button2")
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

		let tftp_popup: Dialog = builder
			.get_object("tftp_popup")
			.expect("Couldn't get dialog box");

		let tftp_ip: Entry = builder
			.get_object("ip_entry2")
			.expect("Couldn't get dialog box");

		let tftp_label: Label = builder
			.get_object("tftp_label")
			.expect("Couldn't get dialog box");

		let tftp_path: Entry = builder
			.get_object("path_entry")
			.expect("Couldn't get dialog box");
		
		connect_button2.connect_clicked(clone!(@weak tftp_popup,
						      @weak tftp_ip,
						      @weak tftp_path,
						      @weak tftp_label
						       => move |_| {
							      // Fetch the input in the three fields
							      let socket_addr: String = tftp_ip.get_text()
								      .expect("Failed to fetch IP from input.")
								      .to_string();
							      let path: String = tftp_path.get_text()
								      .expect("Failed to fetch username from input.")
								      .to_string();
							       // Connect to TFTP server and store it

							       let sock = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
							       let text = tftp_lib::get_file(&path, &sock).unwrap();

							       println!("{}", String::from_utf8_lossy(&text));

							       tftp_label.set_text(&String::from_utf8(text).unwrap().to_string());
							       
							       tftp_popup.show_all();
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
