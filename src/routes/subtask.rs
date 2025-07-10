use crate::handlers::subtask_handler;
use crate::models::task_model::{
    AppResponse, BatchTaskRequest, FailedTaskRequest, LogResult, SubTaskBody, UpdateTaskRequest,
};
use crate::utils::web_error::{self, WebError};
use actix_web::web::{self};
use actix_web::{HttpResponse, get, post, web::Path};

/**
 * 根据id 获取子任务
 */
#[utoipa::path(
    get,
    path = "/api/task/subtasks/{id}",
    params(
        ("id" = i64, Path, description = "SubTask identifier")
    ),
    responses(
        (status = 200, description = "子任务获取成功", body = LogResult),
        (status = 404, description = "Task not found"),
        (status = 500, description = "子任务获取失败",body=String)
    ),
    tag = "sub_task"
)]
#[get("/subtasks/{id}")]
pub async fn get_subtask(params: Path<i64>) -> Result<HttpResponse, web_error::WebError> {
    let id = params.into_inner();
    let task = subtask_handler::get_subtask(id).await;
    //构建返回结果
    match task {
        Ok(res) => {
            let logres = LogResult {
                log: res.log.unwrap_or_else(|| "".to_string()),
            };
            Ok(HttpResponse::Ok().json(logres))
        }
        Err(e) => Err(WebError::ActixError(e.to_string())),
    }
}

/**
 * 根据parent_id 获取主任务下所有子任务
 */
#[utoipa::path(
    get,
    path = "/api/task/maintasks/{parent_id}/subtasks",
    params(
        ("parent_id" = i64, Path, description = "SubTask identifier")
    ),
    responses(
        (status = 200, description = "子任务获取失败", body = AppResponse<Vec<SubTaskBody>>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "子任务获取失败",body=String)
    ),
    tag = "sub_task"
)]
#[get("/maintasks/{parent_id}/subtasks")]
pub async fn get_subtasks_by_parentid(
    params: Path<i64>,
) -> Result<HttpResponse, web_error::WebError> {
    let id = params.into_inner();
    let task = subtask_handler::get_subtasks(id).await;
    //构建返回结果
    match task {
        Ok(vec) => {
            let bodys: Vec<SubTaskBody> = vec
                .into_iter()
                .map(|model| {
                    // 计算duration
                    let duration = if let (Some(update), Some(create)) =
                        (model.update_timestamp, model.create_timestamp)
                    {
                        info!("Debug - update_timestamp: {:?}", update);
                        info!("Debug - create_timestamp: {:?}", create);
                        let duration = (update - create).num_milliseconds() as f64 / 1000.0;
                        info!("Debug - calculated duration: {}", duration);
                        Some((duration * 10.0).round() / 10.0) // 保留一位小数
                    } else {
                        info!("Debug - One or both timestamps are None");
                        None
                    };

                    SubTaskBody {
                        id: model.id,
                        name: model.sub_title,
                        // log: Some(model.log.unwrap_or("".to_string())),
                        status: model.status.to_string(),
                        task_type: model.task_type,
                        task_order: model.task_order.unwrap_or(0),
                        duration,
                    }
                })
                .collect();
            let total = bodys.len() as u64;

            Ok(HttpResponse::Ok()
                .json(AppResponse::<Vec<SubTaskBody>>::success(bodys, Some(total))))
        }
        Err(e) => {
            info!("用户任务查询失败!!{}", e);
            // let response = WebResponse::<String>::error(500, e.to_string());
            Err(WebError::ActixError(e.to_string()))
        }
    }
}

/**
 * 批量创建子任务
 */
#[utoipa::path(
    post,
    path = "/api/task/subtasks:batchCreate",
    request_body = BatchTaskRequest,
    responses(
        (status = 200, description = "批量创建成功", body = String),
        (status = 404, description = "Task not found"),
        (status = 500, description = "批量创建失败",body=String)
    ),
    tag = "sub_task"
)]
#[post("/subtasks:batchCreate")]
pub async fn create_batch_subtask(
    params: web::Json<BatchTaskRequest>,
) -> Result<HttpResponse, web_error::WebError> {
    info! {"此时的params:{:?}",params};
    let request = params.into_inner().tasks;
    info! {"此时的params:{:?}",request};
    let result = subtask_handler::create_batchtask(&request).await;
    match result {
        Ok(_tasks) => Ok(HttpResponse::Ok().json("创建成功！".to_string())),
        Err(e) => {
            info!("用户任务查询失败!!{}", e);
            // let response = WebResponse::<String>::error(500, e.to_string());
            Err(WebError::ActixError(e.to_string()))
        }
    }
}

/**
 * 更新子任务信息
 */
#[utoipa::path(
    post,
    path = "/api/task/subtasks:update",
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "子任务修改成功", body = String),
        (status = 404, description = "Task not found"),
        (status = 500, description = "子任务修改失败",body=String)
    ),
    tag = "sub_task"
)]
#[post("/subtasks:update")]
pub async fn update_subtask_info(
    params: web::Json<UpdateTaskRequest>,
) -> Result<HttpResponse, web_error::WebError> {
    info!("此时update_subtask_info的入参 params:{:?}", params);
    let result = subtask_handler::update_subtask(params.into_inner()).await;

    match result {
        Ok(_task) => Ok(HttpResponse::Ok().json("更新成功".to_string())),
        Err(e) => {
            error!("子任务修改失败!!{}", e);
            Err(WebError::ActixError(e.to_string()))
        }
    }
}
/**
 * 失败任务登记（无法执行）
 */
#[utoipa::path(
    post,
    path = "/api/task/subtasks:singleCreate",
    request_body = FailedTaskRequest,
    responses(
        (status = 200, description = "失败任务登记成功", body = String),
        (status = 404, description = "Task not found"),
        (status = 500, description = "失败任务登记失败",body=String)
    ),
    tag = "sub_task"
)]
#[post("/subtasks:singleCreate")]
pub async fn create_failed_subtask(
    params: web::Json<FailedTaskRequest>,
) -> Result<HttpResponse, web_error::WebError> {
    let result = subtask_handler::create_failed_subtask(&params).await;
    match result {
        Ok(_task) => Ok(HttpResponse::Ok().json("失败登记完成".to_string())),
        Err(e) => {
            error!("失败任务登记出错!!{}", e);
            Err(WebError::ActixError(e.to_string()))
        }
    }
}
