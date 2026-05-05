package common

import (
	"aiprobe/embed"
	"encoding/json"
	"fmt"
	"strings"
	"sync"
)

type MenuConf struct {
	Code     string     `json:"code"`
	Name     string     `json:"name"`
	Icon     string     `json:"icon"`
	Path     string     `json:"path"`
	Children []MenuConf `json:"children,omitempty"`
	SubApi   []MenuConf `json:"subapi,omitempty"`
}

var (
	menus    []MenuConf
	menuOnce sync.Once
)

func GetMenus() []MenuConf {
	menuOnce.Do(func() {
		content, err := embed.Conf.ReadFile("conf/menu.json")
		if err != nil {
			return
		}
		err = json.Unmarshal(content, &menus)
		if err != nil {
			fmt.Println("menu.json:", err)
		}

	})
	return menus
}

// ParseAuthCodes tries to parse admin.Auth into a set of codes.
// Supports JSON array: ["clusters","ssh"] or comma/semicolon separated: "clusters,ssh".
func ParseAuthCodes(auth string) map[string]bool {
	m := make(map[string]bool)
	if auth == "" {
		return m
	}
	var arr []string
	if json.Unmarshal([]byte(auth), &arr) == nil {
		for _, s := range arr {
			s = strings.TrimSpace(s)
			if s != "" {
				m[s] = true
			}
		}
		return m
	}
	// Fallback to separators
	for _, s := range strings.FieldsFunc(auth, func(r rune) bool {
		return r == ',' || r == ';' || r == '|' || r == ' '
	}) {
		s = strings.TrimSpace(s)
		if s != "" {
			m[s] = true
		}
	}
	return m
}

// FilterMenusByCodes returns a pruned menu tree containing only codes present in 'allowed'.
// A parent is included if its own code is allowed or any child/subapi is allowed.
func FilterMenusByCodes(all []MenuConf, allowed map[string]bool) []MenuConf {
	if len(allowed) == 0 {
		return []MenuConf{}
	}
	out := make([]MenuConf, 0, len(all))
	for _, m := range all {
		if allowed[m.Code] {
			item := m
			out = append(out, item)
			continue
		}
		var filteredChildren []MenuConf
		if len(m.Children) > 0 {
			filteredChildren = FilterMenusByCodes(m.Children, allowed)
		}
		var filteredSubApi []MenuConf
		if len(m.SubApi) > 0 {
			filteredSubApi = FilterMenusByCodes(m.SubApi, allowed)
		}
		if len(filteredChildren) > 0 || len(filteredSubApi) > 0 {
			item := m
			item.Children = filteredChildren
			item.SubApi = filteredSubApi
			out = append(out, item)
		}
	}
	return out
}

func FindMenuCodeByPath(requestPath string, adminPath string) string {
	ms := GetMenus()
	return findMenuCodeRecursive(ms, requestPath, adminPath)
}

func findMenuCodeRecursive(menus []MenuConf, requestPath string, adminPath string) string {
	for _, m := range menus {
		// Construct full path for comparison
		// If adminPath is not empty, prepend it.
		// requestPath usually starts with /
		// m.Path usually starts with /
		// e.g. adminPath="admin", m.Path="/index" -> "/admin/index"
		// e.g. adminPath="", m.Path="/index" -> "/index"

		fullPath := ""
		if adminPath != "" {
			fullPath = "/" + adminPath + m.Path
		} else {
			fullPath = m.Path
		}

		// fmt.Println(m.Path, fullPath, requestPath)
		// fmt.Println(m.Children)
		// Check exact match
		if m.Path != "" && fullPath == requestPath {
			return m.Code
		}

		// Recursive check subapi
		if len(m.SubApi) > 0 {
			code := findMenuCodeRecursive(m.SubApi, requestPath, adminPath)
			if code != "" {
				return code
			}
		}

		// Recursive check children
		if len(m.Children) > 0 {
			code := findMenuCodeRecursive(m.Children, requestPath, adminPath)
			if code != "" {
				return code
			}
		}
	}
	return ""
}
