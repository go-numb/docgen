use clap::Parser;
use clap::{arg, command};
use ignore::WalkBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde_json::json;

mod req;
use crate::req::gemini::{get_content, request};

fn scan_for_files(dir: &Path) -> Vec<String> {
    let mut str = Vec::new();
    let walker = WalkBuilder::new(dir)
        .follow_links(true) // シンボリックリンクも追跡
        .standard_filters(true) // デフォルトフィルター (デフォルトで .gitignore 等を適用)
        .build();

    for entry in walker
    // エラーハンドリング
    {
        // ここでファイルのフィルタリングを行い、処理するファイルを選定する
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if is_document_or_source_code(path) {
                    println!("Found file: {}", path.display());
                    // ドキュメントかどうか判定し、処理対象にするか決める
                    // read file to string
                    let content = fs::read_to_string(path).expect("Failed to read file");
                    let filepath = path.to_str().unwrap();
                    str.push(format!(
                        "{}\n
                    ```{}
                    {}
                    ```
                    \n\n------------\n\n",
                        filepath,
                        path.extension().unwrap().to_str().unwrap(),
                        content
                    ));
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    str
}

/// ignore漏れを防ぐため、ファイルの拡張子で判定
fn is_document_or_source_code(file_path: &Path) -> bool {
    matches!(
        file_path.extension().and_then(|s| s.to_str()),
        Some("md")
            | Some("txt")
            | Some("rs")
            | Some("py")
            | Some("js")
            | Some("ts")
            | Some("jsx")
            | Some("tsx")
            | Some("html")
            | Some("css")
            | Some("c")
            | Some("cpp")
            | Some("java")
            | Some("php")
            | Some("go")
            | Some("rb")
            | Some("sh")
            | Some("bat")
            | Some("ps1")
            | Some("psm1")
            | Some("json")
            | Some("yaml")
            | Some("yml")
            | Some("toml")
            | Some("xml")
            | Some("csv")
            | Some("tsv")
            | Some("sql")
            | Some("sqlite")
            | Some("db")
            | Some("log")
            | Some("ini")
            | Some("cfg")
            | Some("conf")
            | Some("properties")
            | Some("env")
            | Some("envrc")
            | Some("env.sample")
            | Some("env.example")
            | Some("env.local")
            | Some("env.development")
            | Some("env.test")
            | Some("env.staging")
            | Some("env.production")
            | Some("env.ci")
            | Some("env.dev")
            | Some("env.prod")
            | Some("env.stage")
            | Some("env.stg")
    )
}

// ignore

// command line arguments
#[derive(Parser)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long, default_value = ".")]
    dir: String,
    #[arg(short, long, default_value = "gemini-1.5-flash")]
    model: String,
    #[arg(
        short,
        long,
        default_value = "与えられたディレクトリのファイル群を基に、以下の要素を丁寧に解説した技術ドキュメントを作成してください。一般項目は前項に、技術項目は後項に配置するようにしてください。特に、新規参加者や既存ユーザが容易に理解し、プロジェクトにすぐに貢献できるような内容を重視してください。

### ドキュメントに含める項目：

1. **プロジェクト概要**
    - プロジェクトの目的や用途、具体的な課題解決方法を簡潔に。

2. **まず利用するために as Getting Started**
    - プロジェクトのセットアップ方法や基本的な使い方を解説。特に、非技術者がが最初にプロジェクトまたはサービスを利用する際に必要な手順を説明してください。

3. **ファイル構造図とディレクトリ説明**
    - 上層部から具体的な構成までのビジュアル化（例：ツリービュー）に加え、各ディレクトリやファイルの役割を詳しく解説。特に関連性や階層構造がどのように意図されているかを説明してください。

4. **各ファイル・モジュールの詳細**
    - 各ファイルやモジュールについて以下の要素をできるだけ網羅的に記述：
        - **目的**: そのファイルがどのような役割を果たすのか。
        - **主要な関数やクラス**: 関数やクラスごとに、引数や返り値、使用例、エラー処理なども含めて詳細に解説。
        - **依存関係**: 他のファイルやライブラリとの関係性、インポート部分の重要なポイント。

5. **設計アルゴリズムやパターン**
    - 使用している設計パターンやアルゴリズムについて、必要に応じて具体的なコード例と共に解説（特に実務に役立つアドバイスを含める）。
    - なぜそれが選ばれたのか、他の選択肢と比較して説明する。

6. **環境構築・セットアップ手順**
    - 開発者が環境を構築する際によく直面する問題点やトラブルシューティングについても言及した手順説明：
        - 例: 必要な依存関係の管理、適切なバージョンの指定、APIキーや他の環境変数の設定方法。
        - 発生しやすいエラーや不具合、その解決策。

7. **ベストプラクティスと拡張方法**
    - プロジェクト拡張時・変更時に知っておくべきプラクティス。また、今後の開発やメンテナンスのために推奨される方針を提案する。

### 新規参加者や利用者向けの追記：
- コード解析の結果に基づき、開発者が直観的に理解できるよう、特に注意すべき依存ライブラリや動作上の特殊な要件、推奨される使用ケースについても明記してください。また、「このライブラリやモジュールは特にどのタイミングで使用するべきか」という点も解説すること。

### ドキュメント構成のスタイル：
- 各セクションは簡潔に、かつ必要最低限の内容だけを削ぎ落とさないよう、読む人がスムーズに理解できるレベルを維持すること。
- 技術的な項目は、実際のコードスニペットや例を交えながら具体的に解説し、関連性を持たせること。"
    )]
    prompt: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let dir = Path::new(&args.dir);
    let contents = scan_for_files(dir);

    // content length
    let content_length = contents.len();
    let content_size = contents.iter().map(|s| s.len()).sum::<usize>();
    let content_size_mb = content_size as f64 / 1024.0 / 1024.0;
    println!(
        "Found {} files, content size: {} MB",
        content_length, content_size_mb
    );

    // confirm to console, yes or no
    let mut input = String::new();
    println!("Do you want to send these files to the server? [y/N]");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input != "y" {
        println!("Canceled.");
        return;
    }

    // send to server
    let model = args.model.as_str();
    let prompt = args.prompt.as_str();
    let contents_str = contents
        .iter()
        .fold(String::new(), |acc, s| acc + format!("{}\n", s).as_str());
    let body = json!({
    "contents":
        json!([{
            // roleがuserの場合はuser、それ以外はmodel as assistant
            "role": "user",
            "parts": [
                {
                    "text": format!("{}\n{}\ncontent length: {}, content size: {:.2}MB",prompt, contents_str, content_length, content_size_mb),
                },
            ],
        }])
    });

    println!("{}", body);

    println!("Sending to server...");
    let result = request(model, body).await;

    match result {
        Ok(json) => {
            let content = get_content(&json).unwrap();
            println!("Response: {}", content);

            // ファイルに保存
            let time_at = chrono::Local::now();
            let docname = format!("doc-{}.md", time_at.format("%Y%m%d"));
            let path = Path::new(&docname);
            let mut file = fs::File::create(path).expect("Failed to create file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to file");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
