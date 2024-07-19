package quickshotmessage

import (
	"fmt"
	"reflect"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

// QType Enum
type EType int

const (
	DEFAULT EType = iota
	EInt
	EFloat
	EString
	EArray
	EJson
)

func QTypeToValue(qType EType) int {
	switch qType {
	case DEFAULT:
		return 0
	case EInt:
		return 1
	case EFloat:
		return 2
	case EString:
		return 3
	case EArray:
		return 4
	default:
		return -1
	}
}

// QTuple Struct
type QTuple struct {
	Field struct {
		EType
		QInteger int64
		QFloat   float64
		QString  string
		QArray   []interface{}
	}
}

// QValue Struct
type QValue struct {
	EType
	MetaData int
	Buffer   string
}

func (qv *QValue) convert() string {
	return ""
}

// QAction Interface
type QAction interface {
	Initialize()
	getValue() QValue
	getBuffer() string
}

// QInteger Struct
type QInteger struct {
	Val  QValue
	Data int64
}

func NewQInteger(data int64) *QInteger {
	qInt := &QInteger{}
	qInt.Val.EType = EInt
	qInt.Val.MetaData = 0
	qInt.Val.Buffer = fmt.Sprintf("[%d:%d:%d]", QTypeToValue(qInt.Val.EType), qInt.Val.MetaData, data)
	qInt.Data = data
	return qInt
}

func (qi *QInteger) Initialize() {
	qi.Val.MetaData = 0
	qi.Val.Buffer = fmt.Sprintf("[%d:%d:%d]", QTypeToValue(qi.Val.EType), qi.Val.MetaData, qi.Data)
}

func (qi *QInteger) getValue() QValue {
	return qi.Val
}

func (qi *QInteger) getBuffer() string {
	return qi.Val.Buffer
}

// QFloat Struct
type QFloat struct {
	Val  QValue
	Data float64
}

func NewQFloat(data float64) *QFloat {
	qFloat := &QFloat{}
	qFloat.Val.EType = EFloat
	qFloat.Val.MetaData = 0
	qFloat.Val.Buffer = fmt.Sprintf("[%d:%d:%f]", QTypeToValue(qFloat.Val.EType), qFloat.Val.MetaData, data)
	qFloat.Data = data
	return qFloat
}

func (qf *QFloat) Initialize() {
	qf.Val.MetaData = 0
	qf.Val.Buffer = fmt.Sprintf("[%d:%d:%f]", QTypeToValue(qf.Val.EType), qf.Val.MetaData, qf.Data)
}

func (qf *QFloat) getValue() QValue {
	return qf.Val
}

func (qf *QFloat) getBuffer() string {
	return qf.Val.Buffer
}

// QString Struct
type QString struct {
	Val  QValue
	Data string
}

func NewQString(data string) *QString {
	qString := &QString{}
	qString.Val.EType = EString
	qString.Val.MetaData = len(data)
	qString.Val.Buffer = fmt.Sprintf("[%d:%d:%s]", QTypeToValue(qString.Val.EType), qString.Val.MetaData, data)
	qString.Data = data
	return qString
}

func (qs *QString) Initialize() {
	qs.Val.MetaData = len(qs.Data)
	qs.Val.Buffer = fmt.Sprintf("[%d:%d:%s]", QTypeToValue(qs.Val.EType), qs.Val.MetaData, qs.Data)
}

func (qs *QString) GetValue() QValue {
	return qs.Val
}

func (qs *QString) GetBuffer() string {
	return qs.Val.Buffer
}

// QArray Struct
type QArray struct {
	Val      QValue
	ElemType EType
	Data     []interface{}
}

func NewEArray(data []interface{}, elemType EType) *QValue {
	var buffer strings.Builder
	buffer.WriteString(fmt.Sprintf("[%d:", EArray))
	buffer.WriteString(fmt.Sprintf("%d:", len(data)))
	buffer.WriteString(fmt.Sprintf("=%d=", elemType))
	for _, elem := range data {
		buffer.WriteString(fmt.Sprintf("%v,", elem))
	}
	result := buffer.String()
	if len(result) > 0 {
		result = result[:len(result)-1]
	}
	result += "]"
	return &QValue{EType: EArray, MetaData: len(data), Buffer: result}
}

func (qa *QArray) Initialize() {
	qa.Val.MetaData = len(qa.Data)
	var buffer strings.Builder
	buffer.WriteString(fmt.Sprintf("[%d:%d:=%d=", QTypeToValue(qa.Val.EType), qa.Val.MetaData, QTypeToValue(qa.ElemType)))
	for _, elem := range qa.Data {
		buffer.WriteString(fmt.Sprintf("%v,", elem))
	}
	result := buffer.String()
	if len(result) > 0 {
		result = result[:len(result)-1]
	}
	result += "]"
	qa.Val.Buffer = result
}

func (qa *QArray) GetValue() QValue {
	return qa.Val
}

func (qa *QArray) GetBuffer() string {
	return qa.Val.Buffer
}

// QMessage Struct
type QMessage struct {
	ID   int64
	Size int
	Data []string
}

func NewQMessage(id int64, size int, data []string) *QMessage {
	return &QMessage{
		ID:   id,
		Size: size,
		Data: data,
	}
}

func (qm *QMessage) GetID() int64 {
	return qm.ID
}

func (qm *QMessage) GetSize() int {
	return qm.Size
}

func (qm *QMessage) GetData() []string {
	return qm.Data
}

func Deserialize(input string) (uint32, uint32, string) {
	re := regexp.MustCompile(`(\d+):(\d+):(.*)`)
	matches := re.FindStringSubmatch(input)
	if len(matches) > 3 {
		id, _ := strconv.Atoi(matches[1])
		size, _ := strconv.Atoi(matches[2])
		data := matches[3]
		return uint32(id), uint32(size), data
	}
	return 0, 0, ""
}

func ExtractData(input string) []string {
	re := regexp.MustCompile(`\[[^\[\]]*\]`)
	matches := re.FindAllString(input, -1)
	var result []string
	for _, match := range matches {
		result = append(result, match)
	}
	return result
}

func Serialize(msg *QMessage) string {
	var serialized strings.Builder
	serialized.WriteString(fmt.Sprintf("%d:%d:{", msg.ID, msg.Size))
	for _, elem := range msg.Data {
		serialized.WriteString(elem)
	}
	serialized.WriteString("}")
	return serialized.String()
}

// TEST

func TEST() {
	// deserialize 함수 테스트
	input1 := "123:456:Hello"
	id, size, data := Deserialize(input1)
	fmt.Printf("deserialize(%q) = (%d, %d, %q)\n", input1, id, size, data)

	// extractData 함수 테스트
	input2 := "[123:456:Hello][789:101112:World]"
	extractedData := ExtractData(input2)
	fmt.Printf("extractData(%q) = %v\n", input2, extractedData)

	// serialize 함수 테스트
	msg := NewQMessage(123, 2, []string{"[1:2:Data1]", "[3:4:Data2]"})
	serializedData := Serialize(msg)
	fmt.Printf("serialize(%v) = %q\n", msg, serializedData)

	// Additional checks for correctness
	expectedExtracted := []string{"[123:456:Hello]", "[789:101112:World]"}
	expectedSerialized := "123:2:{[1:2:Data1][3:4:Data2]}"

	if !reflect.DeepEqual(extractedData, expectedExtracted) {
		fmt.Println("extractData test failed")
	} else {
		fmt.Println("extractData test passed")
	}

	if serializedData != expectedSerialized {
		fmt.Println("serialize test failed")
	} else {
		fmt.Println("serialize test passed")
	}
}

func TestDeserialize(t *testing.T) {
	tests := []struct {
		input    string
		expected struct {
			id   uint32
			size uint32
			data string
		}
	}{
		{"123:456:Hello", struct {
			id   uint32
			size uint32
			data string
		}{123, 456, "Hello"}},
		{"789:101112:World", struct {
			id   uint32
			size uint32
			data string
		}{789, 101112, "World"}},
		{"", struct {
			id   uint32
			size uint32
			data string
		}{0, 0, ""}},
	}

	for _, tt := range tests {
		id, size, data := Deserialize(tt.input)
		if id != tt.expected.id || size != tt.expected.size || data != tt.expected.data {
			t.Errorf("deserialize(%q) = (%d, %d, %q); expected (%d, %d, %q)",
				tt.input, id, size, data, tt.expected.id, tt.expected.size, tt.expected.data)
		}
	}
}

func TestExtractData(t *testing.T) {
	tests := []struct {
		input    string
		expected []string
	}{
		{"[123:456:Hello][789:101112:World]", []string{"[123:456:Hello]", "[789:101112:World]"}},
		{"[1:2:Data][3:4:MoreData]", []string{"[1:2:Data]", "[3:4:MoreData]"}},
		{"[5:6:Single]", []string{"[5:6:Single]"}},
		{"", []string{}},
	}

	for _, tt := range tests {
		result := ExtractData(tt.input)
		if !reflect.DeepEqual(result, tt.expected) {
			t.Errorf("extractData(%q) = %v; expected %v",
				tt.input, result, tt.expected)
		}
	}
}

func TestSerialize(t *testing.T) {
	tests := []struct {
		msg      *QMessage
		expected string
	}{
		{NewQMessage(123, 2, []string{"[1:2:Data1]", "[3:4:Data2]"}), "123:2:{[1:2:Data1][3:4:Data2]}"},
		{NewQMessage(456, 1, []string{"[5:6:Data3]"}), "456:1:{[5:6:Data3]}"},
		{NewQMessage(789, 0, []string{}), "789:0:{}"},
	}

	for _, tt := range tests {
		result := Serialize(tt.msg)
		if result != tt.expected {
			t.Errorf("serialize(%v) = %q; expected %q",
				tt.msg, result, tt.expected)
		}
	}
}
