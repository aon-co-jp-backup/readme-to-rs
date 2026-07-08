use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("使い方: readme-to-rs <README.mdまたはREADMEフォルダ> <出力先ディレクトリ>");
        return ExitCode::FAILURE;
    }

    let input = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    match readme_to_rs::convert_path(&input, &output_dir) {
        Ok(written) => {
            for path in &written {
                println!("生成しました: {}", path.display());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("変換に失敗しました: {e}");
            ExitCode::FAILURE
        }
    }
}
