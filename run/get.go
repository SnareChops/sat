package run

import (
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_get(get expression.Get, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_get\n\t%v\n\t%v\n---\n", get, scope)
	}
	exp, err := run_expression(get.Url, scope)
	if err != nil {
		return nil, err
	}
	str, ok := exp.(primitive.String)
	if !ok {
		return nil, RunError(get.Loc, "expected string value as expression result for get")
	}
	res, err := http.Get(str.Value)
	if err != nil {
		return nil, RunError(get.Loc, err.Error())
	}
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		return nil, RunError(get.Loc, err.Error())
	}
	return primitive.Multi{[]types.Primitive{
		primitive.Number{float32(res.StatusCode)},
		primitive.String{string(body)},
	}}, err
}
