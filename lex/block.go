package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func lex_block(loc types.Location, reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_block\n\t%v\n---\n", reader.debug())
	}
	block := []*token.Tokens{}
	for {
		tokens, ending, err := lex_until(reader, []rune{'\n', ';', '}'})
		if err != nil {
			return nil, err
		}
		if tokens.Len() > 0 {
			block = append(block, tokens)
		}
		switch ending {
		case 0:
			return token.Block{loc, block}, nil
		case '\n':
			reader.advance()
		case ';':
			reader.advance()
		case '}':
			reader.advance()
			return token.Block{loc, block}, nil
		}
	}
}
