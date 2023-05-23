package run

import (
	"fmt"
	"net/http"
	"time"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_healthcheck(healthcheck expression.Healthcheck, scope *types.Scope) (primitive.Void, error) {
	if *options.Trace {
		fmt.Printf("run_healthcheck\n\t%v\n\t%v\n---\n", healthcheck, scope)
	}
	exp, err := run_expression(healthcheck.Url, scope)
	if err != nil {
		return primitive.Void{}, err
	}
	str, ok := exp.(primitive.String)
	if !ok {
		return primitive.Void{}, RunError(healthcheck.Loc, "expected string value as expressions result for healthcheck")
	}
	attempts := 0
	for attempts < 100 {
		res, err := http.Get(str.Value)
		if err == nil && res.StatusCode == 200 {
			return primitive.Void{}, nil
		}
		attempts += 1
		time.Sleep(1 * time.Second)
	}
	return primitive.Void{}, RunError(healthcheck.Loc, "healthcheck timed out")
}
