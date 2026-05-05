package form

type AdminRecipientsInstances struct {
	ID                  int64  `form:"id"`
	Name                string `form:"name" binding:"required"`
	MediaType           string `form:"media_type" binding:"required"`
	Mark                string `form:"mark"`
	HashLife            int64  `form:"hash_life"`
	Token               string `form:"token"`
	SendID              int64  `form:"send_id"`
	TelegramProxyScheme string `form:"telegram_proxy_scheme"`
	TelegramProxyValue  string `form:"telegram_proxy_value"`
	EmailSmtp           string `form:"email_smtp"`
	EmailUsername       string `form:"email_username"`
	EmailPassword       string `form:"email_password"`
	EmailFrom           string `form:"email_from"`
	WebhookUrl          string `form:"webhook_url"`
	WebhookMethod       string `form:"webhook_method"`
	Count               int64  `form:"count" binding:"required"`
	Minutes             int64  `form:"minutes" binding:"required"`
	Status              bool   `form:"status"`
}

type AdminRecipientsInstancesTest struct {
	ID            int64  `form:"id"`
	MediaType     string `form:"media_type" binding:"required"`
	Title         string `form:"title"`
	Content       string `form:"content"`
	SendID        int64  `form:"send_id"`
	EmailSmtp     string `form:"email_smtp"`
	EmailUsername string `form:"email_username"`
	EmailPassword string `form:"email_password"`
	EmailFrom     string `form:"email_from"`
	WebhookUrl    string `form:"webhook_url"`
	WebhookMethod string `form:"webhook_method"`
}
