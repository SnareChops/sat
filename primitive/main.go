package primitive

import (
	"fmt"
	"os/exec"
	"strings"

	"github.com/SnareChops/sat/types"
)

func indent(i int) string {
	out := ""
	for x := 0; x < i; x++ {
		out += "\t"
	}
	return out
}

type Void struct{}

func (self Void) Debug(i int) string {
	return fmt.Sprintf("%sVoid", indent(i))
}

type Boolean struct {
	Value bool
}

func (self Boolean) Debug(i int) string {
	return fmt.Sprintf("%sBoolean(%t)", indent(i), self.Value)
}

type Number struct {
	Value float32
}

func (self Number) Debug(i int) string {
	return fmt.Sprintf("%sNumber(%f)", indent(i), self.Value)
}

type String struct {
	Value string
}

func (self String) Debug(i int) string {
	return fmt.Sprintf("%sString(%s)", indent(i), self.Value)
}

type Object struct {
	Value map[string]types.Expression
}

func (self Object) Debug(i int) string {
	out := []string{}
	for key, value := range self.Value {
		out = append(out, fmt.Sprintf("%s%s: %s", indent(i+1), key, value.Debug(0)))
	}
	return fmt.Sprintf("%sObject(\n%s\n%s)", indent(i), strings.Join(out, "\n"), indent(i))
}

type Array struct {
	Value []types.Expression
}

func (self Array) Debug(i int) string {
	out := []string{}
	for _, value := range self.Value {
		out = append(out, fmt.Sprintf("%s%s", indent(i+1), value.Debug(0)))
	}
	return fmt.Sprintf("%sArray(\n%s\n%s)", indent(i), strings.Join(out, "\n"), indent(i))
}

type Multi struct {
	Value []types.Primitive
}

func (self Multi) Debug(i int) string {
	out := []string{}
	for _, value := range self.Value {
		out = append(out, fmt.Sprintf("%s%s", indent(i+1), value.Debug(0)))
	}
	return fmt.Sprintf("%sMulti(\n%s\n%s)", indent(i), strings.Join(out, "\n"), indent(i))
}

type Process struct {
	Value *exec.Cmd
}

func (self Process) Debug(i int) string {
	return fmt.Sprintf("%sProcess(%v)", indent(i), self.Value)
}
