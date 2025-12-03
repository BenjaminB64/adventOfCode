package testloghandler

import (
	"context"
	"log/slog"
	"testing"
)

type TestLogHandler struct {
	T     *testing.T
	Level slog.Level
}

func (h *TestLogHandler) Enabled(_ context.Context, lvl slog.Level) bool {
	return lvl >= h.Level
}

func (h *TestLogHandler) Handle(_ context.Context, r slog.Record) error {
	attrs := make([]any, 0, 4)
	r.Attrs(func(a slog.Attr) bool {
		attrs = append(attrs, a)
		return true
	})

	h.T.Logf("[%s] %s %v", r.Level, r.Message, attrs)

	return nil
}

func (h *TestLogHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	return h
}

func (h *TestLogHandler) WithGroup(name string) slog.Handler {
	return h
}
