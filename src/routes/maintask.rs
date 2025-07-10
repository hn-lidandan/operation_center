use crate::handlers::main_task_handler;
use crate::models::task_model::{
    AppResponse, CreateMainTaskRequest, FindMainTaskRequest, MainTasksRequest, SubTaskBody,
    WebResponse,
};
use crate::utils::web_error::{self, WebError};
use actix_web::web;
use actix_web::{HttpResponse, get, post};

/**
 *  获取主任务下所有子任务(status选输)
 */
#[utoipa::path(
    get,
    path = "/api/task/maintasks",
    request_body = MainTasksRequest,
    responses(
        (status = 200, description = "Task found", body = AppResponse<Vec<SubTaskBody>>),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error",body=String)
    ),
    tag = "main_task"
)]
#[get("/maintasks")]
async fn get_maintasks(
    params: web::Query<MainTasksRequest>,
) -> Result<HttpResponse, web_error::WebError> {
    let request = params.into_inner();
    let page = request.page.unwrap_or(0);
    let page_size = request.pagesize.unwrap_or(10);
    let task = main_task_handler::get_main_tasks(page_size, page).await;
    //构建返回结果
    match task {
        Ok(vec) => {
            let bodys: Vec<SubTaskBody> = vec
                .data
                .into_iter()
                .map(|model| SubTaskBody {
                    id: model.id,
                    name: model.name,
                    // log: Some(model.log.unwrap_or("".to_string())),
                    status: model.status.to_string(),
                    task_type: model.task_type,
                    task_order: model.priority.unwrap_or(0),
                    duration: None,
                })
                .collect();

            Ok(
                HttpResponse::Ok().json(AppResponse::<Vec<SubTaskBody>>::success(
                    bodys,
                    Some(vec.total),
                )),
            )
        }
        Err(e) => {
            info!("用户任务查询失败!!{}", e);
            Err(WebError::ActixError(e.to_string()))
        }
    }
}
/**
 *  创建主任务
 */
#[utoipa::path(
    post,
    path = "/api/task/maintasks",
    request_body = CreateMainTaskRequest,
    responses(
        (status = 200, description = "主任务创建成功 ", body = String),
        (status = 404, description = "主任务创建失败"),
        (status = 500, description = "Internal server error",body=String)
    ),
    tag = "main_task"
)]
#[post("/maintasks")]
async fn create_main_task(
    params: web::Json<CreateMainTaskRequest>,
) -> Result<HttpResponse, web_error::WebError> {
    let request = params.into_inner();
    match main_task_handler::create_maintask_rm(&request).await {
        Ok(_) => Ok(HttpResponse::Ok().json("新增主任务成功")),
        Err(e) => {
            info!("任务创建失败:{}", e.to_string());
            Err(WebError::ActixError(e.to_string()))
        }
    }
}

/**
 * 根据ID查询主任务
 */
#[utoipa::path(
    get,
    path = "/api/task/maintasks/{id}",
    request_body = FindMainTaskRequest,
    responses(
        (status = 200, description = "Task found", body = SubTaskBody),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error",body=WebResponse<String>)
    ),
    tag = "main_task"
)]
#[get("/maintasks/{id}")]
async fn get_maintask_by_id(params: web::Path<i64>) -> Result<HttpResponse, web_error::WebError> {
    let id = params.into_inner();
    let task = main_task_handler::get_main_task(id).await;
    //构建返回结果
    match task {
        Ok(vec) => {
            let re = SubTaskBody {
                id: vec.id,
                name: vec.name,
                // log: Some(vec.log.unwrap_or("".to_string())),
                status: vec.status.to_string(),
                task_order: vec.priority.unwrap_or(0),
                task_type: vec.task_type,
                duration: None,
            };
            Ok(HttpResponse::Ok().json(re))
        }
        Err(e) => {
            info!("用户任务查询失败!!{}", e);
            Err(WebError::ActixError(e.to_string()))
        }
    }
}
