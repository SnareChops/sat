package parse_test

import (
	"testing"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/parse"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
	"github.com/stretchr/testify/assert"
)

func TestAssert(t *testing.T) {
	test := token.Symbol{types.Location{"file", 1, 1}, "test"}
	name := token.String{types.Location{"file", 1, 6}, "name"}
	ass := token.Symbol{types.Location{"file", 2, 4}, "assert"}
	bool := token.Symbol{types.Location{"file", 2, 13}, "true"}

	tokens := &token.Tokens{}
	tokens.Add(bool)
	pipe := token.Pipe{types.Location{"file", 2, 10}, tokens}

	tokens = &token.Tokens{}
	tokens.Add(ass)
	tokens.Add(pipe)
	block := token.Block{types.Location{"file", 1, 11}, []*token.Tokens{tokens}}

	tokens = &token.Tokens{}
	tokens.Add(test)
	tokens.Add(name)
	tokens.Add(block)

	program, err := parse.Parse([]*token.Tokens{tokens})
	assert.Nil(t, err)

	assert.Equal(t, 1, len(program.Expressions))
	expressions := program.Expressions
	exp, ok := expressions[0].(expression.Test)
	assert.True(t, ok)
	loc := exp.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "name", exp.Name)

	assert.Equal(t, 1, len(exp.Block.Contents))
	axp, ok := exp.Block.Contents[0].(expression.Assert)
	assert.True(t, ok)
	loc = axp.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 4, loc.Col)

	pxp, ok := axp.Value.(expression.Primitive)
	assert.True(t, ok)
	loc = pxp.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 13, loc.Col)
	prim, ok := pxp.Value.(primitive.Boolean)
	assert.True(t, ok)
	assert.Equal(t, true, prim.Value)
}
