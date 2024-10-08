use quick_xml::events::Event;
use quick_xml::Reader;
use std::{fs::File, io::BufReader, path::Path};
use zip::read::ZipArchive;

use std::io::Read;

pub fn read(path: &Path) -> Result<String, String> {
    // .docxファイルを開く
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // ZIPファイルとして開く
    let mut zip = ZipArchive::new(reader).unwrap();

    // Word文書ファイル(docx)は主に "word/document.xml" にテキストデータがあります
    let mut xml_file = zip.by_name("word/document.xml").unwrap();

    // 内容をバッファに読み込む
    let mut xml_content = String::new();
    xml_file.read_to_string(&mut xml_content).unwrap();

    // Quick-XMLでXMLを解析
    let mut reader = Reader::from_str(&xml_content);
    reader.trim_text(true);

    // イベントベースで解析
    let mut results = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"w:t" {
                    // <w:t>タグはテキストの部分
                    if let Ok(text) = reader.read_text(b"w:t", &mut Vec::new()) {
                        results.push(text);
                    }
                }
            }
            Ok(Event::Eof) => break, // ファイルの終わり
            Err(e) => {
                return Err(format!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ))
            }
            _ => (), // 他のイベントは無視
        }
        buf.clear();
    }

    // delete \u{3000}
    Ok(results.join("").replace("\u{3000}", " "))
}

// test read word
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_word() {
        let filename = "test.docx";
        let path = Path::new(filename);
        let content = read(path).unwrap();
        assert_eq!(content, "Hello, world!");
    }
}
