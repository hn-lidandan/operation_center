use core::str;

use crate::entity::subtask_messageinfo::{SubTaskStatus, SubTaskStatusDef};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct TaskCreateRequest {
    #[validate(length(min = 1, message = "任务名称不能为空"))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    #[validate(length(min = 1, message = "任务状态不能为空"))]
    pub status: String,
    #[validate(range(min = 1, max = 5))]
    pub priority: i32,
    #[validate(length(min = 1, message = "任务类型不能为空"))]
    pub task_type: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TaskGetRequest {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct SubTasksRequest {
    #[validate(range(min = 1, message = "parent_id 不能为空"))]
    pub id: i64,
    // #[validate(range(min = 0))]
    // pub page: Option<u64>,
    // #[validate(range(min = 1))]
    // pub pagesize: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct MainTasksRequest {
    // pub status: Option<String>,
    #[validate(range(min = 0))]
    pub page: Option<u64>,
    #[validate(range(min = 1))]
    pub pagesize: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct TaskUpdateRequest {
    #[validate(range(min = 1, message = "任务id不能为空"))]
    pub id: i64,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    #[validate(length(min = 1, message = "任务状态不能为空"))]
    pub status: String,
    pub priority: Option<i32>,
    pub task_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Body {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub priority: i32,
    pub task_type: String,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TaskResponse {
    pub status: i32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SubTaskBody {
    pub id: i64,
    pub name: String,
    // pub log: Option<String>,
    pub status: String,
    pub task_order: i32,
    // pub parent_id: Option<i64>,
    pub task_type: String,
    pub duration: Option<f64>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SubTaskResponse {
    pub status: i32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<SubTaskBody>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SubTasksResponse {
    pub status: i32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<SubTaskBody>>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BatchTaskRequest {
    pub tasks: Vec<TaskBody>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TaskBody {
    pub parent_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct FailedTaskRequest {
    pub id: i64,
    pub log: String,
    pub status: String,
    pub overview: Detail,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Detail {
    pub code: u64,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTaskRequest {
    pub parent_id: i64,
    pub name: String,
    pub log: String,
    #[serde(with = "SubTaskStatusDef")]
    pub status: SubTaskStatus,
    pub order: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ServiceRequest {
    #[validate(range(min = 0))]
    pub page: Option<u64>,
    #[validate(range(min = 1))]
    pub pagesize: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct WebResponse<T> {
    pub status: i32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
    pub total: Option<u64>,
}

// impl<T> WebResponse<T> {
//     pub fn new(
//         status: i32,
//         description: impl Into<String>,
//         body: Option<T>,
//         total: Option<u64>,
//     ) -> Self {
//         Self {
//             status,
//             description: description.into(),
//             body,
//             total,
//         }
//     }

//     // pub fn success(description: impl Into<String>, body: T, total: Option<u64>) -> Self {
//     //     Self::new(200, description, Some(body), total)
//     // }

//     pub fn error(status: i32, description: impl Into<String>) -> Self {
//         Self::new(status, description, None, None)
//     }
// }

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct PagedResult<T> {
    pub total: u64,
    pub data: Vec<T>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateMainTaskRequest {
    pub maintask_name: String,
    pub worker_name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct FindMainTaskRequest {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AppResponse<T> {
    pub list: Option<T>,
    pub total_count: Option<u64>,
}

impl<T> AppResponse<T> {
    pub fn new(list: Option<T>, total_count: Option<u64>) -> Self {
        Self { list, total_count }
    }
    pub fn success(list: T, total_count: Option<u64>) -> Self {
        Self::new(Some(list), total_count)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LogResult {
    pub log: String,
}
