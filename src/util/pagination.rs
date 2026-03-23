use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// 全局通用的分页请求参数
#[derive(Debug, Deserialize, IntoParams)]
pub struct PageParams {
    /// 当前页码 (默认 1)
    #[serde(default = "default_page")]
    pub page: i64,

    /// 每页数量 (默认 10, 最大 100)
    #[serde(default = "default_size")]
    pub size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_size() -> i64 {
    10
}

impl PageParams {
    /// 获取 SQL OFFSET 的偏移量
    pub fn offset(&self) -> i64 {
        (self.page.max(1) - 1) * self.size.min(100)
    }

    /// 获取 SQL LIMIT 的限制量
    pub fn limit(&self) -> i64 {
        self.size.min(100).max(1)
    }
}

/// 全局通用的分页响应包装体
#[derive(Debug, Serialize, ToSchema)]
pub struct PagedRes<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总记录数
    pub total: i64,
    /// 当前页码
    pub page: i64,
    /// 每页大小
    pub size: i64,
    /// 是否有下一页
    pub has_next: bool,
}

impl<T> PagedRes<T> {
    pub fn new(items: Vec<T>, total: i64, page: i64, size: i64) -> Self {
        Self {
            items,
            total,
            page,
            size,
            has_next: (page * size) < total,
        }
    }
}
