use rand::Rng;
use std::fs::{remove_file, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::Instant;

struct DiskBenchmark {
    file_path: String,
    file_size: usize,
    block_size: usize,
}

impl DiskBenchmark {
    fn new(file_path: &str, file_size: usize, block_size: usize) -> Self {
        Self {
            file_path: file_path.to_string(),
            file_size,
            block_size,
        }
    }

    fn prepare(&self) -> std::io::Result<()> {
        let _ = remove_file(&self.file_path); // 删除旧测试文件
        let file = File::create(&self.file_path)?;

        // 预分配空间
        file.set_len(self.file_size as u64)?;

        Ok(())
    }

    fn sequential_write(&self) -> std::io::Result<f64> {
        let mut file = File::create(&self.file_path)?;
        let buffer = vec![0u8; self.block_size];
        let blocks = self.file_size / self.block_size;

        let start = Instant::now();

        for _ in 0..blocks {
            file.write_all(&buffer)?;
        }

        file.sync_all()?;

        let duration = start.elapsed();
        Ok((self.file_size as f64 / 1024.0 / 1024.0) / duration.as_secs_f64())
    }

    fn sequential_read(&self) -> std::io::Result<f64> {
        let mut file = File::open(&self.file_path)?;
        let mut buffer = vec![0u8; self.block_size];
        let blocks = self.file_size / self.block_size;

        let start = Instant::now();

        for _ in 0..blocks {
            file.read_exact(&mut buffer)?;
        }

        let duration = start.elapsed();
        Ok((self.file_size as f64 / 1024.0 / 1024.0) / duration.as_secs_f64())
    }

    fn random_read(&self, iterations: usize) -> std::io::Result<f64> {
        let mut file = File::open(&self.file_path)?;
        let mut buffer = vec![0u8; self.block_size];
        let mut rng = rand::rng();
        let max_offset = self.file_size - self.block_size;

        let start = Instant::now();

        for _ in 0..iterations {
            let offset = rng.random_range(0..=max_offset) as u64;
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut buffer)?;
        }

        let duration = start.elapsed();
        let total_bytes = (self.block_size * iterations) as f64;
        Ok((total_bytes / 1024.0 / 1024.0) / duration.as_secs_f64())
    }

    fn cleanup(&self) -> std::io::Result<()> {
        remove_file(&self.file_path)
    }
}

fn main() -> std::io::Result<()> {
    let benchmark = DiskBenchmark::new("/tmp/disk_benchmark.bin", 1024 * 1024 * 100, 4096); // 100MB文件，4KB块

    benchmark.prepare()?;

    println!("开始硬盘性能测试...");

    let write_speed = benchmark.sequential_write()?;
    println!("顺序写入速度: {:.2} MB/s", write_speed);

    let read_speed = benchmark.sequential_read()?;
    println!("顺序读取速度: {:.2} MB/s", read_speed);

    let random_read = benchmark.random_read(10000)?; // 10000次随机读取
    println!("随机读取速度: {:.2} MB/s", random_read);

    benchmark.cleanup()?;

    Ok(())
}
