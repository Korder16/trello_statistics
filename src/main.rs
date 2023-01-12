mod trello_api;
use trello_api::get_month_lists;
use trello_api::parse_month_list;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let list_id = "63bb271e49f41103014faa4a";
    let month_lists = get_month_lists(list_id).unwrap();
    parse_month_list(month_lists);
}
