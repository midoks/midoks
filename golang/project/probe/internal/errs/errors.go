package errs

import (
	"errors"
	"fmt"

	pkgerr "github.com/pkg/errors"
)

var (
	NotImplement = errors.New("not implement")
	NotSupport   = errors.New("not support")
	RelativePath = errors.New("access using relative path is not allowed")

	MoveBetweenTwoStorages = errors.New("can't move files between two storages, try to copy")
	UploadNotSupported     = errors.New("upload not supported")

	MetaNotFound     = errors.New("meta not found")
	StorageNotFound  = errors.New("storage not found")
	StreamIncomplete = errors.New("upload/download stream incomplete, possible network issue")
	StreamPeekFail   = errors.New("StreamPeekFail")

	UnknownArchiveFormat      = errors.New("unknown archive format")
	WrongArchivePassword      = errors.New("wrong archive password")
	DriverExtractNotSupported = errors.New("driver extraction not supported")

	//task
	TaskAlredyExists = errors.New("task already exists")

	// auth
	EmptyPassword = errors.New("empty password")
	WrongPassword = errors.New("wrong password")
)

// NewErr wrap constant error with an extra message
// use errors.Is(err1, StorageNotFound) to check if err belongs to any internal error
func NewErr(err error, format string, a ...any) error {
	return fmt.Errorf("%w; %s", err, fmt.Sprintf(format, a...))
}

func IsNotSupportError(err error) bool {
	return errors.Is(pkgerr.Cause(err), NotSupport)
}
func IsNotImplement(err error) bool {
	return errors.Is(pkgerr.Cause(err), NotImplement)
}
