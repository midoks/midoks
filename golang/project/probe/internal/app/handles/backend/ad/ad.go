package ad

import (
	"fmt"
	"strconv"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
)

func Home(c *gin.Context) {
	data := common.CommonVer(c)

	fmt.Println("home")
	data["Positions"] = []string{"home_banner", "sidebar", "footer", "popup"}
	c.HTML(200, "backend/ad/index.tmpl", data)
}

func List(c *gin.Context) {
	pageStr := c.DefaultQuery("page", "1")
	page, _ := strconv.Atoi(pageStr)
	if page < 1 {
		page = 1
	}
	sizeStr := c.DefaultQuery("size", "10")
	size, _ := strconv.Atoi(sizeStr)
	if size < 1 {
		size = 10
	}
	position := c.DefaultQuery("position", "")

	list, total, err := db.GetAdList(page, size, position)
	if err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	c.JSON(200, common.LayuiResp[interface{}]{
		Code:  0,
		Count: total,
		Msg:   "success",
		Data:  list,
	})
}

func Add(c *gin.Context) {
	data := common.CommonVer(c)
	data["Positions"] = []string{"home_banner", "sidebar", "footer", "popup"}
	c.HTML(200, "backend/ad/add.tmpl", data)
}

func PostAdd(c *gin.Context) {
	var f form.AdForm
	if err := c.ShouldBind(&f); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	ad := &model.Ad{
		Title:     f.Title,
		Position:  f.Position,
		Content:   f.Content,
		Link:      f.Link,
		ImageURL:  f.ImageURL,
		Status:    f.Status,
		Sort:      f.Sort,
		StartTime: f.StartTime,
		EndTime:   f.EndTime,
	}

	if err := db.CreateAd(ad); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	common.SuccessResp(c, ad)
}

func Update(c *gin.Context) {
	data := common.CommonVer(c)
	idStr := c.Query("id")
	id, _ := strconv.ParseInt(idStr, 10, 64)

	ad, err := db.GetAdByID(id)
	if err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	data["Ad"] = ad
	data["Positions"] = []string{"home_banner", "sidebar", "footer", "popup"}
	c.HTML(200, "backend/ad/update.tmpl", data)
}

func PostUpdate(c *gin.Context) {
	var f form.AdForm
	if err := c.ShouldBind(&f); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	idStr := c.PostForm("id")
	id, _ := strconv.ParseInt(idStr, 10, 64)

	ad, err := db.GetAdByID(id)
	if err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	ad.Title = f.Title
	ad.Position = f.Position
	ad.Content = f.Content
	ad.Link = f.Link
	ad.ImageURL = f.ImageURL
	ad.Status = f.Status
	ad.Sort = f.Sort
	ad.StartTime = f.StartTime
	ad.EndTime = f.EndTime

	if err := db.UpdateAd(ad); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	common.SuccessResp(c, ad)
}

func Delete(c *gin.Context) {
	idStr := c.PostForm("id")
	id, _ := strconv.ParseInt(idStr, 10, 64)

	if err := db.DeleteAd(id); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	common.SuccessResp(c, nil)
}

func TriggerStatus(c *gin.Context) {
	idStr := c.PostForm("id")
	id, _ := strconv.ParseInt(idStr, 10, 64)

	ad, err := db.GetAdByID(id)
	if err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	if err := db.UpdateAdStatus(id, !ad.Status); err != nil {
		common.ErrorResp(c, err, 1)
		return
	}

	common.SuccessResp(c, nil)
}
