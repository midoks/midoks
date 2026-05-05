package notify

import (
	"context"
	"fmt"
	"net/http"
	"net/url"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

type Notification struct {
	telegramBot *tgbotapi.BotAPI
	ChatID      int64
	Enabled     bool
}

func NewNotification(token string, chat_id int64, proxy string, enabled bool) (*Notification, error) {
	if !enabled {
		return &Notification{Enabled: false}, nil
	}

	var bot *tgbotapi.BotAPI
	var err error
	if proxy != "" {
		u, parseErr := url.Parse(proxy)
		if parseErr == nil {
			tr := &http.Transport{Proxy: http.ProxyURL(u)}
			client := &http.Client{Transport: tr}
			bot, err = tgbotapi.NewBotAPIWithClient(token, tgbotapi.APIEndpoint, client)
		} else {
			bot, err = tgbotapi.NewBotAPI(token)
		}
	} else {
		bot, err = tgbotapi.NewBotAPI(token)
	}
	if err != nil {
		return nil, fmt.Errorf("failed to create telegram bot: %w", err)
	}

	return &Notification{
		telegramBot: bot,
		ChatID:      chat_id,
		Enabled:     true,
	}, nil
}

func (n *Notification) Send(ctx context.Context, title, content string) error {
	if !n.Enabled {
		return nil
	}

	message := fmt.Sprintf("%s\n\n%s", title, content)
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
		msg := tgbotapi.NewMessage(n.ChatID, message)
		msg.ParseMode = "HTML"

		_, err := n.telegramBot.Send(msg)
		return err
	}
}
