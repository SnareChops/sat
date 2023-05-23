package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_equality(exp expression.Equality, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_equality\n\t%v\n\t%v\n---\n", exp, scope)
	}
	left_prim, err := run_expression(exp.Left, scope)
	if err != nil {
		return nil, err
	}
	right_prim, err := run_expression(exp.Right, scope)
	if err != nil {
		return nil, err
	}
	switch left_prim.(type) {
	case primitive.Void:
		switch right_prim.(type) {
		case primitive.Void:
			return primitive.Boolean{true}, nil
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.Boolean:
		switch right_prim.(type) {
		case primitive.Boolean:
			return primitive.Boolean{left_prim.(primitive.Boolean).Value == right_prim.(primitive.Boolean).Value}, nil
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.Number:
		switch right_prim.(type) {
		case primitive.Number:
			return primitive.Boolean{left_prim.(primitive.Number).Value == right_prim.(primitive.Number).Value}, nil
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.String:
		switch right_prim.(type) {
		case primitive.String:
			return primitive.Boolean{left_prim.(primitive.String).Value == right_prim.(primitive.String).Value}, nil
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.Object:
		panic("todo")
	case primitive.Array:
		panic("todo")
	case primitive.Multi:
		panic("todo")
	case primitive.Process:
		return primitive.Boolean{false}, nil
	default:
		panic("unknown equality comparison")
	}
}
