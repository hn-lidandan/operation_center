use actix_web::web;
use actix_web::{HttpResponse, Responder, get, post};
use serde::Deserialize;
use serde_json;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust Web!")
}

#[get("/find_info_file")]
async fn find_info_file(path: web::Query<FilePathRequest>) -> impl Responder {
    // 1. 构建完整的文件路径
    let file_path = Path::new(&path.dir_path).join("version.txt");

    // 2. 检查文件是否存在
    if !file_path.exists() {
        return HttpResponse::NotFound().body(format!(
            "在路径 {} 下未找到 version.txt 文件",
            path.dir_path
        ));
    }

    // 3. 读取文件内容
    match std::fs::read_to_string(&file_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/plain").body(content),
        Err(e) => HttpResponse::InternalServerError().body(format!("读取文件失败: {}", e)),
    }
}

// 请求参数结构体
#[derive(Deserialize)]
pub struct FilePathRequest {
    dir_path: String, // 要搜索的目录路径
}

#[get("/find_setting_file")]
async fn find_setting_file(path: web::Query<FilePathRequest>) -> impl Responder {
    let file_path = Path::new(&path.dir_path).join("values/value.yml");
    if !file_path.exists() {
        return HttpResponse::NotFound().body(format!("未找到{:?}文件，请检查", file_path));
    }

    // 读取文件内容
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            // 解析 YAML 内容为 HashMap
            match serde_yaml::from_str::<HashMap<String, String>>(&contents) {
                Ok(yaml_map) => {
                    // 将 HashMap 转换为 JSON 返回
                    match serde_json::to_string(&yaml_map) {
                        Ok(json_str) => HttpResponse::Ok()
                            .content_type("application/json")
                            .body(json_str),
                        Err(e) => HttpResponse::InternalServerError()
                            .body(format!("将设置转换为JSON时出错: {}", e)),
                    }
                }
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("解析YAML文件时出错: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("读取文件时出错: {}", e)),
    }
}

#[derive(Deserialize, Debug)]
pub struct SaveSettingsRequest {
    #[serde(flatten)]
    settings: HashMap<String, String>,
}

#[post("/save_settings")]
async fn save_settings(
    settings: web::Json<SaveSettingsRequest>,
    path: web::Query<FilePathRequest>,
) -> impl Responder {
    info!("保存配置到路径: {}", path.dir_path);

    let dir_path = &path.dir_path;
    let file_path = Path::new(dir_path).join("values/value.yml");

    if !Path::new(dir_path).exists() {
        return HttpResponse::NotFound().body(format!("目录不存在: {}", dir_path));
    }

    // 确保 values 目录存在
    let values_dir = Path::new(dir_path).join("values");
    if !values_dir.exists() {
        match fs::create_dir_all(&values_dir) {
            Ok(_) => info!("创建目录: {:?}", values_dir),
            Err(e) => {
                return HttpResponse::InternalServerError().body(format!("创建目录失败: {}", e));
            }
        }
    }

    // 将设置转换为YAML格式
    match serde_yaml::to_string(&settings.settings) {
        Ok(yaml_str) => {
            info!("保存配置内容: {}", yaml_str);
            // 写入文件
            match fs::write(&file_path, yaml_str) {
                Ok(_) => HttpResponse::Ok().body("配置保存成功"),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("保存配置文件失败: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("转换配置为YAML失败: {}", e)),
    }
}
