package run

import (
	"encoding/json"
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
	var result types.Primitive
	result = primitive.String{string(body)}
	if body[0] == '{' {
		unknown := map[string]interface{}{}
		json.Unmarshal(body, &unknown)
		r, err := parse_json_object(get.Loc, unknown)
		if err != nil {
			return nil, err
		}
		result = r
	} else if body[0] == '[' {
		unknown := []interface{}{}
		json.Unmarshal(body, &unknown)
		r, err := parse_json_array(get.Loc, unknown)
		if err != nil {
			return nil, err
		}
		result = r
	}

	return primitive.Multi{[]types.Primitive{
		primitive.Number{float32(res.StatusCode)},
		result,
	}}, err
}

func parse_json_object(loc types.Location, object map[string]interface{}) (primitive.Object, error) {
	prims := map[string]types.Expression{}
	for key, value := range object {
		parsed, err := parse_unknown(loc, value)
		if err != nil {
			return primitive.Object{}, err
		}
		prims[key] = expression.Primitive{types.Location{}, parsed}
	}
	return primitive.Object{prims}, nil
}

func parse_json_array(loc types.Location, array []interface{}) (primitive.Array, error) {
	prims := []types.Expression{}
	for _, value := range array {
		parsed, err := parse_unknown(loc, value)
		if err != nil {
			return primitive.Array{}, err
		}
		prims = append(prims, expression.Primitive{types.Location{}, parsed})
	}
	return primitive.Array{prims}, nil
}

func parse_unknown(loc types.Location, unknown interface{}) (types.Primitive, error) {
	switch unknown.(type) {
	case map[string]interface{}:
		return parse_json_object(loc, unknown.(map[string]interface{}))
	case []interface{}:
		return parse_json_array(loc, unknown.([]interface{}))
	case string:
		return primitive.String{unknown.(string)}, nil
	case int:
		return primitive.Number{float32(unknown.(int))}, nil
	case int64:
		return primitive.Number{float32(unknown.(int64))}, nil
	case float32:
		return primitive.Number{unknown.(float32)}, nil
	case float64:
		return primitive.Number{float32(unknown.(float64))}, nil
	case bool:
		return primitive.Boolean{unknown.(bool)}, nil
	default:
		return nil, RunError(loc, "unexpected value while parsing json")
	}

}
