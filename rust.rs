use std::io::TcpStream;
use std::io::BufferedStream;
fn main() {
	let nick = "RustBOT";
	let channel = "#mootsinsuits";
	let nick_packet = format!("NICK {}\r\n", nick);
	let user_packet = format!("USER {} {} {} :{}\r\n", nick, nick, nick, nick);
	let join_packet = format!("JOIN {}\r\n", channel);
	let mut socket = BufferedStream::new((TcpStream::connect("irc.tm:6667").unwrap()));
	socket.write(nick_packet.as_bytes());
	println!("Sent {}", nick_packet.trim());
	socket.write(user_packet.as_bytes());
	println!("Sent {}", user_packet.trim());
	socket.flush();
	let mut y = 1;
	while(y == 1) {
		let buffer = socket.read_line().unwrap();
		println!("{}", buffer.trim());
		let mut chan = "";
		if buffer.contains("PRIVMSG ") {
			let mut data = buffer.split_str("PRIVMSG ").nth(1).unwrap();
			chan = data.split_str(" :").nth(0).unwrap().trim();
		}
		if buffer.contains("396") {
			socket.write(join_packet.as_bytes());
			socket.flush();
			println!("Sent join packet");
		}
		if buffer.contains("PING ") {
			let mut hashbit = buffer.split_str("PING ").nth(1).unwrap();
			let pong = format!("PONG {}\r\n", hashbit);
			socket.write(pong.as_bytes());
			socket.flush();
			println!("Sent PONG");
		}
		if buffer.contains(".j ") {
			let mut chan = buffer.split_str(".j ").nth(1).unwrap();
			let j = format!("JOIN {}\r\n", chan);
			socket.write(j.as_bytes());
			socket.flush();
			println!("Sent join channel");
		}
		if buffer.contains(".p ") {
			let mut chan = buffer.split_str(".p ").nth(1).unwrap();
			let p = format!("PART {}\r\n", chan);
			socket.write(p.as_bytes());
			socket.flush();
			println!("Sent PART channel");
		}
		if buffer.contains(".q") {
			socket.write(b"QUIT\r\n");
			socket.flush();
			y = 0;
		}
		if buffer.contains(".rust") {
			let r = format!("PRIVMSG {} :I am a bot made in Rust.\r\n", chan);
			socket.write(r.as_bytes());
			socket.flush();
		}
	}
}
