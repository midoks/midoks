package embed

import (
	"embed"
	"fmt"
)

//go:embed static/*
var Static embed.FS

//go:embed templates
var Templates embed.FS

//go:embed conf/*
var Conf embed.FS

func TemplatesAllNames(dirName string) []string {
	if dirName == "" {
		dirName = "templates"
	}

	names := []string{}
	dir, _ := Templates.ReadDir(dirName)

	for _, d := range dir {
		if d.IsDir() {
			subName := TemplatesAllNames(fmt.Sprintf("%s/%s", dirName, d.Name()))
			names = append(names, subName...)
		} else {
			names = append(names, fmt.Sprintf("%s/%s", dirName, d.Name()))
		}
	}
	return names
}
