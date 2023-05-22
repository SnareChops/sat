package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/types"
)

type Reader struct {
	file     string
	row      int
	col      int
	cursor   int
	contents []rune
}

func NewReader(file, contents string) *Reader {
	return &Reader{file, 1, 1, 0, []rune(contents)}
}

func (self *Reader) rune() (rune, bool) {
	if self.cursor < len(self.contents) {
		return self.contents[self.cursor], true
	}
	return '\000', false
}

func (self *Reader) at(cursor int) (rune, bool) {
	if cursor > 0 && cursor < len(self.contents) {
		return self.contents[cursor], true
	}
	return '\000', false
}

func (self *Reader) prev() (rune, bool) {
	return self.at(self.cursor - 1)
}

func (self *Reader) next() (rune, bool) {
	return self.at(self.cursor + 1)
}

func (self *Reader) advance() (rune, bool) {
	if char, ok := self.rune(); ok && char == '\n' {
		self.row += 1
		self.col = 0
	}
	self.col += 1
	self.cursor += 1
	if *options.Trace {
		fmt.Printf("\t%s\n", self.debug())
	}
	return self.rune()
}

func (self *Reader) loc() types.Location {
	return types.Location{self.file, self.row, self.col}
}

func (self *Reader) debug() string {
	out := ""
	for i, c := range self.contents {
		var char string
		switch c {
		case '\n':
			char = "\\n"
		case '\t':
			char = "\\t"
		case '\r':
			char = "\\r"
		default:
			char = string(c)
		}

		if i == self.cursor {
			out += "(" + char + ")"
		} else {
			out += char
		}
	}
	return fmt.Sprintf("%s:%d:%d: %s", self.file, self.row, self.col, out)
}
