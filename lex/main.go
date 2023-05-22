package lex

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func LexError(loc types.Location, message string) error {
	return fmt.Errorf("%s: %s", loc.ToString(), message)
}

func File(file, contents string) ([]*token.Tokens, error) {
	if *options.Trace {
		fmt.Printf("lex.File\n\t%v\n\t%v\n---\n", file, contents)
	}
	reader := NewReader(file, contents)
	block, err := lex_block(reader.loc(), reader)
	if err != nil {
		return nil, err
	}
	return block.(token.Block).Tokens, nil
}
