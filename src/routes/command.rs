use actix_web::web;
use actix_web::web::Bytes;
use actix_web::{HttpResponse, Responder, post};
use futures_util::stream::unfold;
use serde::Deserialize;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

// 请求参数结构体
#[derive(Deserialize)]
pub struct UnzipRequest {
    zip_path: String, // zip文件路径
}

#[post("/unzip")]
async fn cmd_unzip(req: web::Json<UnzipRequest>) -> impl Responder {
    println!("进入cmd_unzip");
    // 1. 检查文件是否存在
    let zip_path = Path::new(&req.zip_path);
    if !zip_path.exists() {
        return HttpResponse::BadRequest().body(format!("文件不存在: {}", req.zip_path));
    }

    // 2. 创建解压命令
    let mut command = Command::new("unzip");
    command
        .arg("-o") // 覆盖已存在的文件
        .arg(&req.zip_path)
        .arg("-d") // 指定解压目录
        .arg(zip_path.parent().unwrap_or(Path::new("."))) // 解压到同目录
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // 3. 执行解压命令
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                // 解压成功
                HttpResponse::Ok().body(format!(
                    "解压成功! 输出: {}",
                    String::from_utf8_lossy(&output.stdout)
                ))
            } else {
                // 解压失败
                HttpResponse::InternalServerError().body(format!(
                    "解压失败: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("执行解压命令失败: {}", e)),
    }
}

// 请求参数结构体
#[derive(Deserialize)]
pub struct FilePathRequest {
    dir_path: String, // 要搜索的目录路径
}

#[post("/setup")]
async fn cmd_setup(path: web::Json<FilePathRequest>) -> impl Responder {
    println!("进入cmd_setup,path:{}", path.dir_path);

    // 创建通道用于从线程发送数据到异步流
    let (tx, rx) = mpsc::channel::<Result<Bytes, std::io::Error>>(100);

    // 克隆路径用于线程
    let dir_path = path.dir_path.clone();

    // 在单独的线程中执行命令并实时发送输出
    thread::spawn(move || {
        println!("Starting command execution thread");

        // 创建命令
        let mut command = Command::new("gflow");
        command
            .arg("setup")
            // .arg("-e")
            // .arg("local,arm_mac")
            .current_dir(&dir_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        println!("Starting gflow setup in directory: {}...", dir_path);

        // 执行命令
        match command.spawn() {
            Ok(mut child) => {
                // 获取标准输出
                if let Some(stdout) = child.stdout.take() {
                    // 创建一个缓冲读取器，使用小缓冲区以便更快发送数据
                    let mut reader = BufReader::with_capacity(1, stdout);
                    let mut buffer = [0; 1];

                    // 逐字节读取并立即发送
                    while let Ok(n) = reader.read(&mut buffer) {
                        if n == 0 {
                            break; // EOF
                        }

                        // 将字节转换为字符串并发送
                        let data = Bytes::copy_from_slice(&buffer[..n]);
                        println!("Sending byte: {:?}", data);

                        // 尝试发送数据，如果接收端已关闭则退出
                        if tx.blocking_send(Ok(data)).is_err() {
                            println!("Receiver closed, stopping command output");
                            break;
                        }
                    }
                }

                // 等待命令完成
                match child.wait() {
                    Ok(status) => println!("Command completed with status: {}", status),
                    Err(e) => println!("Failed to wait for command: {}", e),
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to spawn command: {}", e);
                println!("{}", error_msg);
                let _ = tx.blocking_send(Ok(Bytes::from(error_msg)));
            }
        }

        println!("Command execution thread finished");
    });

    // 将接收器转换为流
    let stream = ReceiverStream::new(rx);

    println!("Stream created, sending response...");

    // 返回流式响应
    HttpResponse::Ok()
        .insert_header(("X-Accel-Buffering", "no"))
        .insert_header(("Cache-Control", "no-cache, no-transform"))
        .content_type("text/plain")
        .streaming(stream)
}

#[post("/localize")]
async fn cmd_localize(path: web::Json<FilePathRequest>) -> impl Responder {
    println!("进入cmd_localize,path:{}", path.dir_path);
    // 创建 gflow setup 命令，并设置工作目录
    let mut command = Command::new("ds-sys");
    command
        .arg("localize")
        // .arg("-e")
        // .arg("local,arm_mac")
        .current_dir(&path.dir_path) // 设置工作目录为用户指定的路径
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    println!(
        "Starting gflow cmd_localize in directory: {}...",
        path.dir_path
    ); // 调试日志

    // 添加错误处理
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to spawn command: {}", e); // 错误日志
            return HttpResponse::InternalServerError()
                .body(format!("Failed to start command: {}", e));
        }
    };

    println!("Command started successfully"); // 调试日志

    // 获取标准输出（添加错误处理）
    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            eprintln!("Failed to capture stdout"); // 错误日志
            return HttpResponse::InternalServerError().body("Failed to capture stdout");
        }
    };

    println!("Stdout captured successfully"); // 调试日志

    let reader = BufReader::new(stdout);

    //创建一个异步流
    let stream = unfold(reader, |mut reader| async move {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(n) => {
                println!("Read {} bytes: {}", n, line); // 调试日志
                Some((Ok(Bytes::from(line)), reader))
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e); // 错误日志
                Some((Err(e), reader))
            }
        }
    });

    println!("Stream created, sending response..."); // 调试日志

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(stream)
}
