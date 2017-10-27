// +build darwin freebsd openbsd netbsd

package main

import "syscall"

const (
	RAW_MODE = syscall.TIOCSETAF
)
