use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use validator::Validate;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, serde::Deserialize, serde::Serialize, Validate,
)]
#[sea_orm(table_name = "main_task_messageinfo")]
pub struct Model {
    /// 主任务id
    #[sea_orm(primary_key)]
    pub id: i64,
    /// 任务名
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    /// 描述
    pub description: Option<String>,
    #[serde(with = "MainTaskStatusDef")]
    pub status: MainTaskStatus,
    pub log: Option<String>,
    /// 1-5  1为最高优先级，5为最低优先级
    #[validate(range(min = 1, max = 5))]
    pub priority: Option<i32>,
    /// 系统级   业务级
    #[serde(rename = "task_type")]
    pub task_type: String,
    /// worker名称
    pub worker_name: String,
    /// 创建时间
    pub create_timestamp: Option<DateTime<Utc>>,
    /// 更新时间
    pub update_timestamp: Option<DateTime<Utc>>,
    /// 备用字段1（大整数）
    pub bak1: Option<i64>, // 使用 Option 表示可为 NULL（根据实际业务调整）
    /// 备用字段2（字符串）
    pub bak2: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum MainTaskStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "in_progress")]
    Inprogress,
    #[sea_orm(string_value = "success")]
    Success,
    #[sea_orm(string_value = "failure")]
    Failure,
}

impl std::fmt::Display for MainTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MainTaskStatus::Pending => "pending",
            MainTaskStatus::Inprogress => "in_progress",
            MainTaskStatus::Success => "success",
            MainTaskStatus::Failure => "failure",
        };
        write!(f, "{}", s)
    }
}
#[warn(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "MainTaskStatus")]
pub enum MainTaskStatusDef {
    Pending,
    Inprogress,
    Success,
    Failure,
}

impl From<MainTaskStatusDef> for MainTaskStatus {
    fn from(def: MainTaskStatusDef) -> Self {
        match def {
            MainTaskStatusDef::Pending => MainTaskStatus::Pending,
            MainTaskStatusDef::Inprogress => MainTaskStatus::Inprogress,
            MainTaskStatusDef::Success => MainTaskStatus::Success,
            MainTaskStatusDef::Failure => MainTaskStatus::Failure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_main_task_model_validation() {
        let valid_task = Model {
            id: 1,
            name: "Valid Task".to_string(),
            description: Some("Description".to_string()),
            status: MainTaskStatus::Pending,
            log: None,
            priority: Some(3),
            task_type: "business".to_string(),
            worker_name: "worker1".to_string(),
            create_timestamp: Some(Utc::now()),
            update_timestamp: Some(Utc::now()),
            bak1: None,
            bak2: None,
        };
        assert!(valid_task.validate().is_ok());

        let invalid_name = Model {
            name: "".to_string(), // 名称过短
            ..valid_task.clone()
        };
        assert!(invalid_name.validate().is_err());

        let invalid_priority = Model {
            priority: Some(6), // 优先级超出范围
            ..valid_task
        };
        assert!(invalid_priority.validate().is_err());
    }

    #[test]
    fn test_main_task_status_conversion() {
        let def_status = MainTaskStatusDef::Inprogress;
        let status: MainTaskStatus = def_status.into();
        assert_eq!(status, MainTaskStatus::Inprogress);
        assert_eq!(status.to_string(), "in_progress");
    }
}
