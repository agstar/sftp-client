use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::sync::atomic::AtomicBool;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::net::TcpStream;
use std::io::{Write, Read};
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use tauri::Emitter;

// SFTP 连接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpConnectionInfo {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub connected: bool,
}

// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<String>,
    pub permissions: String,
}

// 传输进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    pub id: String,
    pub filename: String,
    pub total_size: u64,
    pub transferred: u64,
    pub speed: f64,
    pub status: String,
}

// 全局连接管理器
type ConnectionManager = Mutex<HashMap<String, Session>>;
static CONNECTIONS: std::sync::LazyLock<ConnectionManager> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

// 存储活跃的传输任务
type TransferTasks = Mutex<HashMap<String, Arc<AtomicBool>>>;
static TRANSFER_TASKS: std::sync::LazyLock<TransferTasks> = 
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

// 测试连接命令
#[tauri::command]
async fn test_sftp_connection(
    host: String,
    port: u16,
    username: String,
    password: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let tcp = TcpStream::connect(format!("{}:{}", host, port))
            .map_err(|e| format!("连接失败: {}", e))?;

        let mut session = Session::new()
            .map_err(|e| format!("创建会话失败: {}", e))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| format!("握手失败: {}", e))?;

        session.userauth_password(&username, &password)
            .map_err(|e| format!("认证失败: {}", e))?;

        if !session.authenticated() {
            return Err("认证失败".to_string());
        }

        Ok("连接成功".to_string())
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 建立 SFTP 连接
#[tauri::command]
async fn connect_sftp(
    connection_info: SftpConnectionInfo,
) -> Result<String, String> {
    let connection_id = connection_info.id.clone();

    tokio::task::spawn_blocking(move || {
        let tcp = TcpStream::connect(format!("{}:{}", connection_info.host, connection_info.port))
            .map_err(|e| format!("连接失败: {}", e))?;

        let mut session = Session::new()
            .map_err(|e| format!("创建会话失败: {}", e))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| format!("握手失败: {}", e))?;

        session.userauth_password(&connection_info.username, &connection_info.password)
            .map_err(|e| format!("认证失败: {}", e))?;

        if !session.authenticated() {
            return Err("认证失败".to_string());
        }

        // 存储连接
        let mut connections = CONNECTIONS.lock().unwrap();
        connections.insert(connection_id.clone(), session);

        Ok(connection_id)
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 列出目录内容
#[tauri::command]
async fn list_directory(
    connection_id: String,
    path: String,
) -> Result<Vec<FileInfo>, String> {
    println!("=== 开始列出目录 ===");
    println!("请求路径: '{}' (连接ID: {})", path, connection_id);

    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        println!("当前活跃连接数: {}", connections.len());

        let session = connections.get(&connection_id)
            .ok_or_else(|| {
                println!("错误: 连接不存在，连接ID: {}", connection_id);
                println!("可用连接ID: {:?}", connections.keys().collect::<Vec<_>>());
                format!("连接不存在: {}", connection_id)
            })?;

        println!("会话已找到，创建 SFTP 连接...");
        let sftp = session.sftp()
            .map_err(|e| {
                println!("创建 SFTP 会话失败: {}", e);
                format!("创建 SFTP 会话失败: {}", e)
            })?;

        // 验证路径格式
        let normalized_path = if path.is_empty() || path == "/" {
            "/"
        } else {
            &path
        };

        println!("标准化路径: '{}' -> '{}'", path, normalized_path);

        // 尝试获取当前工作目录
        match sftp.realpath(Path::new(".")) {
            Ok(real_path) => println!("当前工作目录: {}", real_path.display()),
            Err(e) => println!("无法获取当前工作目录: {}", e),
        }

        println!("尝试读取目录: '{}'", normalized_path);
        let entries = sftp.readdir(Path::new(normalized_path))
            .map_err(|e| {
                let error_msg = format!("读取目录失败 [{}]: {}", normalized_path, e);
                println!("{}", error_msg);

                // 尝试列出根目录作为备选
                if normalized_path != "/" {
                    println!("尝试读取根目录作为备选...");
                    match sftp.readdir(Path::new("/")) {
                        Ok(root_entries) => {
                            println!("根目录包含 {} 个条目", root_entries.len());
                        }
                        Err(root_err) => {
                            println!("根目录也无法读取: {}", root_err);
                        }
                    }
                }

                error_msg
            })?;

        println!("原始条目数量: {}", entries.len());

        let mut files = Vec::new();
        for (index, (entry_path, stat)) in entries.iter().enumerate() {
            println!("处理条目 {}: {:?}", index + 1, entry_path);

            // 获取文件名，支持 Windows 路径格式
            let name = if let Some(file_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                println!("  标准文件名: '{}'", file_name);
                file_name.to_string()
            } else {
                // 处理 Windows 盘符或特殊路径
                let path_str = entry_path.to_string_lossy();
                println!("  原始路径: '{}'", path_str);

                // 检查是否是 Windows 盘符 (如 "C:", "D:" 等)
                if path_str.len() == 2 && path_str.ends_with(':') {
                    let drive_name = format!("{}盘", &path_str[0..1]);
                    println!("  识别为 Windows 盘符: '{}' -> '{}'", path_str, drive_name);
                    drive_name
                } else if path_str.contains('\\') {
                    // Windows 路径，提取最后一部分
                    let parts: Vec<&str> = path_str.split('\\').collect();
                    let last_part = parts.last().map(|s| s.to_string()).unwrap_or_else(|| path_str.to_string());
                    println!("  Windows 路径，提取: '{}'", last_part);
                    if last_part.is_empty() {
                        path_str.to_string()
                    } else {
                        last_part
                    }
                } else {
                    // 使用完整路径作为名称
                    println!("  使用完整路径作为名称: '{}'", path_str);
                    path_str.to_string()
                }
            };

            let file_info = FileInfo {
                name: name.clone(),
                path: entry_path.to_string_lossy().to_string(),
                size: stat.size.unwrap_or(0),
                is_dir: stat.is_dir(),
                modified: None, // 可以后续添加时间解析
                permissions: format!("{:o}", stat.perm.unwrap_or(0)),
            };

            println!("  -> 添加文件: {} (路径: {}, 目录: {}, 大小: {})",
                file_info.name, file_info.path, file_info.is_dir, file_info.size);
            files.push(file_info);
        }

        println!("=== 目录读取完成 ===");
        println!("最终文件数量: {} (原始: {})", files.len(), entries.len());

        if files.is_empty() {
            println!("警告: 目录为空！可能的原因:");
            println!("1. 目录确实为空");
            println!("2. 权限不足");
            println!("3. 路径不正确");
            println!("4. 文件名编码问题");
        }

        Ok(files)
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 下载文件（带进度更新）
#[tauri::command]
async fn download_file_with_progress(
    app_handle: tauri::AppHandle,
    connection_id: String,
    remote_path: String,
    local_path: String,
    transfer_id: String,
) -> Result<String, String> {
    println!("开始下载文件: {} -> {}", remote_path, local_path);

    // 创建取消标志
    let cancel_flag = Arc::new(AtomicBool::new(false));
    {
        let mut tasks = TRANSFER_TASKS.lock().unwrap();
        tasks.insert(transfer_id.clone(), cancel_flag.clone());
    }

    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        // 确保本地目录存在
        let local_path_obj = Path::new(&local_path);
        if let Some(parent_dir) = local_path_obj.parent() {
            println!("创建目录: {:?}", parent_dir);
            fs::create_dir_all(parent_dir)
                .map_err(|e| format!("创建本地目录失败: {}", e))?;
        }

        println!("打开远程文件: {}", remote_path);
        let mut remote_file = sftp.open(Path::new(&remote_path))
            .map_err(|e| format!("打开远程文件失败: {}", e))?;

        // 获取文件大小
        let file_stat = sftp.stat(Path::new(&remote_path))
            .map_err(|e| format!("获取文件信息失败: {}", e))?;
        let total_size = file_stat.size.unwrap_or(0);

        println!("创建本地文件: {}", local_path);
        let mut local_file = std::fs::File::create(&local_path)
            .map_err(|e| format!("创建本地文件失败: {}", e))?;

        println!("开始传输文件，总大小: {} 字节", total_size);

        // 分块传输并更新进度
        let mut buffer = [0u8; 8192]; // 8KB 缓冲区
        let mut bytes_copied = 0u64;
        let mut last_progress_update = std::time::Instant::now();

        loop {
            // 检查是否取消
            if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                // 计算当前进度
                let progress = if total_size > 0 {
                    (bytes_copied as f64 / total_size as f64 * 100.0) as u32
                } else {
                    0
                };

                // 发送取消事件
                let _ = app_handle.emit("download_progress", serde_json::json!({
                    "transfer_id": transfer_id,
                    "bytes_copied": bytes_copied,
                    "total_size": total_size,
                    "progress": progress,
                    "cancelled": true
                }));
                return Err("传输已取消".to_string());
            }
            
            match remote_file.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    local_file.write_all(&buffer[..n])
                        .map_err(|e| format!("写入本地文件失败: {}", e))?;

                    bytes_copied += n as u64;

                    // 每100ms或每1MB更新一次进度
                    if last_progress_update.elapsed().as_millis() >= 100 || bytes_copied % (1024 * 1024) == 0 {
                        let progress = if total_size > 0 {
                            (bytes_copied as f64 / total_size as f64 * 100.0) as u32
                        } else {
                            0
                        };

                        // 发送进度更新事件
                        let _ = app_handle.emit("download_progress", serde_json::json!({
                            "transfer_id": transfer_id,
                            "bytes_copied": bytes_copied,
                            "total_size": total_size,
                            "progress": progress
                        }));

                        last_progress_update = std::time::Instant::now();
                        println!("下载进度: {}/{} 字节 ({}%)", bytes_copied, total_size, progress);
                    }
                }
                Err(e) => return Err(format!("读取远程文件失败: {}", e)),
            }
        }

        println!("文件传输完成，传输字节数: {}", bytes_copied);

        // 发送完成事件
        let _ = app_handle.emit("download_progress", serde_json::json!({
            "transfer_id": transfer_id,
            "bytes_copied": bytes_copied,
            "total_size": total_size,
            "progress": 100,
            "completed": true
        }));

        Ok(format!("下载完成，文件大小: {} 字节", bytes_copied))
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 保留原有的下载函数作为备用
#[tauri::command]
async fn download_file(
    connection_id: String,
    remote_path: String,
    local_path: String,
) -> Result<String, String> {
    println!("开始下载文件: {} -> {}", remote_path, local_path);

    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        // 确保本地目录存在
        let local_path_obj = Path::new(&local_path);
        if let Some(parent_dir) = local_path_obj.parent() {
            println!("创建目录: {:?}", parent_dir);
            fs::create_dir_all(parent_dir)
                .map_err(|e| format!("创建本地目录失败: {}", e))?;
        }

        println!("打开远程文件: {}", remote_path);
        let mut remote_file = sftp.open(Path::new(&remote_path))
            .map_err(|e| format!("打开远程文件失败: {}", e))?;

        println!("创建本地文件: {}", local_path);
        let mut local_file = std::fs::File::create(&local_path)
            .map_err(|e| format!("创建本地文件失败: {}", e))?;

        println!("开始传输文件...");
        let bytes_copied = std::io::copy(&mut remote_file, &mut local_file)
            .map_err(|e| format!("文件传输失败: {}", e))?;

        println!("文件传输完成，传输字节数: {}", bytes_copied);
        Ok(format!("下载完成，文件大小: {} 字节", bytes_copied))
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 上传文件
#[tauri::command]
async fn upload_file(
    connection_id: String,
    local_path: String,
    remote_path: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        let mut local_file = std::fs::File::open(&local_path)
            .map_err(|e| format!("打开本地文件失败: {}", e))?;

        let mut remote_file = sftp.create(Path::new(&remote_path))
            .map_err(|e| format!("创建远程文件失败: {}", e))?;

        std::io::copy(&mut local_file, &mut remote_file)
            .map_err(|e| format!("文件传输失败: {}", e))?;

        Ok("上传完成".to_string())
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 上传文件数据（从前端传来的 base64 数据）
#[tauri::command]
async fn upload_file_data(
    connection_id: String,
    remote_path: String,
    file_data: String,
    file_name: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        // 解码 base64 数据
        let decoded_data = general_purpose::STANDARD.decode(&file_data)
            .map_err(|e| format!("解码文件数据失败: {}", e))?;

        let mut remote_file = sftp.create(Path::new(&remote_path))
            .map_err(|e| format!("创建远程文件失败: {}", e))?;

        remote_file.write_all(&decoded_data)
            .map_err(|e| format!("写入文件数据失败: {}", e))?;

        Ok(format!("文件 {} 上传完成", file_name))
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 创建目录
#[tauri::command]
async fn create_directory(
    connection_id: String,
    path: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        sftp.mkdir(Path::new(&path), 0o755)
            .map_err(|e| format!("创建目录失败: {}", e))?;

        Ok("目录创建成功".to_string())
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 删除文件或目录
#[tauri::command]
async fn delete_file(
    connection_id: String,
    path: String,
    is_dir: bool,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        if is_dir {
            sftp.rmdir(Path::new(&path))
                .map_err(|e| format!("删除目录失败: {}", e))?;
        } else {
            sftp.unlink(Path::new(&path))
                .map_err(|e| format!("删除文件失败: {}", e))?;
        }

        Ok("删除成功".to_string())
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 获取连接状态和信息
#[tauri::command]
async fn get_connection_info(connection_id: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();

        if let Some(session) = connections.get(&connection_id) {
            let sftp = session.sftp()
                .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

            // 尝试获取当前工作目录
            match sftp.realpath(Path::new(".")) {
                Ok(cwd) => Ok(format!("连接活跃，当前目录: {}", cwd.display())),
                Err(e) => Ok(format!("连接活跃，但无法获取当前目录: {}", e)),
            }
        } else {
            Err(format!("连接不存在: {}", connection_id))
        }
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 获取默认下载目录
#[tauri::command]
async fn get_downloads_directory() -> Result<String, String> {
    // 获取用户的下载目录
    let downloads_dir = if cfg!(target_os = "windows") {
        // Windows: 使用用户目录下的 Downloads 文件夹
        match std::env::var("USERPROFILE") {
            Ok(user_profile) => format!("{}\\Downloads", user_profile),
            Err(_) => {
                // 备选方案：使用当前目录下的 downloads 文件夹
                let current_dir = std::env::current_dir()
                    .map_err(|e| format!("无法获取当前目录: {}", e))?;
                format!("{}\\downloads", current_dir.display())
            }
        }
    } else {
        // Unix/Linux/macOS: 使用用户目录下的 Downloads 文件夹
        match std::env::var("HOME") {
            Ok(home) => format!("{}/Downloads", home),
            Err(_) => {
                // 备选方案：使用当前目录下的 downloads 文件夹
                let current_dir = std::env::current_dir()
                    .map_err(|e| format!("无法获取当前目录: {}", e))?;
                format!("{}/downloads", current_dir.display())
            }
        }
    };

    println!("默认下载目录: {}", downloads_dir);

    // 确保目录存在
    fs::create_dir_all(&downloads_dir)
        .map_err(|e| format!("创建下载目录失败: {}", e))?;

    Ok(downloads_dir)
}

// 断开连接
#[tauri::command]
async fn disconnect_sftp(connection_id: String) -> Result<String, String> {
    let mut connections = CONNECTIONS.lock().unwrap();
    connections.remove(&connection_id);
    Ok("连接已断开".to_string())
}

// 打开文件夹
#[tauri::command]
async fn open_file_folder(path: String) -> Result<String, String> {
    println!("打开文件夹: {}", path);

    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let result = Command::new("explorer")
                .arg(&path)
                .spawn();

            match result {
                Ok(_) => Ok(format!("已打开文件夹: {}", path)),
                Err(e) => Err(format!("打开文件夹失败: {}", e))
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let result = Command::new("open")
                .arg(&path)
                .spawn();

            match result {
                Ok(_) => Ok(format!("已打开文件夹: {}", path)),
                Err(e) => Err(format!("打开文件夹失败: {}", e))
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let result = Command::new("xdg-open")
                .arg(&path)
                .spawn();

            match result {
                Ok(_) => Ok(format!("已打开文件夹: {}", path)),
                Err(e) => Err(format!("打开文件夹失败: {}", e))
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err("不支持的操作系统".to_string())
        }
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}

// 取消传输任务
#[tauri::command]
async fn cancel_transfer(transfer_id: String) -> Result<String, String> {
    println!("取消传输: {}", transfer_id);

    let mut tasks = TRANSFER_TASKS.lock().unwrap();
    if let Some(cancel_flag) = tasks.get(&transfer_id) {
        cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        tasks.remove(&transfer_id);
        Ok(format!("传输任务 {} 已取消", transfer_id))
    } else {
        Err(format!("传输任务 {} 不存在或已完成", transfer_id))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            test_sftp_connection,
            connect_sftp,
            list_directory,
            download_file,
            download_file_with_progress,
            upload_file,
            upload_file_data,
            create_directory,
            delete_file,
            get_connection_info,
            get_downloads_directory,
            disconnect_sftp,
            open_file_folder,
            cancel_transfer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
