use base64::Engine;
use once_cell::sync::Lazy;
use rand::Rng;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

mod tests;

// API 响应结构
#[derive(Debug, Deserialize)]
struct WorkflowRunsResponse {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    head_sha: String,
    artifacts_url: String,
}

#[derive(Debug, Deserialize)]
struct ArtifactsResponse {
    artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize)]
struct Artifact {
    name: String,
    archive_download_url: String,
}

// 全局静态变量存储 Lamina 路径
static LAMINA_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| {
    // 加载 .env 文件
    dotenv::dotenv().ok();

    // 1. 创建目标目录
    let out_dir = env::current_dir().unwrap().join("lamina");
    fs::create_dir_all(&out_dir).expect("创建 lamina 目录失败");

    // 2. 获取系统对应的可执行文件名
    let exe_path = if cfg!(target_os = "windows") {
        "build\\Release\\Lamina.exe"
    } else {
        "build/Lamina"
    };

    let lamina_exe = out_dir.join(exe_path);

    // 3. 如果文件已存在，直接返回
    if lamina_exe.exists() {
        return lamina_exe;
    }

    // 4. 获取最新的 workflow run
    let client = Client::new();
    let runs_url = "https://api.github.com/repos/Lamina-dev/Lamina/actions/workflows/build.yml/runs?branch=main&event=push&status=success&per_page=1";

    let runs_response = client
        .get(runs_url)
        .header("User-Agent", "lamina-test")
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .bearer_auth(env::var("GITHUB_TOKEN").expect("请设置环境变量 GITHUB_TOKEN"))
        .send()
        .expect("获取 GitHub Actions 工作流运行数据失败")
        .json::<WorkflowRunsResponse>()
        .expect("解析 GitHub Actions 工作流运行数据失败");

    let run = runs_response
        .workflow_runs
        .first()
        .expect("未找到成功的构建");

    // println!("使用的 Lamina 提交哈希：{}", run.head_sha);
    Command::new("powershell")
        .arg("-Command")
        .arg(format!("\"hash={}\" >> $env:GITHUB_OUTPUT", run.head_sha))
        .status()
        .expect("向 GitHub 输出中写入 hash 失败");

    // 5. 获取 artifacts
    let artifacts_response = client
        .get(&run.artifacts_url)
        .header("User-Agent", "lamina-test")
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .bearer_auth(env::var("GITHUB_TOKEN").expect("请设置环境变量 GITHUB_TOKEN"))
        .send()
        .expect("获取构建产物数据失败")
        .json::<ArtifactsResponse>()
        .expect("解析构建产物数据失败");

    // 6. 选择正确的 artifact
    let target_name = if cfg!(target_os = "windows") {
        "lamina-windows-2025-Release"
    } else {
        "lamina-ubuntu-latest-Release"
    };

    let artifact = artifacts_response
        .artifacts
        .iter()
        .find(|a| a.name == target_name)
        .expect(&format!("产物 {} 未找到", target_name));

    // 7. 下载 artifact
    // println!("下载产物：{}", artifact.name);
    Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "\"download={}\" >> $env:GITHUB_OUTPUT",
            base64::engine::general_purpose::STANDARD.encode(format!(
                "[{}]({})",
                artifact.name, artifact.archive_download_url
            ))
        ))
        .status()
        .expect("向 GitHub 输出中写入 download 失败");

    let download_url = &artifact.archive_download_url;
    let mut response = client
        .get(download_url)
        .header("User-Agent", "lamina-test")
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .bearer_auth(env::var("GITHUB_TOKEN").expect("请设置环境变量 GITHUB_TOKEN"))
        .send()
        .expect("下载产物失败");

    let zip_path = out_dir.join(format!("{}.zip", artifact.name));
    let mut file = fs::File::create(&zip_path).expect("创建 Zip 文件失败");
    std::io::copy(&mut response, &mut file).expect("写入 Zip 文件失败");

    // 8. 解压 ZIP 文件
    let zip_file = fs::File::open(&zip_path).expect("打开 Zip 文件失败");
    let mut archive = zip::ZipArchive::new(zip_file).expect("读取 Zip 文件失败");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("读取 Zip 中的文件失败");
        let out_path = out_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path).expect(&format!("创建文件夹失败：{:?}", out_path));
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).expect(&format!("创建父文件夹失败：{:?}", parent));
            }
            let mut out_file =
                fs::File::create(&out_path).expect(&format!("创建文件失败：{:?}", out_path));
            std::io::copy(&mut file, &mut out_file)
                .expect(&format!("写入文件失败：{:?}", out_path));
        }
    }

    // 9. 删除 ZIP 文件
    fs::remove_file(&zip_path).ok();

    // 10. 设置可执行权限（Unix 系统）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&lamina_exe, perms).expect("设置可执行权限失败");
    }

    // 11. 返回可执行文件路径
    lamina_exe
});

fn extract_script_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    // 如果输出行数不足，直接返回整个输出
    if lines.len() < 3 {
        return output.trim().to_string();
    }

    // 提取脚本实际输出（跳过第一行和最后两行）
    let script_output = &lines[1..lines.len() - 2];

    // 重新组合并去除首尾空白
    script_output.join("\r\n").trim().to_string()
}

fn run(script: String) -> String {
    let scripts_dir = env::current_dir().unwrap().join("lamina").join("scripts");
    fs::create_dir_all(&scripts_dir).expect("创建脚本文件夹失败");

    // 创建唯一的脚本文件名
    let script_name = format!(
        "test_{}.lm",
        rand::rng().random_range(1000000000000000i64..9999999999999999i64)
    );

    let script_path = scripts_dir.join(script_name);

    // 创建临时脚本文件
    let mut file = fs::File::create(&script_path).expect("创建测试脚本文件失败");
    writeln!(file, "{}", script).expect("写入脚本文件失败");

    // 执行 Lamina
    let output = std::process::Command::new(LAMINA_PATH.as_os_str())
        .arg(&script_path.as_os_str())
        .output()
        .expect("执行 Lamina 失败");

    // 清理临时文件
    fs::remove_file(&script_path).ok();

    if *&output.stderr.len() > 0 {
        String::from_utf8_lossy(&output.stderr).trim().to_string()
    } else {
        extract_script_output(&String::from_utf8_lossy(&output.stdout))
    }
}

pub fn s(script: String) -> String {
    run(format!("print({});", script))
}

pub fn m(script: String) -> String {
    run(script)
}

fn main() {
    println!("Lamina 可执行文件路径：{:?}", LAMINA_PATH.as_os_str());
}
