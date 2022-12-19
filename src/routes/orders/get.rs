use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use bigdecimal::BigDecimal;
use chrono::DateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    authorization::parse_jwt,
    response::{OrderDetailsProduct, OrderDetailsResponse, ServiceResponse},
    types::{OrderDetail, OrderOverview},
};

pub async fn get_all_orders(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, GetOrdersError> {
    let mut res = ServiceResponse::new(Vec::<OrderOverview>::new());

    let (user_id, _role) = parse_jwt(&req).map_err(GetOrdersError::JwtError)?;
    let order_overviews = sqlx::query!(r#"SELECT orders.order_id, order_date, orders.total_price, product_name, image_url FROM orders
		JOIN order_items ON order_items.order_id = orders.order_id
		JOIN products ON products.product_id = order_items.product_id
		WHERE user_id = $1"#, user_id).fetch_all(pool.get_ref()).await.map_err(GetOrdersError::SqlxError)?;

    let mut temp_arr = vec![];

    for order_overview in order_overviews {
        let new_o = OrderOverview {
            order_id: order_overview.order_id,
            order_date: order_overview.order_date,
            total_price: order_overview.total_price,
            product: order_overview.product_name,
            product_image_url: order_overview.image_url,
        };

        temp_arr.push(new_o);
    }

    res.data = temp_arr;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_order_details(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, GetOrdersError> {
    let mut res = ServiceResponse::new(OrderDetailsResponse::default());
    let order_id = path.into_inner();

    let (user_id, _role) = parse_jwt(&req).map_err(GetOrdersError::JwtError)?;
    let order_details = sqlx::query!(r#"SELECT order_date, order_items.product_id, product_name, image_url, order_items.quantity, order_items.total_price 
		FROM orders
		inner JOIN order_items ON order_items.order_id = orders.order_id
		JOIN products ON products.product_id = order_items.product_id
		WHERE user_id = $1 AND orders.order_id = $2"#, user_id, order_id).fetch_all(pool.get_ref()).await.map_err(GetOrdersError::SqlxError)?;

    let mut order_de_res = OrderDetailsResponse {
        order_date: DateTime::default(),
        total_price: BigDecimal::default(),
        order_details_product: Vec::<OrderDetailsProduct>::new(),
    };

    for order_de in order_details.iter() {
        let order_detail = OrderDetail {
            order_date: order_de.order_date,
            order_total_price: BigDecimal::from(0),
            order_item_total_price: order_de.total_price.clone(),
            product_id: order_de.product_id.clone().unwrap_or("".to_string()),
            product_name: order_de.product_name.clone(),
            image_url: order_de.image_url.clone(),
            quantity: order_de.quantity,
        };

        let order_de_pro = OrderDetailsProduct {
            product_id: order_detail.product_id,
            product_name: order_detail.product_name,
            image_url: order_detail.image_url,
            quantity: order_detail.quantity,
            total_price: order_detail.order_item_total_price,
        };

        order_de_res.order_details_product.push(order_de_pro);
    }

    order_de_res.order_date = order_details[0].order_date;
    order_de_res.total_price = BigDecimal::default();

    res.data = order_de_res;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum GetOrdersError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl ResponseError for GetOrdersError {}

impl std::fmt::Display for GetOrdersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to send payment link.")
    }
}
