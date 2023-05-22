package lex

import (
	"fmt"
	"unicode"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func lex_object(loc types.Location, reader *Reader) (token.Token, error) {
	if *options.Trace {
		fmt.Printf("lex_object\n\t%v\n---\n", reader.debug())
	}
	object := map[string]*token.Tokens{}
	prop := ""
	for {
		if char, ok := reader.rune(); ok {
			if char == '"' {
				t, err := lex_string(reader)
				if err != nil {
					return nil, err
				}
				prop = t.(token.String).Value
			} else if unicode.IsLetter(char) || unicode.IsDigit(char) {
				t, err := lex_symbol(reader)
				if err != nil {
					return nil, err
				}
				prop = t.(token.Symbol).Value
			} else if char == ':' {
				reader.advance()
				tokens, ending, err := lex_until(reader, []rune{'}', ','})
				if err != nil {
					return nil, err
				}
				object[prop] = tokens
				switch ending {
				case ',':
					reader.advance()
				case '}':
					reader.advance()
					return token.Object{loc, object}, nil
				}
			} else {
				reader.advance()
			}
		}
	}
}
