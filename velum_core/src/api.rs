use crate::piece_tree::PieceTree;
use once_cell::sync::Lazy;
use std::sync::RwLock;

static DOCUMENT: Lazy<RwLock<PieceTree>> = Lazy::new(|| RwLock::new(PieceTree::empty()));

pub fn hello_velum() -> String {
    "Hello from Velum Core (Rust)!".to_string()
}

pub fn get_sample_document() -> String {
    let mut pt = DOCUMENT.write().unwrap();
    *pt = PieceTree::new("Welcome to Velum.".to_string());
    pt.insert(16, " This is Microsoft Word 1:1 replica project.".to_string());
    pt.get_text()
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 创建空文档
pub fn create_empty_document() -> String {
    let mut pt = DOCUMENT.write().unwrap();
    *pt = PieceTree::empty();
    pt.get_text()
}

// 在指定位置插入文本
pub fn insert_text(offset: usize, new_text: String) -> String {
    let mut pt = DOCUMENT.write().unwrap();
    pt.insert(offset, new_text);
    pt.get_text()
}

// 删除指定范围文本
pub fn delete_text(offset: usize, length: usize) -> String {
    let mut pt = DOCUMENT.write().unwrap();
    pt.delete(offset, length);
    pt.get_text()
}

// 获取文本范围
pub fn get_text_range(offset: usize, length: usize) -> String {
    let pt = DOCUMENT.read().unwrap();
    pt.get_text_range(offset, length)
}

// 获取行数统计
pub fn get_line_count() -> usize {
    let pt = DOCUMENT.read().unwrap();
    pt.get_line_count()
}

// 获取指定行内容
pub fn get_line_content(line_number: usize) -> Option<String> {
    let pt = DOCUMENT.read().unwrap();
    pt.get_line(line_number)
}

// 获取完整文本
pub fn get_full_text() -> String {
    let pt = DOCUMENT.read().unwrap();
    pt.get_text()
}

// 撤销
pub fn undo() -> String {
    let mut pt = DOCUMENT.write().unwrap();
    pt.undo();
    pt.get_text()
}

// 重做
pub fn redo() -> String {
    let mut pt = DOCUMENT.write().unwrap();
    pt.redo();
    pt.get_text()
}
