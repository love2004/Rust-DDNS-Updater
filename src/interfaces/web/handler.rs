use actix_web::{web, get, Error};
use actix_files::NamedFile;
use std::path::PathBuf;
use log::{info, error};

/// 首頁處理器
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回首頁 HTML 文件
#[get("/")]
pub async fn index() -> Result<NamedFile, Error> {
    info!("接收到首頁請求");
    
    let path: PathBuf = PathBuf::from("static/index.html");
    match NamedFile::open(path) {
        Ok(file) => {
            info!("成功返回首頁文件");
            Ok(file)
        },
        Err(e) => {
            error!("無法打開首頁文件: {}", e);
            Err(Error::from(e))
        }
    }
}

/// 處理靜態資源
/// 
/// # 參數
/// 
/// - `path`: 資源路徑
/// 
/// # 返回
/// 
/// - `impl Responder`: 返回靜態資源文件
#[get("/{filename:.*}")]
pub async fn static_files(path: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = path.into_inner();
    info!("接收到靜態文件請求: {}", filename);
    
    let path = PathBuf::from(format!("static/{}", filename));
    match NamedFile::open(path) {
        Ok(file) => {
            info!("成功返回靜態文件: {}", filename);
            Ok(file)
        },
        Err(e) => {
            error!("無法打開靜態文件 {}: {}", filename, e);
            Err(Error::from(e))
        }
    }
}

/// 配置 Web UI 路由
/// 
/// # 參數
/// 
/// - `cfg`: 服務配置
/// 
/// # 功能
/// 
/// - 註冊 Web UI 路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    info!("配置Web UI路由");
    cfg.service(
        web::scope("/ui")
            .service(index)
            .service(static_files)
    );
} 