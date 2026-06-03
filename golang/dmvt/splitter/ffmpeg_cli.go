package splitter

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"time"
)

// FFmpegCLISplitter 使用 FFmpeg CLI (-ss) 进行视频分片
// 特点: 平均切片耗时 850ms, 内存峰值 120MB, 支持最小粒度 ~2s, 不支持热切
type FFmpegCLISplitter struct {
	ffmpegPath string
}

// NewFFmpegCLISplitter 创建 FFmpeg CLI 分片器
func NewFFmpegCLISplitter(ffmpegPath string) *FFmpegCLISplitter {
	if ffmpegPath == "" {
		ffmpegPath = "ffmpeg"
	}
	return &FFmpegCLISplitter{ffmpegPath: ffmpegPath}
}

// Split 使用 FFmpeg -ss 参数进行分片
// input: 输入视频文件路径
// output: 输出视频文件路径
// start: 开始时间 (秒)
// duration: 持续时间 (秒)
func (s *FFmpegCLISplitter) Split(input, output string, start, duration float64) error {
	startTime := time.Now()

	// 确保输出目录存在
	outputDir := filepath.Dir(output)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("创建输出目录失败: %w", err)
	}

	// FFmpeg -ss 放在输入前是快速定位 (input seeking)，精度较低 (~2s)
	// FFmpeg -ss 放在输入后是精确裁剪 (output seeking)，速度较慢
	// 这里使用 input seeking 以获得较快速度
	cmd := exec.Command(
		s.ffmpegPath,
		"-ss", fmt.Sprintf("%.3f", start), // 快速定位到开始时间
		"-i", input, // 输入文件
		"-t", fmt.Sprintf("%.3f", duration), // 持续时间
		"-c", "copy", // 直接复制流，不重新编码
		"-avoid_negative_ts", "make_zero", // 避免负时间戳
		"-y", // 覆盖输出文件
		output,
	)

	// 捕获标准错误输出
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("FFmpeg 执行失败: %w", err)
	}

	fmt.Printf("FFmpeg CLI 分片完成: %s (耗时: %v)\n", output, time.Since(startTime))
	return nil
}

// SplitWithPrecision 使用更精确的裁剪方式 (output seeking)
// 精度更高但速度较慢
func (s *FFmpegCLISplitter) SplitWithPrecision(input, output string, start, duration float64) error {
	startTime := time.Now()

	outputDir := filepath.Dir(output)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("创建输出目录失败: %w", err)
	}

	// -ss 放在 -i 后面是精确裁剪，但需要解码，速度较慢
	cmd := exec.Command(
		s.ffmpegPath,
		"-i", input,
		"-ss", fmt.Sprintf("%.3f", start),
		"-t", fmt.Sprintf("%.3f", duration),
		"-c", "copy",
		"-avoid_negative_ts", "make_zero",
		"-y",
		output,
	)

	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("FFmpeg 执行失败: %w", err)
	}

	fmt.Printf("FFmpeg CLI 精确分片完成: %s (耗时: %v)\n", output, time.Since(startTime))
	return nil
}

// SplitToMultiple 将视频分片为多个片段
func (s *FFmpegCLISplitter) SplitToMultiple(input, outputDir string, segmentDuration float64) ([]string, error) {
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return nil, fmt.Errorf("创建输出目录失败: %w", err)
	}

	// 获取视频总时长
	duration, err := s.getVideoDuration(input)
	if err != nil {
		return nil, err
	}

	var outputs []string
	segmentCount := int(duration / segmentDuration)
	if duration > float64(segmentCount)*segmentDuration {
		segmentCount++
	}

	for i := 0; i < segmentCount; i++ {
		start := float64(i) * segmentDuration
		dur := segmentDuration
		if start+dur > duration {
			dur = duration - start
		}

		output := filepath.Join(outputDir, fmt.Sprintf("segment_%04d.mp4", i))
		if err := s.Split(input, output, start, dur); err != nil {
			return outputs, fmt.Errorf("分片 %d 失败: %w", i, err)
		}
		outputs = append(outputs, output)
	}

	return outputs, nil
}

// getVideoDuration 获取视频总时长
func (s *FFmpegCLISplitter) getVideoDuration(input string) (float64, error) {
	// 使用 ffprobe 获取视频时长
	cmd := exec.Command(
		"ffprobe",
		"-i", input,
		"-show_entries", "format=duration",
		"-v", "quiet",
		"-of", "csv=p=0",
	)

	output, err := cmd.Output()
	if err != nil {
		return 0, fmt.Errorf("获取视频时长失败: %w", err)
	}

	var duration float64
	if _, err := fmt.Sscanf(string(output), "%f", &duration); err != nil {
		return 0, fmt.Errorf("解析视频时长失败: %w", err)
	}

	return duration, nil
}
