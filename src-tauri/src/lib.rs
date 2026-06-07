use serde::{Deserialize, Serialize};
use jieba_rs::Jieba;

// ==================== 数据结构定义 ====================

/// 前端传来的请求
#[derive(Debug, Deserialize)]
pub struct NamingRequest {
    pub meaning: String,           // 含义，如 "用户最大登录次数"
    pub naming_types: Vec<String>, // 命名类型，如 ["variable", "function"]
}

/// 一种命名类型的所有风格结果
#[derive(Debug, Serialize)]
pub struct NamingResult {
    pub naming_type: String,       // "variable" / "function" / "project"
    pub results: Vec<StyleResult>, // 该类型下所有风格的命名
}

/// 单条命名结果
#[derive(Debug, Serialize)]
pub struct StyleResult {
    pub style: String,  // 风格名，如 "camelCase"
    pub value: String,  // 生成的命名，如 "maxLoginAttempts"
}

// ==================== 核心逻辑 ====================

/// 主入口：根据含义和命名类型，生成所有结果
pub fn generate_names(meaning: &str, naming_types: &[String]) -> Vec<NamingResult> {
    // 1. 分词：把中文含义拆成单词列表
    let words = tokenize(meaning);

    // 2. 术语映射：把中文词替换成英文词
    let en_words: Vec<String> = words.iter().map(|w| translate(w)).collect();

    // 3. 为每种命名类型生成不同风格的命名
    let mut all_results = Vec::new();

    for ntype in naming_types {
        let mut results = Vec::new();

        // 处理函数命名：加动词前缀
        let final_words = match ntype.as_str() {
            "function" => add_verb_prefix(&en_words),
            _ => en_words.clone(),
        };

        // 生成各种风格
        results.push(StyleResult {
            style: "camelCase".into(),
            value: to_camel_case(&final_words),
        });
        results.push(StyleResult {
            style: "PascalCase".into(),
            value: to_pascal_case(&final_words),
        });
        results.push(StyleResult {
            style: "snake_case".into(),
            value: to_snake_case(&final_words),
        });
        results.push(StyleResult {
            style: "UPPER_SNAKE".into(),
            value: to_upper_snake_case(&final_words),
        });
        results.push(StyleResult {
            style: "kebab-case".into(),
            value: to_kebab_case(&final_words),
        });

        all_results.push(NamingResult {
            naming_type: ntype.clone(),
            results,
        });
    }

    all_results
}

// ==================== 分词模块 ====================

/// 简单分词：对中文用 jieba 分词，对英文按空格分词
fn tokenize(text: &str) -> Vec<String> {
    // 检测是否包含中文
    let has_chinese = text.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}');

    if has_chinese {
        // 用 jieba 进行中文分词
        let jieba = Jieba::new();
        let words = jieba.cut(text, false);
        words.into_iter()
            .map(|w| w.to_string())
            .filter(|w| !w.trim().is_empty())
            .collect()
    } else {
        // 英文按空格拆分
        let separators = [' ', ',', '，', '、', '　', '\t', '\n'];
        text.split(&separators[..])
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

// ==================== 翻译/术语映射模块 ====================

/// 简易术语映射表（后续会扩展为完整词库）
/// 简易术语映射表（后续会扩展为完整词库）
fn translate(word: &str) -> String {
    match word {
        "用户" => "user".to_string(),
        "最大" => "max".to_string(),
        "最小" => "min".to_string(),
        "登录" => "login".to_string(),
        "次数" => "attempts".to_string(),
        "尝试" => "attempts".to_string(),
        "密码" => "password".to_string(),
        "错误" => "error".to_string(),
        "次数" => "count".to_string(),
        "超时" => "timeout".to_string(),
        "连接" => "connection".to_string(),
        "数据库" => "db".to_string(),
        "获取" => "get".to_string(),
        "设置" => "set".to_string(),
        "删除" => "delete".to_string(),
        "更新" => "update".to_string(),
        "创建" => "create".to_string(),
        "文件" => "file".to_string(),
        "上传" => "upload".to_string(),
        "下载" => "download".to_string(),
        "大小" => "size".to_string(),
        "限制" => "limit".to_string(),
        "名称" => "name".to_string(),
        "地址" => "address".to_string(),
        "状态" => "status".to_string(),
        "类型" => "type".to_string(),
        "标识" => "id".to_string(),
        "ID" => "id".to_string(),
        "Id" => "id".to_string(),
        "HTTP" => "http".to_string(),
        "URL" => "url".to_string(),
        "API" => "api".to_string(),
        // 英文直接返回小写
        _ => word.to_lowercase(),
    }
}
// ==================== 动词前缀模块 ====================

/// 为函数命名添加动词前缀
fn add_verb_prefix(words: &[String]) -> Vec<String> {
    if words.is_empty() {
        return vec!["get".to_string()];
    }

    // 检查第一个词是否已经是动词
    let first = &words[0].to_lowercase();
    let verbs = ["get", "set", "delete", "update", "create", "fetch", "remove", "add", "check", "handle"];

    if verbs.contains(&first.as_str()) {
        // 已经是动词，直接返回
        words.to_vec()
    } else {
        // 不是动词，在前面加上 "get"
        let mut result = vec!["get".to_string()];
        result.extend(words.iter().cloned());
        result
    }
}

// ==================== 命名风格转换模块 ====================

/// camelCase: 第一个单词小写，后续单词首字母大写
fn to_camel_case(words: &[String]) -> String {
    words.iter().enumerate().map(|(i, w)| {
        if i == 0 {
            w.to_lowercase()
        } else {
            capitalize(w)
        }
    }).collect()
}

/// PascalCase: 每个单词首字母大写
fn to_pascal_case(words: &[String]) -> String {
    words.iter().map(|w| capitalize(w)).collect()
}

/// snake_case: 全小写，下划线连接
fn to_snake_case(words: &[String]) -> String {
    words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("_")
}

/// UPPER_SNAKE: 全大写，下划线连接
fn to_upper_snake_case(words: &[String]) -> String {
    words.iter().map(|w| w.to_uppercase()).collect::<Vec<_>>().join("_")
}

/// kebab-case: 全小写，短横线连接
fn to_kebab_case(words: &[String]) -> String {
    words.iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("-")
}

/// 辅助函数：首字母大写
fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// ==================== Tauri 命令 ====================

/// 这是前端调用的入口函数
#[tauri::command]
fn name_it(request: NamingRequest) -> Vec<NamingResult> {
    generate_names(&request.meaning, &request.naming_types)
}

/// Tauri 应用启动入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![name_it])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}