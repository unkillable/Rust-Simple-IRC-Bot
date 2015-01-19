use std::io::TcpStream;
use std::io::BufferedStream;
fn main() {
	let nick = "RustBOT";
	let mut channel = "#mootsinsuits";
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
		if buffer.as_slice().contains("396") {
			socket.write(join_packet.as_bytes());
			socket.flush();
			println!("Sent join packet");
		}
		if buffer.as_slice().contains("PING ") {
			let mut hashbit = buffer.split_str("PING ");
			for s in hashbit{
				if s.contains(":") {
					let pong = format!("PONG {}\r\n", s);
					socket.write(pong.as_bytes());
					socket.flush();
					println!("Sent PONG");
				}
				println!("{}", s);
			}
		}
		if buffer.as_slice().contains(".j ") {
			let mut chan = buffer.split_str(".j ");
			for s in chan{
				if s.contains("#"){
					let j = format!("JOIN {}\r\n", s);
					socket.write(j.as_bytes());
					println!("Sent join channel");
				}
			}
		}
		if buffer.as_slice().contains(".p ") {
			let mut chan = buffer.split_str(".p ");
			for s in chan{
				if s.contains("#") {
					let p = format!("PART {}\r\n", s);
					socket.write(p.as_bytes());
					socket.flush();
					println!("Sent PART channel");
				}
			}
		}
		if buffer.as_slice().contains(".q") {
			socket.write(b"QUIT\r\n");
			socket.flush();
			y = 0;
		}
		if buffer.contains(".rust") {
			let mut chan = buffer.split_str(".rust ");
			for s in chan {
				if s.contains("#") {
					println!("Channel found");
					let r = format!("PRIVMSG {} :I am a bot made in Rust.\r\n", s.trim());
					socket.write(r.as_bytes());
					socket.flush();
				}
			}
		}
	}
}
