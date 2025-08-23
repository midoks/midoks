use crate::db::pool;

fn admin_create() -> None {
    let t = pool::Manager::instance();
    print!("{:?}", t);
}
