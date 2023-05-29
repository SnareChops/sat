package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_equality(exp expression.Equality, scope *types.Scope) (primitive.Boolean, error) {
	if *options.Trace {
		fmt.Printf("run_equality\n\t%v\n\t%v\n---\n", exp, scope)
	}
	left_prim, err := run_expression(exp.Left, scope)
	if err != nil {
		return primitive.Boolean{}, err
	}
	right_prim, err := run_expression(exp.Right, scope)
	if err != nil {
		return primitive.Boolean{}, err
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
		switch right_prim.(type) {
		case primitive.Object:
			return object_equality(left_prim.(primitive.Object), right_prim.(primitive.Object), scope)
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.Array:
		switch right_prim.(type) {
		case primitive.Array:
			return array_equality(left_prim.(primitive.Array), right_prim.(primitive.Array), scope)
		default:
			return primitive.Boolean{false}, nil
		}
	case primitive.Multi:
		panic("todo")
	case primitive.Process:
		return primitive.Boolean{false}, nil
	default:
		panic("unknown equality comparison")
	}
}

func object_equality(left, right primitive.Object, scope *types.Scope) (primitive.Boolean, error) {
	if len(left.Value) != len(right.Value) {
		return primitive.Boolean{false}, nil
	}
	for key, lvalue := range left.Value {
		rvalue, ok := right.Value[key]
		if !ok {
			return primitive.Boolean{false}, nil
		}
		b, err := run_equality(expression.Equality{lvalue.Location(), lvalue, rvalue}, scope)
		if err != nil {
			return primitive.Boolean{}, err
		}
		if !b.Value {
			return primitive.Boolean{false}, nil
		}
	}
	return primitive.Boolean{true}, nil
}

func array_equality(left, right primitive.Array, scope *types.Scope) (primitive.Boolean, error) {
	if len(left.Value) != len(right.Value) {
		for i, lvalue := range left.Value {
			rvalue := right.Value[i]
			b, err := run_equality(expression.Equality{lvalue.Location(), lvalue, rvalue}, scope)
			if err != nil {
				return primitive.Boolean{}, err
			}
			if !b.Value {
				return primitive.Boolean{false}, nil
			}
		}
	}
	return primitive.Boolean{true}, nil
}
