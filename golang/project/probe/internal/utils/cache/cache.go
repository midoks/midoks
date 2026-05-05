package cache

import (
	"sync"
	"time"
)

// CacheItem represents a cached item with expiration
type CacheItem struct {
	Value      interface{}
	Expiration time.Time
}

// MemoryCache is a simple in-memory cache implementation
type MemoryCache struct {
	items map[string]CacheItem
	mu    sync.RWMutex
}

// NewMemoryCache creates a new memory cache
func NewMemoryCache() *MemoryCache {
	return &MemoryCache{
		items: make(map[string]CacheItem),
	}
}

// Set adds or updates an item in the cache with expiration time
func (c *MemoryCache) Set(key string, value interface{}, duration time.Duration) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.items[key] = CacheItem{
		Value:      value,
		Expiration: time.Now().Add(duration),
	}
}

// Get retrieves an item from the cache
func (c *MemoryCache) Get(key string) (interface{}, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()

	item, found := c.items[key]
	if !found {
		return nil, false
	}

	// Check if item has expired
	if time.Now().After(item.Expiration) {
		return nil, false
	}

	return item.Value, true
}

// Delete removes an item from the cache
func (c *MemoryCache) Delete(key string) {
	c.mu.Lock()
	defer c.mu.Unlock()

	delete(c.items, key)
}

// Cleanup removes expired items from the cache
func (c *MemoryCache) Cleanup() {
	c.mu.Lock()
	defer c.mu.Unlock()

	now := time.Now()
	for key, item := range c.items {
		if now.After(item.Expiration) {
			delete(c.items, key)
		}
	}
}

// Global cache instance
var globalCache *MemoryCache

func init() {
	globalCache = NewMemoryCache()

	// Start cleanup goroutine
	go func() {
		ticker := time.NewTicker(time.Minute * 5)
		defer ticker.Stop()

		for range ticker.C {
			globalCache.Cleanup()
		}
	}()
}

// SetGlobal sets a value in the global cache
func SetGlobal(key string, value interface{}, duration time.Duration) {
	globalCache.Set(key, value, duration)
}

// GetGlobal gets a value from the global cache
func GetGlobal(key string) (interface{}, bool) {
	return globalCache.Get(key)
}

// DeleteGlobal deletes a value from the global cache
func DeleteGlobal(key string) {
	globalCache.Delete(key)
}
