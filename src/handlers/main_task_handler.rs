use crate::config::settings::DatabaseConfig;
use crate::entity::main_task_messageinfo::Model;
use crate::entity::main_task_messageinfo::{self, MainTaskStatus};
use crate::models::task_model::{CreateMainTaskRequest, PagedResult};
// use crate::utils::id_generator::next_id;
use crate::utils::web_error::WebError;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
// use sea_orm::DatabaseConnection;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

/*查询任务 */
pub async fn get_main_task(id: i64) -> Result<Model, WebError> {
    info!("get_main_task");
    let db = DatabaseConfig::get_connect().await?;
    main_task_messageinfo::Entity::find_by_id(id)
        .all(&db)
        .await
        .map_err(WebError::from)?
        .into_iter() // 将 Vec<Model> 转为迭代器
        .next() // 获取第一个元素（Option<Model>）
        .ok_or_else(|| WebError::NotFound("Task not found".into()))
}

/*查询全部主任务 */
pub async fn get_main_tasks(
    // status: Option<String>,
    page_size: u64,
    page: u64,
) -> Result<PagedResult<Model>, WebError> {
    info!("get_main_tasks");
    let db = DatabaseConfig::get_connect().await?;
    let mut query = main_task_messageinfo::Entity::find();
    // if let Some(status_val) = status {
    //     query = query.filter(main_task_messageinfo::Column::Status.eq(status_val));
    // }
    // 按 create_timestamp 降序排序
    query = query.order_by_desc(main_task_messageinfo::Column::CreateTimestamp);
    // query.paginate(&db, page_size).fetch_page(page).await
    let paginator = query.paginate(&db, page_size);
    let total = paginator.num_items().await.map_err(WebError::from)?;
    let data = paginator.fetch_page(page).await.map_err(WebError::from)?;
    Ok(PagedResult { total, data })
}

// /* 根据Task 创建子任务*/
// pub async fn create_maintask(task: &MqttRequest) -> Result<Model, WebError> {
//     info!("进入create_maintask");
//     let inserted_id = next_id();
//     let db = DatabaseConfig::get_connect().await?;
//     let new_subtask = main_task_messageinfo::ActiveModel {
//         id: Set(inserted_id),
//         name: Set(task.maintask_name.to_string()),
//         description: Set(Some(task.description.to_string())),
//         status: Set(MainTaskStatus::Pending),
//         priority: Set(Some(1)),
//         worker_name: Set(task.worker_name.to_string()),
//         task_type: Set(task.task_type.to_string()),
//         create_timestamp: Set(chrono::Utc::now().into()),
//         update_timestamp: Set(chrono::Utc::now().into()),
//         ..Default::default()
//     };

//     let insert_result = new_subtask.insert(&db).await;
//     match insert_result {
//         Ok(res) => Ok(res),
//         Err(sea_orm::DbErr::RecordNotFound(_)) => {
//             let res = find_maintask(&db, inserted_id).await?;
//             Ok(res)
//         }
//         Err(e) => {
//             // Handle any other database errors
//             error!("插入主任务失败 (create_maintask): {:?}", e);
//             Err(WebError::from(actix_web::error::ErrorBadRequest(e)))
//         }
//     }
// }

/* 根据Task 创建子任务*/
pub async fn create_maintask_rm(request: &CreateMainTaskRequest) -> Result<(), WebError> {
    info!("进入create_maintask_rm");
    let db = DatabaseConfig::get_connect().await?;
    let new_task = main_task_messageinfo::ActiveModel {
        id: Set(request.id),
        name: Set(request.maintask_name.to_string()),
        description: Set(request.description.clone()),
        status: Set(MainTaskStatus::Pending),
        priority: Set(Some(1)),
        worker_name: Set(request.worker_name.to_string()),
        task_type: Set(request.task_type.to_string()),
        create_timestamp: Set(chrono::Utc::now().into()),
        update_timestamp: Set(chrono::Utc::now().into()),
        ..Default::default()
    };
    let insert_result = new_task.insert(&db).await;
    match insert_result {
        Ok(_) => Ok(()),
        Err(sea_orm::DbErr::RecordNotFound(_)) => {
            info!("插入主任务成功 (create_maintask_rm): SQLite 不返回插入的模型，数据已成功插入。");
            Ok(())
        }
        Err(e) => {
            // Handle any other database errors
            eprintln!("插入主任务失败 (create_maintask_rm): {:?}", e);
            Err(WebError::from(actix_web::error::ErrorBadRequest(e)))
        }
    }
}

// async fn find_maintask(db: &DatabaseConnection, inserted_id: i64) -> Result<Model, WebError> {
//     let inserted_model = main_task_messageinfo::Entity::find_by_id(inserted_id)
//         .one(db)
//         .await?
//         .ok_or_else(|| WebError::NotFound("未能找到刚刚插入的任务。".to_string()))?;

//     Ok(inserted_model)
// }
