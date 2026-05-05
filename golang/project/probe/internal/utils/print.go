package tools

import (
	"encoding/json"
	"fmt"
)

func PrettyJSON(v interface{}) string {
	b, _ := json.MarshalIndent(v, "", "  ")
	return string(b)
}

func PrintJSON(v interface{}) {
	fmt.Println(PrettyJSON(v))
}
