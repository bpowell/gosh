extern crate libc;

fn new_termios() -> libc::termios {
    let original = libc::termios {
        c_iflag: 1,
        c_oflag: 1,
        c_cflag: 1,
        c_lflag: 1,
        c_cc: ['0' as libc::cc_t;20],
        c_ispeed: 1,
        c_ospeed: 1,
	};

    original
}

fn raw_mode(term: libc::termios) {
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &term);
    }
}

fn main() {
    let mut original: libc::termios = new_termios();
    let mut raw: libc::termios = new_termios();

    unsafe {
        libc::tcgetattr(libc::STDIN_FILENO, &mut original);
        libc::tcgetattr(libc::STDIN_FILENO, &mut raw);
    }

    raw.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    raw.c_oflag &= !(libc::OPOST);
    raw.c_cflag |= libc::CS8;
    raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);

    raw_mode(raw);
    unsafe {
        let mut buf = String::from("\x1b[?25l\x1b[HHello World");
        libc::write(libc::STDIN_FILENO, buf.as_ptr() as *const libc::c_void, buf.len());
        buf.clear();
        buf.push_str("Goodbye!");
        libc::write(libc::STDIN_FILENO, buf.as_ptr() as *const libc::c_void, buf.len());
    }

    raw_mode(original);
}
