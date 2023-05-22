package run_test

import (
	"testing"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/run"
	"github.com/SnareChops/sat/types"
	"github.com/stretchr/testify/assert"
)

func TestRef(t *testing.T) {
	loc := types.Location{"file", 4, 3}
	scope := types.NewScope(nil)
	scope.Vars["bool"] = primitive.Boolean{true}
	val := map[string]types.Expression{}
	val["hello"] = expression.Primitive{loc, primitive.String{"world"}}
	ref := expression.Ref{loc, "bool"}

	prim, err := run.RunRef(ref, scope)
	assert.Nil(t, err)
	b, ok := prim.(primitive.Boolean)
	assert.True(t, ok)
	assert.True(t, b.Value)

	scope.Vars["obj"] = primitive.Object{val}

	ref = expression.Ref{loc, "obj.hello"}
	prim, err = run.RunRef(ref, scope)
	assert.Nil(t, err)
	s, ok := prim.(primitive.String)
	assert.True(t, ok)
	assert.Equal(t, "world", s.Value)
}

func TestAssignment(t *testing.T) {
	loc := types.Location{"file", 4, 7}
	left := expression.Ref{loc, "hello"}
	right := expression.Primitive{loc, primitive.String{"world"}}
	scope := types.NewScope(nil)
	exp := expression.Assignment{loc, left, right}
	prim, err := run.RunAssignment(exp, scope)
	assert.Nil(t, err)
	str, ok := prim.(primitive.String)
	assert.True(t, ok)
	assert.Equal(t, "world", str.Value)
}

func TestAssert(t *testing.T) {
	loc := types.Location{"file", 3, 5}
	number := expression.Primitive{loc, primitive.Number{2.3}}
	equality := expression.Equality{loc, number, number}
	ass := expression.Assert{loc, equality}
	scope := types.NewScope(nil)

	prim, err := run.RunAssert(ass, scope)
	assert.Nil(t, err)
	b, ok := prim.(primitive.Boolean)
	assert.True(t, ok)
	assert.Equal(t, true, b.Value)

}
