package expression

import (
	"fmt"
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

type Primitive struct {
	Loc   types.Location
	Value types.Primitive
}

func (self Primitive) Location() types.Location {
	return self.Loc
}

func (self Primitive) Debug(i int) string {
	return fmt.Sprintf("%sPrimitive(%s: %s)", indent(i), self.Loc.ToString(), self.Value.Debug(0))
}

type Assignment struct {
	Loc   types.Location
	Left  types.Expression
	Right types.Expression
}

func (self Assignment) Location() types.Location {
	return self.Loc
}

func (self Assignment) Debug(i int) string {
	return fmt.Sprintf("%sAssignment(%s\n%s\n%s=\n%s\n%s)", indent(i), self.Loc.ToString(), self.Left.Debug(i+1), indent(i+1), self.Right.Debug(i+1), indent(i))
}

type Equality struct {
	Loc   types.Location
	Left  types.Expression
	Right types.Expression
}

func (self Equality) Location() types.Location {
	return self.Loc
}

func (self Equality) Debug(i int) string {
	return fmt.Sprintf("%sEquality(%s\n%s\n%s==\n%s\n%s)", indent(i), self.Loc.ToString(), self.Left.Debug(i+1), indent(i+1), self.Right.Debug(i+1), indent(i))
}

type Ref struct {
	Loc   types.Location
	Value string
}

func (self Ref) Location() types.Location {
	return self.Loc
}

func (self Ref) Debug(i int) string {
	return fmt.Sprintf("%sRef(%s %s)", indent(i), self.Loc.ToString(), self.Value)
}

type Assert struct {
	Loc   types.Location
	Value types.Expression
}

func (self Assert) Location() types.Location {
	return self.Loc
}

func (self Assert) Debug(i int) string {
	return fmt.Sprintf("%sAssert(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Value.Debug(i+1), indent(i))
}

type Test struct {
	Loc   types.Location
	Name  string
	Block Block
}

func (self Test) Location() types.Location {
	return self.Loc
}

func (self Test) Debug(i int) string {
	return fmt.Sprintf("%sTest(%s %s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Name, self.Block.Debug(i+1), indent(i))
}

type MultiRef struct {
	Loc  types.Location
	Refs []string
}

func (self MultiRef) Location() types.Location {
	return self.Loc
}

func (self MultiRef) Debug(i int) string {
	items := []string{}
	for _, ref := range self.Refs {
		items = append(items, fmt.Sprintf("%s%s", indent(i+1), ref))
	}
	return fmt.Sprintf("%sMultiRef(%s\n%s\n%s)", indent(i), self.Loc.ToString(), strings.Join(items, "\n"), indent(i))
}

type Get struct {
	Loc types.Location
	Url types.Expression
}

func (self Get) Location() types.Location {
	return self.Loc
}

func (self Get) Debug(i int) string {
	return fmt.Sprintf("%sGet(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Url.Debug(i+1), indent(i))
}

type Run struct {
	Loc     types.Location
	Command types.Expression
}

func (self Run) Location() types.Location {
	return self.Loc
}

func (self Run) Debug(i int) string {
	return fmt.Sprintf("%sRun(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Command.Debug(i+1), indent(i))
}

type Spawn struct {
	Loc     types.Location
	Command types.Expression
}

func (self Spawn) Location() types.Location {
	return self.Loc
}

func (self Spawn) Debug(i int) string {
	return fmt.Sprintf("%sSpawn(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Command.Debug(i+1), indent(i))
}

type Kill struct {
	Loc    types.Location
	Handle types.Expression
}

func (self Kill) Location() types.Location {
	return self.Loc
}

func (self Kill) Debug(i int) string {
	return fmt.Sprintf("%sKill(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Handle.Debug(i+1), indent(i))
}

// ////////////////////
type Block struct {
	Loc      types.Location
	Contents []types.Expression
	Scope    *types.Scope
}

func NewBlock(loc types.Location, contents []types.Expression, parent *types.Scope) Block {
	return Block{
		Loc:      loc,
		Contents: contents,
		Scope:    types.NewScope(parent),
	}
}

func (self Block) Debug(i int) string {
	contents := []string{}
	for _, exp := range self.Contents {
		contents = append(contents, exp.Debug(i+1))
	}
	return fmt.Sprintf("%sBlock(%s\n%s\n%s)", indent(i), self.Loc.ToString(), strings.Join(contents, "\n"), indent(i))
}
