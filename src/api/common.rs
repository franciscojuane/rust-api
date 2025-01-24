#![allow(dead_code)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page_number: usize,
    pub page_size: usize,
    pub total_items: usize,
    pub total_pages: usize,
}