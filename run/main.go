package run

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os/exec"
	"runtime"
	"strconv"
	"strings"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

type Feedback struct {
	output []string
}

func (self *Feedback) addTestPassed(name string) {
	self.output = append(self.output, fmt.Sprintf("\t✓\t%s", name))
}

func (self *Feedback) addTestFailed(name string, failed []types.Location) {
	asserts := ""
	for _, fail := range failed {
		asserts += fmt.Sprintf("\n\t\t- assert %s", fail.ToString())
	}
	self.output = append(self.output, fmt.Sprintf("\t❌\t%s%s", name, asserts))
}

func (self *Feedback) ToString() string {
	return strings.Join(self.output, "\n")
}

func RunError(loc types.Location, message string) error {
	return fmt.Errorf("%s: %s", loc.ToString(), message)
}

func Run(program types.Program) (*Feedback, error) {
	if *options.Trace {
		fmt.Printf("run\n\t%v\n---\n", program)
	}
	feedback := &Feedback{}
	for _, exp := range program.Expressions {
		var err error
		switch exp.(type) {
		case expression.Assignment:
			_, err = RunAssignment(exp.(expression.Assignment), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Test:
			err = run_test(feedback, exp.(expression.Test))
			if err != nil {
				return nil, err
			}
		case expression.Assert:
			_, err = RunAssert(exp.(expression.Assert), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Get:
			_, err = run_get(exp.(expression.Get), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Run:
			_, err = run_command(exp.(expression.Run), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Spawn:
			return nil, RunError(exp.Location(), "spawn at the root level must be used in an assignment")
			// _, err = run_spawn(exp.(expression.Spawn), program.Scope)
		case expression.Kill:
			_, err = run_kill(exp.(expression.Kill), program.Scope)
			if err != nil {
				return nil, err
			}
		default:
			return nil, fmt.Errorf("unexpected expression type")
		}
		if err != nil {
			return nil, err
		}
	}
	return feedback, nil
}

func run_expression(exp types.Expression, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_expression\n\t%v\n---\n", exp)
	}
	switch exp.(type) {
	case expression.Assignment:
		return RunAssignment(exp.(expression.Assignment), scope)
	case expression.Primitive:
		return exp.(expression.Primitive).Value, nil
	case expression.Equality:
		return run_equality(exp.(expression.Equality), scope)
	case expression.Ref:
		return RunRef(exp.(expression.Ref), scope)
	case expression.Assert:
		return RunAssert(exp.(expression.Assert), scope)
	case expression.Test:
		panic("")
	case expression.MultiRef:
		panic("")
	case expression.Get:
		return run_get(exp.(expression.Get), scope)
	case expression.Run:
		return run_command(exp.(expression.Run), scope)
	case expression.Spawn:
		return run_spawn(exp.(expression.Spawn), scope)
	case expression.Kill:
		return run_kill(exp.(expression.Kill), scope)
	default:
		return nil, RunError(exp.Location(), "unknown expression")
	}
}

func RunAssignment(assert expression.Assignment, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_asssignment\n\t%v\n---\n", assert)
	}
	switch assert.Left.(type) {
	case expression.Ref:
		ref := assert.Left.(expression.Ref)
		prim, err := run_expression(assert.Right, scope)
		if err != nil {
			return nil, err
		}
		scope.Vars[ref.Value] = prim
		return prim, nil
	case expression.MultiRef:
		multi := assert.Left.(expression.MultiRef)
		prim, err := run_expression(assert.Right, scope)
		if err != nil {
			return nil, err
		}
		values, ok := prim.(primitive.Multi)
		if !ok {
			return nil, RunError(assert.Right.Location(), "expected multi value expression")
		}
		if len(multi.Refs) > len(values.Value) {
			return nil, RunError(assert.Left.Location(), "expression returns fewer values than variables specified")
		}
		for i, ref := range multi.Refs {
			scope.Vars[ref] = values.Value[i]
		}
		return values.Value[0], nil
	default:
		return nil, RunError(assert.Left.Location(), "invalid assignment")
	}
}

func run_test(feedback *Feedback, exp expression.Test) error {
	if *options.Trace {
		fmt.Printf("run_test\n\t%v\n---\n", exp)
	}
	for _, e := range exp.Block.Contents {
		_, err := run_expression(e, exp.Block.Scope)
		if err != nil {
			return err
		}
	}
	failed := []types.Location{}
	for _, ass := range exp.Block.Scope.Asserts {
		if !ass.Passed {
			failed = append(failed, ass.Loc)
		}
	}
	if len(failed) > 0 {
		feedback.addTestFailed(exp.Name, failed)
	} else {
		feedback.addTestPassed(exp.Name)
	}
	return nil
}

func RunAssert(assert expression.Assert, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_assert\n\t%v\n\t%v\n---\n", assert, scope)
	}
	prim, err := run_expression(assert.Value, scope)
	if err != nil {
		return nil, err
	}
	b, ok := prim.(primitive.Boolean)
	if !ok {
		return nil, RunError(assert.Loc, "expected bool value as expression result for assert")
	}
	scope.Asserts = append(scope.Asserts, types.AssertResult{assert.Loc, b.Value})
	return b, nil
}

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

func run_command(exp expression.Run, scope *types.Scope) (primitive.Number, error) {
	if *options.Trace {
		fmt.Printf("run_command\n\t%v\n\t%v\n---\n", exp, scope)
	}
	prim, err := run_expression(exp.Command, scope)
	if err != nil {
		return primitive.Number{}, err
	}
	str, ok := prim.(primitive.String)
	if !ok {
		return primitive.Number{}, RunError(exp.Loc, "expected string value for run expression")
	}
	var cmd *exec.Cmd
	if runtime.GOOS == "windows" {
		cmd = exec.Command("cmd", "/C", str.Value)
	} else {
		cmd = exec.Command("sh", "-c", str.Value)
	}
	err = cmd.Run()
	if err != nil {
		return primitive.Number{float32(err.(*exec.ExitError).ExitCode())}, RunError(exp.Loc, err.Error())
	}
	return primitive.Number{0.}, nil
}

func run_spawn(exp expression.Spawn, scope *types.Scope) (primitive.Process, error) {
	if *options.Trace {
		fmt.Printf("run_spawn\n\t%v\n\t%v\n---\n", exp, scope)
	}
	prim, err := run_expression(exp.Command, scope)
	if err != nil {
		return primitive.Process{}, err
	}
	str, ok := prim.(primitive.String)
	if !ok {
		return primitive.Process{}, RunError(exp.Loc, "expected string value for run expression")
	}
	var cmd *exec.Cmd
	if runtime.GOOS == "windows" {
		cmd = exec.Command("cmd", "/C", str.Value)
	} else {
		cmd = exec.Command("sh", "-c", str.Value)
	}
	err = cmd.Start()
	if err != nil {
		return primitive.Process{}, RunError(exp.Loc, err.Error())
	}
	return primitive.Process{cmd}, nil
}

func run_kill(exp expression.Kill, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_kill\n\t%v\n\t%v\n---\n", exp, scope)
	}
	prim, err := run_expression(exp, scope)
	if err != nil {
		return nil, err
	}
	proc, ok := prim.(primitive.Process)
	if !ok {
		return nil, RunError(exp.Location(), "expected process reference for kill expression")
	}
	err = proc.Value.Process.Kill()
	if err != nil {
		return nil, RunError(exp.Location(), err.Error())
	}
	return primitive.Void{}, nil
}

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

func RunRef(exp expression.Ref, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_ref\n\t%v\n\t%v\n---\n", exp, scope)
	}
	if !strings.Contains(exp.Value, ".") {
		if val, ok := scope.Vars[exp.Value]; ok {
			return val, nil
		}
		panic("todo")
	} else {
		parts := strings.Split(exp.Value, ".")
		first := parts[0]
		next := parts[1]
		if val, ok := scope.Vars[first]; ok {
			switch val.(type) {
			case primitive.Object:
				return run_expression(val.(primitive.Object).Value[next], scope)
			case primitive.Array:
				index, err := strconv.ParseInt(next, 10, 64)
				if err != nil {
					return nil, err
				}
				if len(val.(primitive.Array).Value) < int(index) {
					return nil, RunError(exp.Location(), "array index ref out of range")
				}
				return run_expression(val.(primitive.Array).Value[index], scope)
			}
		} else {
			return nil, RunError(exp.Loc, "expression")
		}
	}
	panic("")
}
