package token

import (
	"fmt"
	"strings"

	"github.com/SnareChops/sat/types"
)

const (
	ASSIGN = iota
	EQUALITY
)

type Tokens struct {
	tokens []Token
	cursor int
}

func (self *Tokens) Debug(i int) string {
	contents := []string{}
	for _, token := range self.tokens {
		contents = append(contents, token.Debug(i+1))
	}
	return fmt.Sprintf("%sTokens(\n%s\n%s)", indent(i), strings.Join(contents, "\n"), indent(i))
}

func (self *Tokens) Add(token Token) {
	self.tokens = append(self.tokens, token)
}

func (self *Tokens) Token() (Token, bool) {
	return self.At(self.cursor)
}

func (self *Tokens) At(cursor int) (Token, bool) {
	if cursor >= 0 && cursor < len(self.tokens) {
		return self.tokens[cursor], true
	}
	return nil, false
}

func (self *Tokens) Prev() (Token, bool) {
	return self.At(self.cursor - 1)
}

func (self *Tokens) Next() (Token, bool) {
	return self.At(self.cursor + 1)
}

func (self *Tokens) Advance() {
	self.cursor += 1
}

func (self *Tokens) Len() int {
	return len(self.tokens)
}

func (self *Tokens) Loc() types.Location {
	tok, ok := self.Token()
	if !ok {
		return types.Location{}
	}
	return tok.Location()
}

func (self *Tokens) SplitOnFirstSpecial() (int, *Tokens, *Tokens) {
	i := 0
	for i < len(self.tokens) {
		if _, ok := self.tokens[i].(Special); ok {
			left := &Tokens{}
			right := &Tokens{}
			x := 0
			for x < i {
				left.Add(self.tokens[x])
				x++
			}
			x++
			for x < len(self.tokens) {
				right.Add(self.tokens[x])
				x++
			}
			return self.tokens[i].(Special).Value, left, right
		}
		i++
	}
	return -1, nil, nil
}

type Token interface {
	Location() types.Location
	Debug(indent int) string
}

type Symbol struct {
	Loc   types.Location
	Value string
}

func (self Symbol) Location() types.Location {
	return self.Loc
}

func indent(i int) string {
	out := ""
	for x := 0; x < i; x++ {
		out += "\t"
	}
	return out
}

func (self Symbol) Debug(i int) string {
	return fmt.Sprintf("%sSymbol(%s: %s)", indent(i), self.Loc.ToString(), self.Value)
}

type String struct {
	Loc   types.Location
	Value string
}

func (self String) Location() types.Location {
	return self.Loc
}

func (self String) Debug(i int) string {
	return fmt.Sprintf("%sString(%s: %s)", indent(i), self.Loc.ToString(), self.Value)
}

type Number struct {
	Loc   types.Location
	Value string
}

func (self Number) Location() types.Location {
	return self.Loc
}

func (self Number) Debug(i int) string {
	return fmt.Sprintf("%sNumber(%s: %s)", indent(i), self.Loc.ToString(), self.Value)
}

type Block struct {
	Loc    types.Location
	Tokens []*Tokens
}

func (self Block) Location() types.Location {
	return self.Loc
}

func (self Block) Debug(i int) string {
	children := []string{}
	for _, child := range self.Tokens {
		children = append(children, child.Debug(i+1))
	}
	return fmt.Sprintf("%sBlock(%s:\n%s\n%s)", indent(i), self.Loc.ToString(), strings.Join(children, "\n"), indent(i))
}

type Object struct {
	Loc   types.Location
	Props map[string]*Tokens
}

func (self Object) Location() types.Location {
	return self.Loc
}

func (self Object) Debug(i int) string {
	children := []string{}
	for key, value := range self.Props {
		children = append(children, fmt.Sprintf("%s%s: %s", indent(i+1), key, value.Debug(0)))
	}
	return fmt.Sprintf("%sObject(%s){\n%s\n%s}", indent(i), self.Loc.ToString(), strings.Join(children, "\n"), indent(i))
}

type Array struct {
	Loc   types.Location
	Items []*Tokens
}

func (self Array) Location() types.Location {
	return self.Loc
}

func (self Array) Debug(i int) string {
	children := []string{}
	for _, value := range self.Items {
		children = append(children, fmt.Sprintf("%s%s", indent(i+1), value.Debug(0)))
	}
	return fmt.Sprintf("%sArray(%s)[\n%s\n%s]", indent(i), self.Loc.ToString(), strings.Join(children, "\n"), indent(i))
}

type Special struct {
	Loc   types.Location
	Value int
}

func (self Special) Location() types.Location {
	return self.Loc
}

func (self Special) Debug(i int) string {
	value := ""
	switch self.Value {
	case ASSIGN:
		value = "="
	case EQUALITY:
		value = "=="
	}
	return fmt.Sprintf("%sSpecial(%s: %s)", indent(i), self.Loc.ToString(), value)
}

type Pipe struct {
	Loc    types.Location
	Tokens *Tokens
}

func (self Pipe) Location() types.Location {
	return self.Loc
}

func (self Pipe) Debug(i int) string {
	return fmt.Sprintf("%sPipe(%s\n%s\n%s)", indent(i), self.Loc.ToString(), self.Tokens.Debug(i+1), indent(i))
}

type Comma struct {
	Loc types.Location
}

func (self Comma) Location() types.Location {
	return self.Loc
}

func (self Comma) Debug(i int) string {
	return fmt.Sprintf("%sComma(%s)", indent(i), self.Loc.ToString())
}

type Dot struct {
	Loc types.Location
}

func (self Dot) Location() types.Location {
	return self.Loc
}

func (self Dot) Debug(i int) string {
	return fmt.Sprintf("%sDot(%s)", indent(i), self.Loc.ToString())
}

type Eol struct {
	Loc types.Location
}

func (self Eol) Location() types.Location {
	return self.Loc
}

func (self Eol) Debug(i int) string {
	return fmt.Sprintf("%sEol(%s)", indent(i), self.Loc.ToString())
}
