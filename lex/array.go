package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
)

func lex_array(reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_array\n\t%v\n---\n", reader.debug())
	}
	loc := reader.loc()
	array := []*token.Tokens{}
	reader.advance()
	for {
		tokens, ending, err := lex_until(reader, []rune{',', ']'})
		if err != nil {
			return nil, err
		}
		array = append(array, tokens)
		switch ending {
		case ',':
			reader.advance()
		case ']':
			reader.advance()
			return token.Array{loc, array}, nil
		}
	}
}
