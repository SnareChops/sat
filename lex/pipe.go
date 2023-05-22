package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
)

func lex_pipe(reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_pipe\n\t%v\n---\n", reader.debug())
	}
	loc := reader.loc()
	reader.advance()
	tokens, _, err := lex_until(reader, []rune{';', '\n'})
	if err != nil {
		return nil, err
	}
	return token.Pipe{loc, tokens}, nil
}
