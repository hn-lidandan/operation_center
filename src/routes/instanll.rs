use actix_web::web;
use actix_web::{HttpResponse, Responder, get, post};
use serde::Deserialize;
use serde_json;
use serde_yaml;
use std::collections::HashMap;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;
use tracing::info;

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


#[get("/find_setting")]
async fn find_setting(path: web::Query<FilePathRequest>) -> impl Responder {
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

#[get("/find_settings")]
async fn find_settings(path: web::Query<FilePathRequest>) -> impl Responder {
    let dir_path = Path::new(&path.dir_path).join("values");
    if !dir_path.exists() {
        return HttpResponse::NotFound().body(format!("未找到目录:{:?}，请检查", dir_path));
    }

    // 存储所有配置文件的内容
    let mut all_settings: HashMap<String, IndexMap<String, String>> = HashMap::new();

    // 加载目录下所有.yml文件
    match std::fs::read_dir(&dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(extension) = file_path.extension() {
                            if extension == "yml" || extension == "yaml" {
                                if let Some(file_name) = file_path.file_name() {
                                    if let Some(file_name_str) = file_name.to_str() {
                                        load_yaml_file(&dir_path, file_name_str, &mut all_settings);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("读取目录时出错: {}", e));
        }
    }

    respond_with_settings(all_settings)
}

// 从文件加载YAML内容并添加到settings映射中
fn load_yaml_file(dir_path: &Path, file_name: &str, all_settings: &mut HashMap<String, IndexMap<String, String>>) {
    let file_path = dir_path.join(file_name);
    if !file_path.exists() {
        // 记录文件不存在但继续处理
        all_settings.insert(file_name.to_string(), IndexMap::new());
        return;
    }

    // 读取文件内容
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            // 解析 YAML 内容为 IndexMap 保持顺序
            match serde_yaml::from_str::<IndexMap<String, String>>(&contents) {
                Ok(yaml_map) => {
                    // 将解析后的配置添加到结果中
                    all_settings.insert(file_name.to_string(), yaml_map);
                }
                Err(e) => {
                    // 解析错误时添加空配置并记录错误
                    all_settings.insert(file_name.to_string(), IndexMap::new());
                    eprintln!("解析YAML文件 {} 时出错: {}", file_name, e);
                }
            }
        }
        Err(e) => {
            // 读取错误时添加空配置并记录错误
            all_settings.insert(file_name.to_string(), IndexMap::new());
            eprintln!("读取文件 {} 时出错: {}", file_name, e);
        }
    }
}

// 将设置转换为JSON并返回
fn respond_with_settings(all_settings: HashMap<String, IndexMap<String, String>>) -> HttpResponse {
    match serde_json::to_string(&all_settings) {
        Ok(json_str) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json_str),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("将设置转换为JSON时出错: {}", e)),
    }
}

#[derive(Deserialize, Debug)]
pub struct SaveSettingsRequest {
    file_name: Option<String>, // 要保存的文件名，不包含路径
    settings: IndexMap<String, String>, // 配置项作为一个独立的字段，使用IndexMap保持顺序
}

#[post("/save_setting")]
async fn save_setting(
    settings: web::Json<SaveSettingsRequest>,
    path: web::Query<FilePathRequest>,
) -> impl Responder {
    let dir_path = &path.dir_path;
    
    // 确定文件名，默认为value.yml
    let file_name = settings.file_name.clone().unwrap_or_else(|| "value.yml".to_string());
    
    info!("保存配置到路径: {}, 文件: {}", dir_path, file_name);

    let values_dir = Path::new(dir_path).join("values");
    let file_path = values_dir.join(&file_name);

    if !Path::new(dir_path).exists() {
        return HttpResponse::NotFound().body(format!("目录不存在: {}", dir_path));
    }

    // 确保 values 目录存在
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
                Ok(_) => HttpResponse::Ok().body(format!("配置保存成功到文件 {}", file_name)),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("保存配置文件失败: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("转换配置为YAML失败: {}", e)),
    }
}
