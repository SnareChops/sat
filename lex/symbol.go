package lex

import (
	"fmt"
	"unicode"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
)

func lex_symbol(reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_symbol\n\t%v\n---\n", reader.debug())
	}
	loc := reader.loc()
	symbol := []rune{}
	for {
		if char, ok := reader.rune(); ok && (unicode.IsLetter(char) || unicode.IsNumber(char) || char == '_') {
			symbol = append(symbol, char)
			reader.advance()
		} else {
			return token.Symbol{loc, string(symbol)}, nil
		}
	}
}
