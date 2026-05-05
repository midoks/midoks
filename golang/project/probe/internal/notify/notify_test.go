package notify

import (
	"context"
	"os"
	"strconv"
	"testing"
	"time"
)

// export TELEGRAM_TOKEN="您的BotToken"
// export TELEGRAM_CHAT_ID="您的ChatID"
// go test -v internal/notify/notify_test.go internal/notify/notify.go

func TestNotification_Send(t *testing.T) {
	// 尝试从环境变量获取配置，如果没有则跳过真实发送测试
	token := os.Getenv("TELEGRAM_TOKEN")
	chatIDStr := os.Getenv("TELEGRAM_CHAT_ID")

	if token == "" || chatIDStr == "" {
		t.Log("Skipping real sending test because TELEGRAM_TOKEN or TELEGRAM_CHAT_ID is not set")

		// 测试禁用状态
		n, err := NewNotification("dummy_token", 0, "", false)
		if err != nil {
			t.Fatalf("Failed to create disabled notification: %v", err)
		}

		err = n.Send(context.Background(), "Test Title", "Test Content")
		if err != nil {
			t.Errorf("Send() failed in disabled mode: %v", err)
		}
		return
	}

	chatID, err := strconv.ParseInt(chatIDStr, 10, 64)
	if err != nil {
		t.Fatalf("Invalid TELEGRAM_CHAT_ID: %v", err)
	}

	// 测试真实发送
	n, err := NewNotification(token, chatID, "", true)
	if err != nil {
		t.Fatalf("Failed to create notification: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	err = n.Send(ctx, "Test Title", "This is a test message from automated test.")
	if err != nil {
		t.Errorf("Send() failed: %v", err)
	} else {
		t.Log("Message sent successfully")
	}
}
