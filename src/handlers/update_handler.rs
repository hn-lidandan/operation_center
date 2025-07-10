use std::path::Path;
use std::fs::{ File};
use std::io::{self, Read, Write};
use zip::{write::FileOptions, ZipWriter};
use walkdir::WalkDir;
use std::process::Command;



// 验证ZIP文件是否有效
pub fn is_valid_zip(zip_path: &Path) -> bool {
    // 使用unzip命令测试ZIP文件是否有效
    let output = Command::new("unzip")
        .arg("-t")  // 测试模式
        .arg(zip_path)
        .output();
        
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

// 创建ZIP归档
pub fn create_zip_archive(src_dir: &Path, zip_path: &Path) -> io::Result<()> {
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    let src_dir_str = src_dir.to_string_lossy().to_string();
    let src_dir_len = src_dir_str.len() + if src_dir_str.ends_with('/') { 0 } else { 1 };
    
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.to_string_lossy();
        
        // 跳过源目录本身
        if path == src_dir {
            continue;
        }
        
        // 计算相对路径
        let relative_path = if name.len() > src_dir_len {
            Path::new(&name[src_dir_len..])
        } else {
            Path::new(path.file_name().unwrap_or_default().to_str().unwrap_or_default())
        };
        
        if path.is_file() {
            // 添加文件到ZIP
            zip.start_file(relative_path.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() && !name.ends_with('/') {
            // 添加目录到ZIP
            let dir_path = format!("{}/", relative_path.to_string_lossy());
            zip.add_directory(dir_path, options)?;
        }
    }
    
    zip.finish()?;
    Ok(())
}