use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, serde::Deserialize, serde::Serialize, Validate,
)]
#[sea_orm(table_name = "subtask_messageinfo")]
pub struct Model {
    /// 子任务id
    #[sea_orm(primary_key)]
    pub id: i64,
    /// 子任务名称
    #[validate(length(min = 1, max = 255))]
    pub sub_title: String,
    /// 描述
    pub description: Option<String>,
    /// 已创建  运行中  成功  失败
    #[serde(with = "SubTaskStatusDef")]
    pub status: SubTaskStatus,
    /// 运行日志
    pub log: Option<String>,
    /// 执行顺序
    #[validate(range(min = 1))]
    #[serde(rename = "task_order")]
    pub task_order: Option<i32>,
    /// 子任务类型
    #[serde(rename = "task_type")]
    pub task_type: String,
    /// 主任务id
    #[serde(rename = "parent_id")]
    pub parent_id: i64,
    /// 创建时间
    pub create_timestamp: Option<DateTime<Utc>>,
    /// 更新时间
    pub update_timestamp: Option<DateTime<Utc>>,
    /// 备用字段1（大整数）
    pub bak1: Option<i64>,
    /// 备用字段2（字符串）
    pub bak2: Option<String>,
    // 添加 #[sea_orm(ignore)] 让 Sea-ORM 忽略这个字段
    /// 耗时 -- 任务执行时长（秒），仅内存计算使用，不存储数据库
    #[sea_orm(ignore)]
    #[serde(skip)]
    pub duration: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, ToSchema)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum SubTaskStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "in_progress")]
    Inprogress,
    #[sea_orm(string_value = "success")]
    Success,
    #[sea_orm(string_value = "failure")]
    Failure,
}

impl std::fmt::Display for SubTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SubTaskStatus::Pending => "pending",
            SubTaskStatus::Inprogress => "in_progress",
            SubTaskStatus::Success => "success",
            SubTaskStatus::Failure => "failure",
        };
        write!(f, "{}", s)
    }
}
#[warn(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "SubTaskStatus")]
pub enum SubTaskStatusDef {
    Pending,
    Inprogress,
    Success,
    Failure,
}

impl From<SubTaskStatusDef> for SubTaskStatus {
    fn from(def: SubTaskStatusDef) -> Self {
        match def {
            SubTaskStatusDef::Pending => SubTaskStatus::Pending,
            SubTaskStatusDef::Inprogress => SubTaskStatus::Inprogress,
            SubTaskStatusDef::Success => SubTaskStatus::Success,
            SubTaskStatusDef::Failure => SubTaskStatus::Failure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_subtask_model_validation() {
        let valid_subtask = Model {
            id: 1,
            sub_title: "Sub Task".to_string(),
            description: Some("Description".to_string()),
            status: SubTaskStatus::Pending,
            log: None,
            task_order: Some(1),
            task_type: "processing".to_string(),
            parent_id: 100,
            create_timestamp: Some(Utc::now()),
            update_timestamp: Some(Utc::now()),
            bak1: None,
            bak2: None,
            duration: None, // 忽略字段
        };
        assert!(valid_subtask.validate().is_ok());

        let invalid_title = Model {
            sub_title: "".to_string(), // 标题过短
            ..valid_subtask.clone()
        };
        assert!(invalid_title.validate().is_err());

        let invalid_order = Model {
            task_order: Some(0), // 顺序值过小
            ..valid_subtask
        };
        assert!(invalid_order.validate().is_err());
    }

    #[test]
    fn test_subtask_status_conversion() {
        let def_status = SubTaskStatusDef::Success;
        let status: SubTaskStatus = def_status.into();
        assert_eq!(status, SubTaskStatus::Success);
        assert_eq!(status.to_string(), "success");
    }
}
