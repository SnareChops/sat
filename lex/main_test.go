package lex_test

import (
	"testing"

	"github.com/SnareChops/sat/lex"
	"github.com/SnareChops/sat/token"
	"github.com/stretchr/testify/assert"
)

func TestString(t *testing.T) {
	tokens, err := lex.File("file", " \"some 123 \\\"string\\\" \n34\"")
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 1, tokens[0].Len())
	kok, ok := tokens[0].Token()
	assert.True(t, ok)
	tok, ok := kok.(token.String)
	assert.True(t, ok)
	loc := tok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 2, loc.Col)
	assert.Equal(t, "some 123 \"string\" \n34", tok.Value)
}

func TestNumber(t *testing.T) {
	tokens, err := lex.File("file", "123.456")
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 1, tokens[0].Len())
	kok, ok := tokens[0].Token()
	assert.True(t, ok)
	tok, ok := kok.(token.Number)
	assert.True(t, ok)
	loc := tok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "123.456", tok.Value)

	tokens, err = lex.File("file", " 1_000_000")
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 1, tokens[0].Len())
	kok, ok = tokens[0].Token()
	assert.True(t, ok)
	tok, ok = kok.(token.Number)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 2, loc.Col)
	assert.Equal(t, "1000000", tok.Value)
}

func TestNewlineIssue(t *testing.T) {
	contents := "\na"
	tokens, err := lex.File("file", contents)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 1, tokens[0].Len())
	kok, ok := tokens[0].Token()
	assert.True(t, ok)
	assert.Equal(t, "a", kok.(token.Symbol).Value)
}

func TestObject(t *testing.T) {
	file := "file"
	contents := "test {\nobj = {hello: \"world\", 1: 23.0, \"other\": true == true}\na=b\n}"
	tokens, err := lex.File(file, contents)
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 2, tokens[0].Len())

	kok, ok := tokens[0].At(0)
	assert.True(t, ok)
	tok, ok := kok.(token.Symbol)
	assert.True(t, ok)
	loc := tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "test", tok.Value)

	kok, ok = tokens[0].At(1)
	assert.True(t, ok)
	bok := kok.(token.Block)
	assert.True(t, ok)
	loc = bok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 6, loc.Col)
	assert.Equal(t, 2, len(bok.Tokens))
	assert.Equal(t, 3, bok.Tokens[0].Len())

	kok, ok = bok.Tokens[0].At(0)
	assert.True(t, ok)
	tok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "obj", tok.Value)

	kok, ok = bok.Tokens[0].At(1)
	assert.True(t, ok)
	pok, ok := kok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 5, loc.Col)
	assert.Equal(t, token.ASSIGN, pok.Value)

	kok, ok = bok.Tokens[0].At(2)
	assert.True(t, ok)
	ook, ok := kok.(token.Object)
	assert.True(t, ok)
	loc = ook.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 7, loc.Col)
	assert.Equal(t, 3, len(ook.Props))
	assert.Equal(t, 1, ook.Props["hello"].Len())
	assert.Equal(t, 1, ook.Props["1"].Len())
	assert.Equal(t, 3, ook.Props["other"].Len())

	kok, ok = ook.Props["hello"].At(0)
	assert.True(t, ok)
	sok, ok := kok.(token.String)
	assert.True(t, ok)
	loc = sok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 15, loc.Col)
	assert.Equal(t, "world", sok.Value)

	kok, ok = ook.Props["1"].At(0)
	assert.True(t, ok)
	nok, ok := kok.(token.Number)
	assert.True(t, ok)
	loc = nok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 27, loc.Col)
	assert.Equal(t, "23.0", nok.Value)

	kok, ok = ook.Props["other"].At(0)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 42, loc.Col)
	assert.Equal(t, "true", tok.Value)

	kok, ok = ook.Props["other"].At(1)
	assert.True(t, ok)
	pok, ok = kok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 47, loc.Col)
	assert.Equal(t, token.EQUALITY, pok.Value)

	kok, ok = ook.Props["other"].At(2)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 50, loc.Col)
	assert.Equal(t, "true", tok.Value)

	assert.Equal(t, 3, bok.Tokens[1].Len())
	kok, ok = bok.Tokens[1].At(0)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 3, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "a", tok.Value)

	kok, ok = bok.Tokens[1].At(1)
	assert.True(t, ok)
	pok, ok = kok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 3, loc.Row)
	assert.Equal(t, 2, loc.Col)
	assert.Equal(t, token.ASSIGN, pok.Value)

	kok, ok = bok.Tokens[1].At(2)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 3, loc.Row)
	assert.Equal(t, 3, loc.Col)
	assert.Equal(t, "b", tok.Value)
}

func TestArray(t *testing.T) {
	file := "file"
	tokens, err := lex.File(file, "array = [\"haha\", 45.6, false, true == true]")
	assert.Nil(t, err)

	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 3, tokens[0].Len())
	kok, ok := tokens[0].At(0)
	assert.True(t, ok)
	tok, ok := kok.(token.Symbol)
	assert.True(t, ok)
	loc := tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "array", tok.Value)

	kok, ok = tokens[0].At(1)
	assert.True(t, ok)
	pok, ok := kok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 7, loc.Col)
	assert.Equal(t, token.ASSIGN, pok.Value)

	kok, ok = tokens[0].At(2)
	assert.True(t, ok)
	aok, ok := kok.(token.Array)
	assert.True(t, ok)
	loc = aok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 9, loc.Col)
	assert.Equal(t, 4, len(aok.Items))

	assert.Equal(t, 1, aok.Items[0].Len())
	kok, ok = aok.Items[0].At(0)
	assert.True(t, ok)
	sok, ok := kok.(token.String)
	assert.True(t, ok)
	loc = sok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 10, loc.Col)
	assert.Equal(t, "haha", sok.Value)

	assert.Equal(t, 1, aok.Items[1].Len())
	kok, ok = aok.Items[1].At(0)
	assert.True(t, ok)
	nok, ok := kok.(token.Number)
	assert.True(t, ok)
	loc = nok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 18, loc.Col)
	assert.Equal(t, "45.6", nok.Value)

	assert.Equal(t, 1, aok.Items[2].Len())
	kok, ok = aok.Items[2].At(0)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 24, loc.Col)
	assert.Equal(t, "false", tok.Value)

	assert.Equal(t, 3, aok.Items[3].Len())
	kok, ok = aok.Items[3].At(0)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 31, loc.Col)
	assert.Equal(t, "true", tok.Value)

	kok, ok = aok.Items[3].At(1)
	assert.True(t, ok)
	pok, ok = kok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 36, loc.Col)
	assert.Equal(t, token.EQUALITY, pok.Value)

	kok, ok = aok.Items[3].At(2)
	assert.True(t, ok)
	tok, ok = kok.(token.Symbol)
	assert.True(t, ok)
	loc = tok.Loc
	assert.Equal(t, file, loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 39, loc.Col)
	assert.Equal(t, "true", tok.Value)
}

func TestSimpleArray(t *testing.T) {
	tokens, err := lex.File("file", "array = [1,2]")
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 3, tokens[0].Len())

	tok, ok := tokens[0].At(0)
	assert.True(t, ok)
	sok, ok := tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "file", sok.Loc.File)
	assert.Equal(t, 1, sok.Loc.Row)
	assert.Equal(t, 1, sok.Loc.Col)
	assert.Equal(t, "array", sok.Value)

	tok, ok = tokens[0].At(1)
	assert.True(t, ok)
	pok, ok := tok.(token.Special)
	assert.True(t, ok)
	assert.Equal(t, "file", pok.Loc.File)
	assert.Equal(t, 1, pok.Loc.Row)
	assert.Equal(t, 7, pok.Loc.Col)
	assert.Equal(t, token.ASSIGN, pok.Value)

	tok, ok = tokens[0].At(2)
	assert.True(t, ok)
	aok, ok := tok.(token.Array)
	assert.True(t, ok)
	assert.Equal(t, "file", aok.Loc.File)
	assert.Equal(t, 1, aok.Loc.Row)
	assert.Equal(t, 9, aok.Loc.Col)

	assert.Equal(t, 2, len(aok.Items))
	assert.Equal(t, 1, aok.Items[0].Len())
	tok, ok = aok.Items[0].At(0)
	assert.True(t, ok)
	nok, ok := tok.(token.Number)
	assert.True(t, ok)
	assert.Equal(t, "file", nok.Loc.File)
	assert.Equal(t, 1, nok.Loc.Row)
	assert.Equal(t, 10, nok.Loc.Col)
	assert.Equal(t, "1", nok.Value)

	assert.Equal(t, 1, aok.Items[1].Len())
	tok, ok = aok.Items[1].At(0)
	assert.True(t, ok)
	nok, ok = tok.(token.Number)
	assert.True(t, ok)
	assert.Equal(t, "file", nok.Loc.File)
	assert.Equal(t, 1, nok.Loc.Row)
	assert.Equal(t, 12, nok.Loc.Col)
	assert.Equal(t, "2", nok.Value)
}

func TestEnsureBlockInTest(t *testing.T) {
	tokens, err := lex.File("file", "test \"a\" {\nx=1\n}")
	assert.Nil(t, err)

	assert.Equal(t, 1, len(tokens))
	tok, ok := tokens[0].At(0)
	assert.True(t, ok)
	sok, ok := tok.(token.Symbol)
	assert.True(t, ok)
	loc := sok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "test", sok.Value)

	tok, ok = tokens[0].At(1)
	assert.True(t, ok)
	stok, ok := tok.(token.String)
	assert.True(t, ok)
	loc = stok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 6, loc.Col)
	assert.Equal(t, "a", stok.Value)

	tok, ok = tokens[0].At(2)
	assert.True(t, ok)
	bok, ok := tok.(token.Block)
	assert.True(t, ok)
	loc = bok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 1, loc.Row)
	assert.Equal(t, 10, loc.Col)

	assert.Equal(t, 1, len(bok.Tokens))
	tok, ok = bok.Tokens[0].At(0)
	assert.True(t, ok)
	sok, ok = tok.(token.Symbol)
	assert.True(t, ok)
	loc = sok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 1, loc.Col)
	assert.Equal(t, "x", sok.Value)

	tok, ok = bok.Tokens[0].At(1)
	assert.True(t, ok)
	pok, ok := tok.(token.Special)
	assert.True(t, ok)
	loc = pok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 2, loc.Col)
	assert.Equal(t, token.ASSIGN, pok.Value)

	tok, ok = bok.Tokens[0].At(2)
	assert.True(t, ok)
	nok, ok := tok.(token.Number)
	assert.True(t, ok)
	loc = nok.Loc
	assert.Equal(t, "file", loc.File)
	assert.Equal(t, 2, loc.Row)
	assert.Equal(t, 3, loc.Col)
	assert.Equal(t, "1", nok.Value)
}

func TestBlockLineParsingIssue(t *testing.T) {
	tokens, err := lex.File("file", "test \"simple test\" {\n    assert : true\n    bool = true\n}")
	assert.Nil(t, err)
	assert.Equal(t, 1, len(tokens))
	assert.Equal(t, 3, tokens[0].Len())

	tok, ok := tokens[0].At(0)
	assert.True(t, ok)
	sok, ok := tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "test", sok.Value)

	tok, ok = tokens[0].At(1)
	assert.True(t, ok)
	stok, ok := tok.(token.String)
	assert.True(t, ok)
	assert.Equal(t, "simple test", stok.Value)

	tok, ok = tokens[0].At(2)
	assert.True(t, ok)
	bok, ok := tok.(token.Block)
	assert.True(t, ok)
	assert.Equal(t, 2, len(bok.Tokens))

	assert.Equal(t, 2, bok.Tokens[0].Len())
	tok, ok = bok.Tokens[0].At(0)
	assert.True(t, ok)
	sok, ok = tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "assert", sok.Value)

	tok, ok = bok.Tokens[0].At(1)
	assert.True(t, ok)
	pok, ok := tok.(token.Pipe)
	assert.True(t, ok)
	assert.Equal(t, 1, pok.Tokens.Len())

	tok, ok = pok.Tokens.At(0)
	assert.True(t, ok)
	sok, ok = tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "true", sok.Value)

	assert.Equal(t, 3, bok.Tokens[1].Len())
	tok, ok = bok.Tokens[1].At(0)
	assert.True(t, ok)
	sok, ok = tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "bool", sok.Value)

	tok, ok = bok.Tokens[1].At(1)
	assert.True(t, ok)
	spok, ok := tok.(token.Special)
	assert.True(t, ok)
	assert.Equal(t, token.ASSIGN, spok.Value)

	tok, ok = bok.Tokens[1].At(2)
	assert.True(t, ok)
	sok, ok = tok.(token.Symbol)
	assert.True(t, ok)
	assert.Equal(t, "true", sok.Value)
}
