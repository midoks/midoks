package tools

import (
	"testing"
)

func TestStrTo_Exist(t *testing.T) {
	tests := []struct {
		name string
		str  StrTo
		want bool
	}{
		{
			name: "empty string",
			str:  StrTo(""),
			want: true,
		},
		{
			name: "normal string",
			str:  StrTo("hello"),
			want: true,
		},
		{
			name: "special character",
			str:  StrTo(string(rune(0x1E))),
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.str.Exist(); got != tt.want {
				t.Errorf("StrTo.Exist() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestStrTo_Uint8(t *testing.T) {
	tests := []struct {
		name    string
		str     StrTo
		want    uint8
		wantErr bool
	}{
		{
			name:    "valid uint8",
			str:     StrTo("123"),
			want:    123,
			wantErr: false,
		},
		{
			name:    "zero",
			str:     StrTo("0"),
			want:    0,
			wantErr: false,
		},
		{
			name:    "max uint8",
			str:     StrTo("255"),
			want:    255,
			wantErr: false,
		},
		{
			name:    "invalid string",
			str:     StrTo("abc"),
			want:    0,
			wantErr: true,
		},
		{
			name:    "overflow",
			str:     StrTo("256"),
			want:    255,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := tt.str.Uint8()
			if (err != nil) != tt.wantErr {
				t.Errorf("StrTo.Uint8() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("StrTo.Uint8() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestStrTo_Int(t *testing.T) {
	tests := []struct {
		name    string
		str     StrTo
		want    int
		wantErr bool
	}{
		{
			name:    "positive int",
			str:     StrTo("123"),
			want:    123,
			wantErr: false,
		},
		{
			name:    "negative int",
			str:     StrTo("-123"),
			want:    -123,
			wantErr: false,
		},
		{
			name:    "zero",
			str:     StrTo("0"),
			want:    0,
			wantErr: false,
		},
		{
			name:    "invalid string",
			str:     StrTo("abc"),
			want:    0,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := tt.str.Int()
			if (err != nil) != tt.wantErr {
				t.Errorf("StrTo.Int() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("StrTo.Int() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestStrTo_Int64(t *testing.T) {
	tests := []struct {
		name    string
		str     StrTo
		want    int64
		wantErr bool
	}{
		{
			name:    "positive int64",
			str:     StrTo("9223372036854775807"),
			want:    9223372036854775807,
			wantErr: false,
		},
		{
			name:    "negative int64",
			str:     StrTo("-9223372036854775808"),
			want:    -9223372036854775808,
			wantErr: false,
		},
		{
			name:    "zero",
			str:     StrTo("0"),
			want:    0,
			wantErr: false,
		},
		{
			name:    "invalid string",
			str:     StrTo("abc"),
			want:    0,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := tt.str.Int64()
			if (err != nil) != tt.wantErr {
				t.Errorf("StrTo.Int64() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("StrTo.Int64() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestStrTo_Float64(t *testing.T) {
	tests := []struct {
		name    string
		str     StrTo
		want    float64
		wantErr bool
	}{
		{
			name:    "positive float",
			str:     StrTo("123.456"),
			want:    123.456,
			wantErr: false,
		},
		{
			name:    "negative float",
			str:     StrTo("-123.456"),
			want:    -123.456,
			wantErr: false,
		},
		{
			name:    "zero",
			str:     StrTo("0.0"),
			want:    0.0,
			wantErr: false,
		},
		{
			name:    "integer as float",
			str:     StrTo("123"),
			want:    123.0,
			wantErr: false,
		},
		{
			name:    "invalid string",
			str:     StrTo("abc"),
			want:    0,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := tt.str.Float64()
			if (err != nil) != tt.wantErr {
				t.Errorf("StrTo.Float64() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("StrTo.Float64() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestStrTo_MustMethods(t *testing.T) {
	// Test Must methods that ignore errors
	t.Run("MustUint8", func(t *testing.T) {
		if got := StrTo("123").MustUint8(); got != 123 {
			t.Errorf("StrTo.MustUint8() = %v, want %v", got, 123)
		}
		// Test with invalid input (should return zero value)
		if got := StrTo("abc").MustUint8(); got != 0 {
			t.Errorf("StrTo.MustUint8() with invalid input = %v, want %v", got, 0)
		}
	})

	t.Run("MustInt", func(t *testing.T) {
		if got := StrTo("123").MustInt(); got != 123 {
			t.Errorf("StrTo.MustInt() = %v, want %v", got, 123)
		}
		if got := StrTo("abc").MustInt(); got != 0 {
			t.Errorf("StrTo.MustInt() with invalid input = %v, want %v", got, 0)
		}
	})

	t.Run("MustInt64", func(t *testing.T) {
		if got := StrTo("123").MustInt64(); got != 123 {
			t.Errorf("StrTo.MustInt64() = %v, want %v", got, 123)
		}
		if got := StrTo("abc").MustInt64(); got != 0 {
			t.Errorf("StrTo.MustInt64() with invalid input = %v, want %v", got, 0)
		}
	})

	t.Run("MustFloat64", func(t *testing.T) {
		if got := StrTo("123.456").MustFloat64(); got != 123.456 {
			t.Errorf("StrTo.MustFloat64() = %v, want %v", got, 123.456)
		}
		if got := StrTo("abc").MustFloat64(); got != 0 {
			t.Errorf("StrTo.MustFloat64() with invalid input = %v, want %v", got, 0)
		}
	})
}

func TestStrTo_String(t *testing.T) {
	tests := []struct {
		name string
		str  StrTo
		want string
	}{
		{
			name: "normal string",
			str:  StrTo("hello"),
			want: "hello",
		},
		{
			name: "empty string",
			str:  StrTo(""),
			want: "",
		},
		{
			name: "numeric string",
			str:  StrTo("123"),
			want: "123",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.str.String(); got != tt.want {
				t.Errorf("StrTo.String() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestPowInt(t *testing.T) {
	tests := []struct {
		name string
		x    int
		y    int
		want int
	}{
		{
			name: "2^3",
			x:    2,
			y:    3,
			want: 8,
		},
		{
			name: "5^0",
			x:    5,
			y:    0,
			want: 1,
		},
		{
			name: "3^1",
			x:    3,
			y:    1,
			want: 3,
		},
		{
			name: "10^2",
			x:    10,
			y:    2,
			want: 100,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := PowInt(tt.x, tt.y); got != tt.want {
				t.Errorf("PowInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestToStr(t *testing.T) {
	tests := []struct {
		name  string
		value interface{}
		args  []int
		want  string
	}{
		{
			name:  "string",
			value: "hello",
			want:  "hello",
		},
		{
			name:  "int",
			value: 123,
			want:  "123",
		},
		{
			name:  "float64",
			value: 123.456,
			want:  "123.456",
		},
		{
			name:  "float64 with precision",
			value: 123.456789,
			args:  []int{2},
			want:  "123.46",
		},
		{
			name:  "bool true",
			value: true,
			want:  "true",
		},
		{
			name:  "bool false",
			value: false,
			want:  "false",
		},
		{
			name:  "nil",
			value: nil,
			want:  "<nil>",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ToStr(tt.value, tt.args...); got != tt.want {
				t.Errorf("ToStr() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestArgInt_Get(t *testing.T) {
	tests := []struct {
		name string
		args argInt
		i    int
		def  []int
		want int
	}{
		{
			name: "get existing index",
			args: argInt{1, 2, 3},
			i:    1,
			want: 2,
		},
		{
			name: "get non-existing index with default",
			args: argInt{1, 2, 3},
			i:    5,
			def:  []int{99},
			want: 99,
		},
		{
			name: "get non-existing index without default",
			args: argInt{1, 2, 3},
			i:    5,
			want: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.args.Get(tt.i, tt.def...); got != tt.want {
				t.Errorf("argInt.Get() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestHexStr2int(t *testing.T) {
	tests := []struct {
		name    string
		hexStr  string
		want    int
		wantErr bool
	}{
		{
			name:    "valid hex lowercase",
			hexStr:  "ff",
			want:    255,
			wantErr: false,
		},
		{
			name:    "valid hex ab",
			hexStr:  "ab",
			want:    171,
			wantErr: false,
		},
		{
			name:    "zero",
			hexStr:  "0",
			want:    0,
			wantErr: false,
		},
		{
			name:    "invalid hex",
			hexStr:  "GG",
			want:    -1,
			wantErr: true,
		},
		{
			name:    "empty string",
			hexStr:  "",
			want:    0,
			wantErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := HexStr2int(tt.hexStr)
			if (err != nil) != tt.wantErr {
				t.Errorf("HexStr2int() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("HexStr2int() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestInt2HexStr(t *testing.T) {
	tests := []struct {
		name string
		num  int
		want string
	}{
		{
			name: "255 to ff",
			num:  255,
			want: "ff",
		},
		{
			name: "0 to 0",
			num:  0,
			want: "0",
		},
		{
			name: "16 to 10",
			num:  16,
			want: "10",
		},
		{
			name: "10 to a",
			num:  10,
			want: "a",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Int2HexStr(tt.num); got != tt.want {
				t.Errorf("Int2HexStr() = %v, want %v", got, tt.want)
			}
		})
	}
}

// Benchmark tests
func BenchmarkStrTo_Int(b *testing.B) {
	str := StrTo("12345")
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		str.Int()
	}
}

func BenchmarkToStr(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		ToStr(12345)
	}
}

func BenchmarkPowInt(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		PowInt(2, 10)
	}
}
