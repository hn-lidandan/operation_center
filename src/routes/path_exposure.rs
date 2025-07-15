use crate::routes::command::{cmd_localize, cmd_setup, cmd_unzip};
use crate::routes::instanll::{find_info_file, find_settings, index, save_settings};
use crate::routes::maintask::{get_maintasks,get_maintask_by_id,create_main_task};
use crate::routes::subtask::{get_subtask,get_subtasks_by_parentid,create_batch_subtask,update_subtask_info,create_failed_subtask};
use crate::routes::update::{bakeup};
use actix_web::{self, web};

pub fn path_config(service_config: &mut web::ServiceConfig) {
    let stu_scope = web::scope("api")
        //install
        .service(index)
        .service(find_info_file)
        .service(find_settings)
        .service(save_settings)
        //command
        .service(cmd_setup)
        .service(cmd_unzip)
        .service(cmd_localize)
        // subtask
        .service(get_subtask)
        .service(get_subtasks_by_parentid)
        .service(create_batch_subtask)
        .service(update_subtask_info)
        .service(create_failed_subtask)
        // maintask
        .service(get_maintasks)
        .service(get_maintask_by_id)
        .service(create_main_task)
        // update
        .service(bakeup);

    service_config.service(stu_scope);
}
