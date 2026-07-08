//! readme-to-rs: READMEの内容を、Rustのドキュメントコメント付きモジュール
//! (`.rs`)へ変換するツール。
//!
//! 変換後の`.rs`ファイルは本体を持たず、README本文をそのまま`//!`
//! (モジュールレベルドキュメントコメント)として埋め込むだけの、
//! `cargo doc`や通常のエディタでそのまま読める形式になる。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Markdown文字列を、`//!`ドキュメントコメントで全行を包んだRustソースへ変換する。
pub fn convert_to_rs(markdown: &str) -> String {
    let mut out = String::new();
    for line in markdown.lines() {
        if line.is_empty() {
            out.push_str("//!\n");
        } else {
            out.push_str("//! ");
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// 元のファイル名(拡張子抜き、例: "README-Japan")を、有効なRustファイル名
/// (例: "readme_japan")へ変換する。
pub fn sanitize_module_filename(original_stem: &str) -> String {
    let mut result = String::new();
    let mut last_was_underscore = false;
    for c in original_stem.chars() {
        let lower = c.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            result.push(lower);
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }
    let trimmed = result.trim_matches('_');
    if trimmed.is_empty() {
        "readme".to_string()
    } else if trimmed.chars().next().unwrap().is_ascii_digit() {
        format!("m_{trimmed}")
    } else {
        trimmed.to_string()
    }
}

/// `input`が単一ファイルならそのファイルを、ディレクトリなら直下の
/// `*.md`/`*.markdown`ファイルすべてを`.rs`へ変換し、`output_dir`へ書き出す。
/// 書き出した`.rs`ファイルのパス一覧を返す。
pub fn convert_path(input: &Path, output_dir: &Path) -> io::Result<Vec<PathBuf>> {
    fs::create_dir_all(output_dir)?;

    let inputs: Vec<PathBuf> = if input.is_dir() {
        let mut files: Vec<PathBuf> = fs::read_dir(input)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
                        .unwrap_or(false)
            })
            .collect();
        files.sort();
        files
    } else {
        vec![input.to_path_buf()]
    };

    let mut written = Vec::with_capacity(inputs.len());
    for md_path in inputs {
        let markdown = fs::read_to_string(&md_path)?;
        let stem = md_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("readme");
        let rs_name = format!("{}.rs", sanitize_module_filename(stem));
        let out_path = output_dir.join(rs_name);
        fs::write(&out_path, convert_to_rs(&markdown))?;
        written.push(out_path);
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_plain_markdown_lines_to_doc_comments() {
        let md = "# Title\n\nSome text.\n";
        let rs = convert_to_rs(md);
        assert_eq!(rs, "//! # Title\n//!\n//! Some text.\n");
    }

    #[test]
    fn empty_lines_become_bare_doc_comment_markers() {
        let md = "a\n\nb";
        let rs = convert_to_rs(md);
        assert_eq!(rs, "//! a\n//!\n//! b\n");
    }

    #[test]
    fn sanitizes_hyphens_and_spaces_to_underscores() {
        assert_eq!(sanitize_module_filename("README-Japan"), "readme_japan");
        assert_eq!(sanitize_module_filename("README-UK-English"), "readme_uk_english");
    }

    #[test]
    fn sanitizes_parentheses_and_collapses_repeats() {
        assert_eq!(sanitize_module_filename("README-Iran(Persian)"), "readme_iran_persian");
    }

    #[test]
    fn prefixes_names_starting_with_a_digit() {
        assert_eq!(sanitize_module_filename("3rd-readme"), "m_3rd_readme");
    }

    #[test]
    fn empty_stem_falls_back_to_readme() {
        assert_eq!(sanitize_module_filename("---"), "readme");
    }

    #[test]
    fn convert_path_handles_a_single_file() {
        let dir = std::env::temp_dir().join(format!("readme_to_rs_test_single_{}", std::process::id()));
        let out_dir = dir.join("out");
        fs::create_dir_all(&dir).unwrap();
        let md_path = dir.join("README-Japan.md");
        fs::write(&md_path, "# こんにちは\n").unwrap();

        let written = convert_path(&md_path, &out_dir).unwrap();
        assert_eq!(written.len(), 1);
        assert_eq!(written[0].file_name().unwrap(), "readme_japan.rs");
        let content = fs::read_to_string(&written[0]).unwrap();
        assert_eq!(content, "//! # こんにちは\n");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn convert_path_handles_a_directory_of_markdown_files() {
        let dir = std::env::temp_dir().join(format!("readme_to_rs_test_dir_{}", std::process::id()));
        let out_dir = dir.join("out");
        let in_dir = dir.join("README");
        fs::create_dir_all(&in_dir).unwrap();
        fs::write(in_dir.join("README-Japan.md"), "# 日本語\n").unwrap();
        fs::write(in_dir.join("README-US-English.md"), "# English\n").unwrap();
        fs::write(in_dir.join("not-markdown.txt"), "ignored").unwrap();

        let mut written = convert_path(&in_dir, &out_dir).unwrap();
        written.sort();
        let names: Vec<_> = written
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        assert_eq!(names, vec!["readme_japan.rs", "readme_us_english.rs"]);

        fs::remove_dir_all(&dir).ok();
    }
}
