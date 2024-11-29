package lexer

import (
	"fmt"
	"regexp"
	"sort"
	"strings"
)

type TokenType int

const (
	// TEXTUAL TOKENS
	EOF TokenType = iota
	NEWLINE
	OPEN_PARENTHESIS
	CLOSE_PARENTHESIS
	OPEN_BRACE
	CLOSE_BRACE
	OPEN_BRACKET
	CLOSE_BRACKET
	COMMA
	COLON
	PERIOD
	SEMICOLON
	PLUS
	MINUS
	MULTIPLY
	DIVIDE
	MODULO
	INT_DIVIDE

	// IDENTIFIERS
	IDENTIFIER

	// LITERALS
	INTEGER
	FLOAT
	STRING

	// VALUES
	TRUE
	FALSE
	NIL

	// COMMENTS
	LINE_COMMENT
	BLOCK_COMMENT

	// STRUCTURES
	ENUM
	STRUCT
	REQUIRED

	// OOP
	CLASS
	ABSTRACT
	IMPLEMENTS
	GET
	SET
	GETTER
	SETTER
	OF
	AS
	SUPER
	IS
	DECORATOR
	THIS

	// FUNCTIONS
	FUNCTION
	ARROW
	RETURN
	ASYNC

	// MODULES
	MODULE
	IMPORT
	FROM

	// FOR LOOP
	FOR
	BREAK
	CONTINUE
	IN
	RANGE

	// LOOPS
	WHILE
	LOOP

	// CONDITIONALS
	IF
	ELIF
	ELSE

	// MATCH
	MATCH
	MATCH_ARROW

	// VARIABLE DECLARATIONS
	MUT
	LET

	// TYPES
	TYPE
	U8_TYPE
	U16_TYPE
	U32_TYPE
	U64_TYPE
	I8_TYPE
	I16_TYPE
	I32_TYPE
	I64_TYPE
	F32_TYPE
	F64_TYPE
	BOOL_TYPE
	CHAR_TYPE
	STR_TYPE
	INT_TYPE
	UINT_TYPE
	FLOAT_TYPE

	// ASSIGNMENTS
	ASSIGN
	PLUS_ASSIGN
	MINUS_ASSIGN
	MULTIPLY_ASSIGN
	DIVIDE_ASSIGN
	MODULO_ASSIGN

	// COMPARISONS
	EQUAL
	NOT_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL
	AND
	OR
	NOT

	// OPTIONALS
	OPTIONAL
	ERROR
	EXCEPT
)

type TokenPhrase = []Token

var keywords = map[string]TokenType{
	"async":    ASYNC,
	"module":   MODULE,
	"import":   IMPORT,
	"from":     FROM,
	"required": REQUIRED,
	"optional": OPTIONAL,
	"struct":   STRUCT,
	"class":    CLASS,
	"abs":      ABSTRACT,
	"impl":     IMPLEMENTS,
	"enum":     ENUM,
	"fn":       FUNCTION,
	"let":      LET,
	"mut":      MUT,
	"getter":   GETTER,
	"setter":   SETTER,
	"get":      GET,
	"set":      SET,
	"if":       IF,
	"else":     ELSE,
	"match":    MATCH,
	"for":      FOR,
	"while":    WHILE,
	"loop":     LOOP,
	"ret":      RETURN,
	"break":    BREAK,
	"continue": CONTINUE,
	"true":     TRUE,
	"false":    FALSE,
	"nil":      NIL,
	"as":       AS,
	"is":       IS,
	"in":       IN,
	"of":       OF,
	"except":   EXCEPT,
	"super":    SUPER,
	"this":     THIS,
}

var symbols = map[string]TokenType{
	"+":   PLUS,
	"-":   MINUS,
	"*":   MULTIPLY,
	"/":   DIVIDE,
	"%":   MODULO,
	"//":  INT_DIVIDE,
	":=":  ASSIGN,
	"+=":  PLUS_ASSIGN,
	"-=":  MINUS_ASSIGN,
	"*=":  MULTIPLY_ASSIGN,
	"/=":  DIVIDE_ASSIGN,
	"%=":  MODULO_ASSIGN,
	"==":  EQUAL,
	"!=":  NOT_EQUAL,
	">":   GREATER,
	">=":  GREATER_EQUAL,
	"<":   LESS,
	"<=":  LESS_EQUAL,
	"and": AND,
	"or":  OR,
	"not": NOT,
	"=>":  MATCH_ARROW,
	"->":  ARROW,
	"..":  RANGE,
	":":   COLON,
	",":   COMMA,
	".":   PERIOD,
	";":   SEMICOLON,
	"(":   OPEN_PARENTHESIS,
	")":   CLOSE_PARENTHESIS,
	"{":   OPEN_BRACE,
	"}":   CLOSE_BRACE,
	"[":   OPEN_BRACKET,
	"]":   CLOSE_BRACKET,
}

var types = map[string]TokenType{
	"u8":    U8_TYPE,
	"u16":   U16_TYPE,
	"u32":   U32_TYPE,
	"u64":   U64_TYPE,
	"i8":    I8_TYPE,
	"i16":   I16_TYPE,
	"i32":   I32_TYPE,
	"i64":   I64_TYPE,
	"f32":   F32_TYPE,
	"f64":   F64_TYPE,
	"bool":  BOOL_TYPE,
	"char":  CHAR_TYPE,
	"str":   STR_TYPE,
	"int":   INT_TYPE,
	"uint":  UINT_TYPE,
	"float": FLOAT_TYPE,
}

func (t TokenType) String() string {
	switch t {
	case EOF:
		return "EOF"
	case NEWLINE:
		return "NEWLINE"
	case OPEN_PARENTHESIS:
		return "OPEN_PARENTHESIS"
	case CLOSE_PARENTHESIS:
		return "CLOSE_PARENTHESIS"
	case OPEN_BRACE:
		return "OPEN_BRACE"
	case CLOSE_BRACE:
		return "CLOSE_BRACE"
	case OPEN_BRACKET:
		return "OPEN_BRACKET"
	case CLOSE_BRACKET:
		return "CLOSE_BRACKET"
	case COMMA:
		return "COMMA"
	case COLON:
		return "COLON"
	case PERIOD:
		return "PERIOD"
	case SEMICOLON:
		return "SEMICOLON"
	case PLUS:
		return "PLUS"
	case MINUS:
		return "MINUS"
	case MULTIPLY:
		return "MULTIPLY"
	case DIVIDE:
		return "DIVIDE"
	case MODULO:
		return "MODULO"
	case INT_DIVIDE:
		return "INT_DIVIDE"
	case IDENTIFIER:
		return "IDENTIFIER"
	case INTEGER:
		return "INTEGER"
	case FLOAT:
		return "FLOAT"
	case STRING:
		return "STRING"
	case TRUE:
		return "TRUE"
	case FALSE:
		return "FALSE"
	case NIL:
		return "NIL"
	case LINE_COMMENT:
		return "LINE_COMMENT"
	case BLOCK_COMMENT:
		return "BLOCK_COMMENT"
	case ENUM:
		return "ENUM"
	case STRUCT:
		return "STRUCT"
	case REQUIRED:
		return "REQUIRED"
	case CLASS:
		return "CLASS"
	case ABSTRACT:
		return "ABSTRACT"
	case IMPLEMENTS:
		return "IMPLEMENTS"
	case GET:
		return "GET"
	case SET:
		return "SET"
	case GETTER:
		return "GETTER"
	case SETTER:
		return "SETTER"
	case OF:
		return "OF"
	case AS:
		return "AS"
	case SUPER:
		return "SUPER"
	case THIS:
		return "THIS"
	case FUNCTION:
		return "FUNCTION"
	case ARROW:
		return "ARROW"
	case RETURN:
		return "RETURN"
	case ASYNC:
		return "ASYNC"
	case MODULE:
		return "MODULE"
	case IMPORT:
		return "IMPORT"
	case FROM:
		return "FROM"
	case FOR:
		return "FOR"
	case BREAK:
		return "BREAK"
	case CONTINUE:
		return "CONTINUE"
	case IN:
		return "IN"
	case RANGE:
		return "RANGE"
	case WHILE:
		return "WHILE"
	case LOOP:
		return "LOOP"
	case IF:
		return "IF"
	case ELIF:
		return "ELIF"
	case ELSE:
		return "ELSE"
	case MATCH:
		return "MATCH"
	case MATCH_ARROW:
		return "MATCH_ARROW"
	case MUT:
		return "MUT"
	case LET:
		return "LET"
	case TYPE:
		return "TYPE"
	case U8_TYPE:
		return "U8_TYPE"
	case U16_TYPE:
		return "U16_TYPE"
	case U32_TYPE:
		return "U32_TYPE"
	case U64_TYPE:
		return "U64_TYPE"
	case I8_TYPE:
		return "I8_TYPE"
	case I16_TYPE:
		return "I16_TYPE"
	case I32_TYPE:
		return "I32_TYPE"
	case I64_TYPE:
		return "I64_TYPE"
	case F32_TYPE:
		return "F32_TYPE"
	case F64_TYPE:
		return "F64_TYPE"
	case BOOL_TYPE:
		return "BOOL_TYPE"
	case CHAR_TYPE:
		return "CHAR_TYPE"
	case STR_TYPE:
		return "STR_TYPE"
	case INT_TYPE:
		return "INT_TYPE"
	case UINT_TYPE:
		return "UINT_TYPE"
	case FLOAT_TYPE:
		return "FLOAT_TYPE"
	case EXCEPT:
		return "EXCEPT"
	case ERROR:
		return "ERROR"
	case DECORATOR:
		return "DECORATOR"
	case ASSIGN:
		return "ASSIGN"
	case PLUS_ASSIGN:
		return "PLUS_ASSIGN"
	case MINUS_ASSIGN:
		return "MINUS_ASSIGN"
	case MULTIPLY_ASSIGN:
		return "MULTIPLY_ASSIGN"
	case DIVIDE_ASSIGN:
		return "DIVIDE_ASSIGN"
	case MODULO_ASSIGN:
		return "MODULO_ASSIGN"
	case EQUAL:
		return "EQUAL"
	case NOT_EQUAL:
		return "NOT_EQUAL"
	case GREATER:
		return "GREATER"
	case GREATER_EQUAL:
		return "GREATER_EQUAL"
	case LESS:
		return "LESS"
	case LESS_EQUAL:
		return "LESS_EQUAL"
	case AND:
		return "AND"
	case OR:
		return "OR"
	case NOT:
		return "NOT"
	case OPTIONAL:
		return "OPTIONAL"
	default:
		return fmt.Sprintf("UNKNOWN %d", int(t))
	}
}

type Code string

type Token struct {
	Type  TokenType
	Value string
}

func (t Token) String() string {
	switch t.Type {
	case LINE_COMMENT, BLOCK_COMMENT, STRING, INTEGER, FLOAT, IDENTIFIER, TYPE:
		return t.Value
	default:
		return t.Type.String()
	}
}

func (s Code) Split(splitter ...string) []string {
	if len(splitter) == 0 {
		return strings.Split(string(s), "\n")
	}
	return strings.Split(string(s), splitter[0])
}

func (s Code) Tokenize() (TokenPhrase, error) {
	tokens := TokenPhrase{}
	for _, line := range s.Split() {
		if len(line) == 0 {
			continue
		} else {
			newTokens, err := tokenizeLine(line)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, newTokens...)
		}
	}

	return tokens, nil
}

func tokenizeLine(line string) (tokens TokenPhrase, err error) {
	tokens = TokenPhrase{}
	line = strings.TrimSpace(line)
	idx := 0
	newIdx := 0
	for idx < len(line) {
		tokens, newIdx, err = tokenizeNext(line, tokens, idx)
		if err != nil {
			return nil, err
		}
		if newIdx == idx {
			idx++
		} else {
			idx = newIdx
		}
	}

	return tokens, nil
}

func tokenizeNext(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	newTokens = tokens
	newIdx = idx
	if idx >= len(line) {
		return tokens, idx, nil
	}
	trimmed := strings.TrimSpace(line[idx:])
	delta := len(line[idx:]) - len(trimmed)

	attemptTokenize := func(tokenizer func(string, []Token, int) ([]Token, int, error)) ([]Token, int, error) {
		return tokenizer(line, tokens, newIdx)
	}

	tokenizers := []func(string, []Token, int) ([]Token, int, error){
		tokenizeComment,
		tokenizeString,
		tokenizeKeyword,
		tokenizeFloat,
		tokenizeInteger,
		tokenizeSymbol,
		tokenizeType,
		tokenizeDecorator,
		tokenizeIdentifier,
	}

	for _, tokenizer := range tokenizers {
		newTokens, newIdx, err = attemptTokenize(tokenizer)
		if err != nil {
			return nil, 0, err
		} else if newIdx != idx {
			return newTokens, newIdx + delta, nil
		}
	}
	return newTokens, newIdx, nil
}

func sortKeys(keys []string) {
	sort.Slice(keys, func(i, j int) bool {
		return len(keys[i]) > len(keys[j])
	})
}

func getKeys(m map[string]TokenType) []string {
	keys := []string{}
	for k := range m {
		keys = append(keys, k)
	}
	return keys
}

func tokenizeFromMap(line string, tokens TokenPhrase, idx int, m map[string]TokenType) (newTokens TokenPhrase, newIdx int, err error) {
	keys := getKeys(m)
	sortKeys(keys)
	for _, k := range keys {
		if strings.HasPrefix(line[idx:], k) {
			tokens = append(tokens, Token{Type: m[k], Value: k})
			return tokens, idx + len(k), nil
		}
	}
	return tokens, idx, nil
}

func tokenizeKeyword(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	// we use greedy matching here so we can match the longest possible keyword
	return tokenizeFromMap(line, tokens, idx, keywords)
}

func tokenizeSymbol(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	return tokenizeFromMap(line, tokens, idx, symbols)
}

func tokenizeType(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	newTokens, newIdx, err = tokenizeFromMap(line, tokens, idx, types)
	if err != nil {
		return nil, 0, err
	}
	if newIdx == idx {
		// use regex for custom types
		regex := regexp.MustCompile(`^[A-Z][a-zA-Z0-9_]?`)
		if regex.MatchString(line[newIdx:]) {
			tokens = append(tokens, Token{Type: TYPE, Value: line[newIdx:]})
			return tokens, len(line), nil
		} else {
			return tokens, newIdx, nil
		}
	} else {
		return tokens, newIdx, nil
	}
}

func tokenizeIdentifier(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	// identifiers are either of the form hello_world or HELLO_WORLD
	regex := regexp.MustCompile(`^[a-z_][a-z0-9_]*|^[A-Z_][a-zA-Z0-9_]*`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: IDENTIFIER, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	return tokens, idx, nil
}

func tokenizeInteger(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	regex := regexp.MustCompile(`^[0-9]+`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: INTEGER, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	return tokens, idx, nil
}

func tokenizeFloat(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	// floats are either of the form 1.23 or 0.32e12 or 4.8e-10
	regex := regexp.MustCompile(`^[0-9]*(\.[0-9]+)?e-?[0-9]+|^[0-9]+\.[0-9]+`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: FLOAT, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	return tokens, idx, nil
}

func tokenizeComment(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	// single line comments start with '--'
	// multi line comments start with '---' and end with '---'
	// for now we won't handle block comments
	if strings.HasPrefix(line[idx:], "--") {
		tokens = append(tokens, Token{Type: LINE_COMMENT, Value: line[idx:]})
		return tokens, len(line), nil
	}
	return tokens, idx, nil
}

func tokenizeString(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	// strings are enclosed in double quotes or single quotes
	// for now we won't handle escape sequences
	// double quote regex
	regex := regexp.MustCompile(`^"[^"]*"`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: STRING, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	// single quote regex
	regex = regexp.MustCompile(`^'[^']*'`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: STRING, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	return tokens, idx, nil
}

func tokenizeDecorator(line string, tokens TokenPhrase, idx int) (newTokens TokenPhrase, newIdx int, err error) {
	regex := regexp.MustCompile(`^@[a-zA-Z_][a-zA-Z0-9_]*`)
	if regex.MatchString(line[idx:]) {
		tokens = append(tokens, Token{Type: DECORATOR, Value: regex.FindString(line[idx:])})
		return tokens, idx + len(regex.FindString(line[idx:])), nil
	}
	return tokens, idx, nil
}
