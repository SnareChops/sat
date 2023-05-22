package lex

import (
	"fmt"
	"unicode"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
)

func lex_number(reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_number\n\t%v\n---\n", reader.debug())
	}
	loc := reader.loc()
	number := []rune{}
	for {
		if char, ok := reader.rune(); ok {
			if unicode.IsNumber(char) || char == '.' {
				number = append(number, char)
				reader.advance()
			} else if char == '_' {
				reader.advance()
			} else {
				return token.Number{loc, string(number)}, nil
			}
		} else {
			return token.Number{loc, string(number)}, nil
		}
	}
}
