package main

import (
	"syscall"
	"unsafe"
)

type terminal int

func main() {
	var raw syscall.Termios
	tty := terminal(syscall.Stdin)

	raw.Iflag &^= syscall.BRKINT | syscall.ICRNL | syscall.INPCK | syscall.ISTRIP | syscall.IXON
	raw.Oflag &^= syscall.OPOST
	raw.Cflag |= syscall.CS8
	raw.Lflag &^= syscall.ECHO | syscall.ICANON | syscall.ISIG
	raw.Cc[syscall.VMIN] = 1
	raw.Cc[syscall.VTIME] = 0

	_, _, err := syscall.Syscall6(syscall.SYS_IOCTL, uintptr(tty), RAW_MODE, uintptr(unsafe.Pointer(&raw)), 0, 0, 0)
	if err != 0 {
		panic(err)
	}
}
