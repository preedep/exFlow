use actix_web::{HttpResponse, Responder, web};
use log::debug;
use tracing_attributes::instrument;

use crate::mod_service_api::entities::ExFlowRuntimeRegisterRequest;

#[instrument]
pub async fn post_register_runtime(
    request: web::Json<ExFlowRuntimeRegisterRequest>,
) -> impl Responder {
    debug!("post_register_runtime : {:#?}", request);



    HttpResponse::Ok().finish()
}
