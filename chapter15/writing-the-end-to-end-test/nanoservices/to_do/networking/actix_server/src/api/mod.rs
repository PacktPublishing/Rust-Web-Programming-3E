pub mod basic_actions;

use actix_web::web::ServiceConfig;


pub fn views_factory(app: &mut ServiceConfig) {
    basic_actions::basic_actions_factory(app);
}
