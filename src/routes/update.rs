use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::path::Path;
use std::fs::{self};
use chrono::Local;


use crate::handlers::update_handler::{create_zip_archive,is_valid_zip};
#[derive(Deserialize)]
pub struct BakeupRequest {
    pub file_dir: String,
}

#[post("/bakeup")]
async fn bakeup(params: web::Json<BakeupRequest>) -> impl Responder {
    // 备份目录
    let bak_dir = "/Users/ldd/Workspaces/zip/bak-apk";
    // 源目录
    let src_dir = Path::new(&params.file_dir);
    
    // 检查源目录是否存在
    if !src_dir.exists() {
        return HttpResponse::NotFound().body(
            format!("没有找到此路径: {}", params.file_dir)
        );
    }
    
    // 检查备份目录是否存在，不存在则创建
    let bak_dir_path = Path::new(bak_dir);
    if !bak_dir_path.exists() {
        if let Err(e) = fs::create_dir_all(bak_dir_path) {
            return HttpResponse::InternalServerError().body(
                format!("备份文件不存在，且创建备份目录失败: {}", e)
            );
        }
    }
    
    // 生成备份文件名（使用当前时间戳和源目录名）
    let src_dir_name = src_dir.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("backup");
    
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let zip_filename = format!("{}_{}.zip", src_dir_name, timestamp);
    let zip_path = bak_dir_path.join(&zip_filename);
    
    // 创建ZIP文件
    match create_zip_archive(&src_dir, &zip_path) {
        Ok(_) => {
            // 检查ZIP文件是否创建成功
            if !zip_path.exists() {
                return HttpResponse::InternalServerError().body("备份文件创建失败");
            }
            
            // 验证ZIP文件是否有效
            if !is_valid_zip(&zip_path) {
                // 删除无效的ZIP文件
                let _ = fs::remove_file(&zip_path);
                return HttpResponse::InternalServerError().body("备份文件无效，可能压缩过程出错");
            }
            
            // 备份成功，可以选择删除源文件
            // 注意：这里只是示例，实际使用时请谨慎考虑是否需要删除源文件
            // if let Err(e) = fs::remove_dir_all(src_dir) {
            //     return HttpResponse::Ok().body(
            //         format!("备份成功，但删除源文件失败: {}. 备份文件路径: {}", e, zip_path.display())
            //     );
            // }
            
            HttpResponse::Ok().body(
                format!("备份成功，文件已保存至: {}", zip_path.display())
            )
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(
                format!("创建备份文件失败: {}", e)
            )
        }
    }
}
