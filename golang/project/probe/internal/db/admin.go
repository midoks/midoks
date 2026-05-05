package db

import (
	"time"

	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
	utils "aiprobe/internal/utils"
)

func GetAdminList(page, size int) ([]model.Admin, int64, error) {
	adminM := db.Model(&model.Admin{})
	var count int64
	if err := adminM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get server count")
	}

	var list []model.Admin
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetAdminByID(id int64) (*model.Admin, error) {
	var u model.Admin
	if err := db.First(&u, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get admin")
	}
	return &u, nil
}

func GetAdminByName(username string) (*model.Admin, error) {
	info := model.Admin{Username: username}
	if err := db.Where(info).First(&info).Error; err != nil {
		return nil, errors.Wrapf(err, "failed find admin")
	}
	return &info, nil
}

func AdminUpdateEmail(tx *gorm.DB, id int64, email string) error {
	if tx == nil {
		tx = db
	}
	return tx.Model(&model.Admin{ID: id}).Update("email", email).Error
}

func AdminUpdateTel(tx *gorm.DB, id int64, tel string) error {
	if tx == nil {
		tx = db
	}
	return tx.Model(&model.Admin{ID: id}).Update("tel", tel).Error
}

func AdminUpdatePass(tx *gorm.DB, id int64, password string) error {
	if tx == nil {
		tx = db
	}
	u := model.Admin{}
	u.ID = id

	if password != "" {
		salt := utils.RandString(16)
		u.Password = model.TwoHashPwd(password, salt)
		u.Salt = salt
	}
	u.UpdateTime = time.Now().Unix()
	return tx.Model(&u).Updates(map[string]interface{}{
		"password":    u.Password,
		"salt":        u.Salt,
		"update_time": u.UpdateTime,
	}).Error
}

func UpdateAdminModel(tx *gorm.DB, u *model.Admin) error {
	if tx == nil {
		tx = db
	}
	if u.Password == "" {
		if err := tx.Model(u).Updates(map[string]interface{}{"password": u.Password, "update_time": u.UpdateTime}).Error; err != nil {
			return errors.WithStack(err)
		}
	} else {
		if err := tx.Model(u).Updates(u).Error; err != nil {
			return errors.WithStack(err)
		}
	}
	return nil
}

func UpdateAdmin(tx *gorm.DB, id int64, username string, password string, full_name string, auth string, allow_login bool, super_admin bool) error {
	if tx == nil {
		tx = db
	}
	data := &model.Admin{}
	if err := tx.First(data, id).Error; err != nil {
		return errors.WithStack(err)
	}

	data.Username = username
	data.FullName = full_name
	data.AllowLogin = allow_login
	data.SuperAdmin = super_admin
	data.Auth = auth

	if password != "" {
		salt := utils.RandString(16)
		data.Password = model.TwoHashPwd(password, salt)
		data.Salt = salt
	}

	data.UpdateTime = time.Now().Unix()
	if err := errors.WithStack(tx.Save(data).Error); err != nil {
		return err
	}
	return nil
}

func AddAdmin(tx *gorm.DB, username string, password string, full_name string, auth string, allow_login bool, super_admin bool) error {
	if tx == nil {
		tx = db
	}
	salt := utils.RandString(16)

	pass := model.TwoHashPwd(password, salt)

	data := &model.Admin{
		Username:   username,
		Password:   pass,
		Salt:       salt,
		FullName:   full_name,
		AllowLogin: allow_login,
		SuperAdmin: super_admin,
		Auth:       auth,
	}

	data.CreateTime = time.Now().Unix()
	data.UpdateTime = time.Now().Unix()
	if err := errors.WithStack(tx.Create(data).Error); err != nil {
		return err
	}
	return nil
}

func AdminTriggerStatus(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var data model.Admin
	if err := tx.First(&data, id).Error; err != nil {
		return errors.Wrapf(err, "failed get cluster region")
	}

	var status bool
	if data.Status {
		status = false
	} else {
		status = true
	}

	data.UpdateTime = time.Now().Unix()
	data.Status = status

	if err := tx.Model(&model.Admin{}).
		Where("id = ?", id).
		Updates(&data).Error; err != nil {
		return err
	}
	return nil
}

func InitAdmin(user string, pass string) error {
	_, err := GetAdminByID(1)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {

			salt := utils.RandString(16)
			admin := &model.Admin{
				Username: user,
				Password: model.TwoHashPwd(pass, salt),
				Salt:     salt,
			}

			admin.CreateTime = time.Now().Unix()
			admin.UpdateTime = time.Now().Unix()
			if err := CreateAdmin(nil, admin); err != nil {
				return err
			}
		}
	}
	return nil
}

func AdminDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.Admin
	return tx.Where("id = ?", id).Delete(&d).Error
}

func CreateAdmin(tx *gorm.DB, u *model.Admin) error {
	if tx == nil {
		tx = db
	}
	return errors.WithStack(tx.Create(u).Error)
}
