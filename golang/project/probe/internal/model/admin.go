package model

import (
	"fmt"

	"github.com/pkg/errors"

	"aiprobe/internal/errs"
	utils "aiprobe/internal/utils"
)

type Admin struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                      // unique key
	Username   string `json:"username" gorm:"unique" binding:"required"` // username
	Password   string `json:"password"`                                  // password
	Salt       string `json:"salt"`                                      // salt
	SuperAdmin bool   `json:"super_admin"`                               // super_admin
	AllowLogin bool   `json:"allow_login"`                               // allow_login
	FullName   string `json:"full_name"`                                 // full_name
	Auth       string `json:"auth"`                                      // auth
	Status     bool   `json:"status"`                                    // status
	CreateTime int64  `json:"create_time"`                               // create_time
	UpdateTime int64  `json:"update_time"`                               // update_time
}

func (u *Admin) ValidatePwdStaticHash(password string) error {
	if password == "" {
		return errors.WithStack(errs.EmptyPassword)
	}
	if u.Password != HashPwd(password, u.Salt) {
		return errors.WithStack(errs.WrongPassword)
	}
	return nil
}

func HashPwd(password string, salt string) string {
	return utils.HashData(utils.SHA256, []byte(fmt.Sprintf("%s-%s", password, salt)))
}

func TwoHashPwd(password string, salt string) string {
	return HashPwd(password, salt)
}
