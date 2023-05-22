package parse

import (
	"fmt"
	"strconv"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_number(tokens *token.Tokens) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_number\n\t%v\n---\n", tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected number")
	}
	tokens.Advance()
	nok := tok.(token.Number)
	number, err := strconv.ParseFloat(nok.Value, 32)
	if err != nil {
		return nil, err
	}
	return expression.Primitive{nok.Loc, primitive.Number{float32(number)}}, nil
}
