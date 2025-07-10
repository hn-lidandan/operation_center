use crate::config::settings::DatabaseConfig;
use crate::entity::main_task_messageinfo::{self};
use crate::entity::subtask_messageinfo::{self};
use crate::entity::subtask_messageinfo::{Model, SubTaskStatus};
use crate::models::task_model::{FailedTaskRequest, TaskBody, UpdateTaskRequest};
use crate::utils::id_generator::next_id;
use crate::utils::web_error::WebError;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::QueryOrder;
use sea_orm::{ColumnTrait, Set};
use sea_orm::{EntityTrait, QueryFilter};

/*查询任务 */
pub async fn get_subtask(id: i64) -> Result<Model, WebError> {
    let db = DatabaseConfig::get_connect().await?;
    subtask_messageinfo::Entity::find_by_id(id)
        .all(&db)
        .await
        .map_err(WebError::from)?
        .into_iter() // 将 Vec<Model> 转为迭代器
        .next() // 获取第一个元素（Option<Model>）
        .ok_or_else(|| WebError::NotFound("Task not found".into()))
}

/*根据parent_id查询全部任务 */
pub async fn get_subtasks(
    id: i64,
    // page_size: u64,
    // page: u64,
) -> Result<Vec<Model>, WebError> {
    let db = DatabaseConfig::get_connect().await?;

    let data = subtask_messageinfo::Entity::find()
        .filter(subtask_messageinfo::Column::ParentId.eq(id))
        .order_by_asc(subtask_messageinfo::Column::TaskOrder)
        .all(&db)
        .await
        .map_err(WebError::from)?;

    Ok(data)
}

/* 根据批量创建子任务*/
pub async fn create_batchtask(tasks: &Vec<TaskBody>) -> Result<Vec<Model>, WebError> {
    info!("进入create_batchtask");
    let db = DatabaseConfig::get_connect().await?;

    if tasks.is_empty() {
        return Err(WebError::DBError("Tasks vector cannot be empty".into()));
    }
    let first_task = tasks
        .first()
        .ok_or_else(|| WebError::DBError("Failed to get first task".into()))?;

    // 查询主任务
    let main_task = main_task_messageinfo::Entity::find_by_id(first_task.parent_id)
        .one(&db)
        .await
        .map_err(WebError::from)?
        .ok_or_else(|| {
            info!("主任务不存在 parent_id:{}", first_task.parent_id);
            WebError::NotFound("主任务不存在,无法更新子任务".into())
        })?;

    let mut results = Vec::new();
    info!("子任务集tasks为：{:?}", tasks);
    for task in tasks {
        info!("子任务task为：{:?}", task);
        let inserted_id = next_id();
        let new_subtask = subtask_messageinfo::ActiveModel {
            id: Set(inserted_id),
            sub_title: Set(task.name.clone()),
            description: Set(task.description.clone()),
            status: Set(SubTaskStatus::Inprogress),
            task_order: Set(Some(task.order)),
            task_type: Set(main_task.task_type.clone()),
            parent_id: Set(task.parent_id),
            create_timestamp: Set(chrono::Utc::now().into()),
            update_timestamp: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        let insert_result = new_subtask.insert(&db).await;

        match insert_result {
            Ok(_) => {
                let res = find_maintask(&db, inserted_id).await?;
                results.push(res);
            }
            Err(sea_orm::DbErr::RecordNotFound(_)) => {
                let res = find_maintask(&db, inserted_id).await?;
                results.push(res);
            }
            Err(e) => {
                // Handle any other database errors
                error!("批量插入子任务失败 (create_batchtask): {:?}", e);
                return Err(WebError::from(actix_web::error::ErrorBadRequest(e)));
            }
        }
    }
    Ok(results)
}

pub async fn update_subtask(taskrequest: UpdateTaskRequest) -> Result<Model, WebError> {
    let db = DatabaseConfig::get_connect().await?;
    //查询子任务是否存在
    let mut query = subtask_messageinfo::Entity::find()
        .filter(subtask_messageinfo::Column::ParentId.eq(taskrequest.parent_id))
        .filter(subtask_messageinfo::Column::TaskOrder.eq(taskrequest.order));
    let name = &taskrequest.name;
    query = query.filter(subtask_messageinfo::Column::SubTitle.eq(name.clone()));
    let existing = query.one(&db).await.map_err(WebError::from)?;
    //存在则更新数据信息
    if let Some(model) = existing {
        let mut active: subtask_messageinfo::ActiveModel = model.into();
        active.status = Set(taskrequest.status);
        active.log = Set(Some(taskrequest.log));
        active.update_timestamp = Set(Some(chrono::Utc::now()));
        let update = active.update(&db).await.map_err(WebError::from)?;
        Ok(update)
    } else {
        Err(WebError::NotFound("SubTask not found".to_string()))
    }
}

/**
 * 登记失败任务
 */
pub async fn create_failed_subtask(task: &FailedTaskRequest) -> Result<Model, WebError> {
    let db = DatabaseConfig::get_connect().await?;
    //查询主任务是否存在
    let result = main_task_messageinfo::Entity::find_by_id(task.id)
        .one(&db)
        .await
        .map_err(WebError::from)?;

    if let Some(model) = result {
        let log_value = format!(
            "log: {}
            (code: {}, msg: {})",
            task.log, task.overview.code, task.overview.msg
        );
        let inserted_id = next_id();
        let new_subtask = subtask_messageinfo::ActiveModel {
            id: Set(inserted_id),
            sub_title: Set(model.name.clone()),
            status: Set(SubTaskStatus::Failure),
            description: Set(Some(format!("{:?}", task.overview))),
            task_type: Set(model.task_type.clone()),
            parent_id: Set(model.id),
            log: Set(Some(log_value)),
            create_timestamp: Set(Some(chrono::Utc::now())),
            update_timestamp: Set(Some(chrono::Utc::now())),
            ..Default::default()
        };
        let insert_result = new_subtask.insert(&db).await;
        match insert_result {
            Ok(_) => {
                let res = find_maintask(&db, inserted_id).await?;
                Ok(res)
            }
            Err(sea_orm::DbErr::RecordNotFound(_)) => {
                let res = find_maintask(&db, inserted_id).await?;
                Ok(res)
            }
            Err(e) => {
                // Handle any other database errors
                error!("批量插入子任务失败 (create_batchtask): {:?}", e);
                Err(WebError::from(actix_web::error::ErrorBadRequest(e)))
            }
        }
    } else {
        Err(WebError::NotFound(
            "主任务不存在，无法登记失败子任务".to_string(),
        ))
    }
}

async fn find_maintask(db: &DatabaseConnection, inserted_id: i64) -> Result<Model, WebError> {
    let inserted_model = subtask_messageinfo::Entity::find_by_id(inserted_id)
        .one(db)
        .await?
        .ok_or_else(|| WebError::NotFound("未能找到刚刚插入的任务。".to_string()))?;

    Ok(inserted_model)
}
