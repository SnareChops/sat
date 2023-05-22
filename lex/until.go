package lex

import (
	"fmt"
	"unicode"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"golang.org/x/exp/slices"
)

func lex_until(reader *Reader, until []rune) (*token.Tokens, rune, error) {
	if *options.Trace {
		fmt.Printf("lex_until\n\t%v\n\t%v\n---\n", until, reader.debug())
	}
	tokens := &token.Tokens{}
	for char, ok := reader.rune(); ok; char, ok = reader.rune() {
		if slices.Contains(until, char) {
			// reader.advance()
			return tokens, char, nil
		}
		if char == '\n' || char == ';' {
			reader.advance()
			tokens.Add(token.Eol{reader.loc()})
		} else if char == ':' {
			token, err := lex_pipe(reader)
			if err != nil {
				return nil, '\000', err
			}
			tokens.Add(token)
		} else if char == '=' {
			loc := reader.loc()
			reader.advance()
			if char, ok := reader.rune(); ok && char == '=' {
				tokens.Add(token.Special{loc, token.EQUALITY})
				reader.advance()
				continue
			}
			tokens.Add(token.Special{loc, token.ASSIGN})
		} else if char == '.' {
			tokens.Add(token.Dot{reader.loc()})
			reader.advance()
		} else if char == ',' {
			tokens.Add(token.Comma{reader.loc()})
			reader.advance()
		} else if char == '"' {
			token, err := lex_string(reader)
			if err != nil {
				return nil, '\000', err
			}
			tokens.Add(token)
		} else if char == '{' {
			loc := reader.loc()
			reader.advance()
			if last, ok := tokens.At(tokens.Len() - 1); ok {
				switch last.(type) {
				case token.Special:
					token, err := lex_object(loc, reader)
					if err != nil {
						return nil, '\000', err
					}
					tokens.Add(token)
				case token.Pipe:
					token, err := lex_object(loc, reader)
					if err != nil {
						return nil, '\000', err
					}
					tokens.Add(token)
				default:
					token, err := lex_block(loc, reader)
					if err != nil {
						return nil, '\000', err
					}
					tokens.Add(token)
				}
			} else {
				token, err := lex_block(loc, reader)
				if err != nil {
					return nil, '\000', err
				}
				tokens.Add(token)
			}
		} else if char == '[' {
			token, err := lex_array(reader)
			if err != nil {
				return nil, '\000', err
			}
			tokens.Add(token)
		} else if unicode.IsLetter(char) || char == '_' {
			token, err := lex_symbol(reader)
			if err != nil {
				return nil, '\000', err
			}
			tokens.Add(token)
		} else if unicode.IsNumber(char) {
			token, err := lex_number(reader)
			if err != nil {
				return nil, '\000', err
			}
			tokens.Add(token)
		} else {
			reader.advance()
		}
	}
	return tokens, '\000', nil
}
