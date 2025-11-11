use crate::core::authentication::enums::Access;
use crate::core::authentication::services::authenticate;
use crate::core::base::models::BaseResponse;
use crate::core::database::models::CreateResponse;
use crate::core::database::services::error_message;
use crate::trading::asset::models::{Asset, SaveAssetRequest};
use crate::trading::asset::repositories::insert_one;
use crate::user::log::models::UserLog;
use crate::user::log::repositories::find_one_by_authentication;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::Database;

pub async fn create(
    request: HttpRequest,
    database: web::Data<Database>,
    data: web::Json<SaveAssetRequest>,
) -> impl Responder {
    let mut response: CreateResponse = CreateResponse::default();

    let authentication: BaseResponse = authenticate(request, &Access::Private, &data.clone());

    if authentication.result {
        let log: Option<UserLog> =
            find_one_by_authentication(database.get_ref(), &data.authentication).await;

        if !log.is_none() {
            match insert_one(
                database.get_ref(),
                &Asset {
                    provider: data.provider.clone(),
                    status: data.clone().status,
                    ticker: data.clone().symbol,
                    ..Default::default()
                },
                Some(log.unwrap_or_default().user),
            )
            .await
            {
                Ok(_) => {
                    response.response = "Asset has been created successfully.".to_string();
                    response.result = true;
                }
                Err(error) => {
                    for key in vec!["name"] {
                        response.response = error_message(error.clone(), key);
                    }
                }
            };
        } else {
            response.response = "Abnormal request detected.".to_string();
        }
    } else {
        return HttpResponse::Unauthorized().json(authentication);
    }

    HttpResponse::Ok().json(response)
}
