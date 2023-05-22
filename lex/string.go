package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
)

func lex_string(reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_string\n\t%v\n---\n", reader.debug())
	}
	loc := reader.loc()
	result := []rune{}
	reader.advance()
	for {
		char, ok := reader.rune()
		if !ok {
			panic("")
		}
		if char == '\\' {
			reader.advance()
			if char, ok := reader.rune(); ok {
				reader.advance()
				result = append(result, char)
			} else {
				panic("")
			}
		} else if char == '"' {
			reader.advance()
			return token.String{loc, string(result)}, nil
		} else {
			reader.advance()
			result = append(result, char)
		}
	}
}
