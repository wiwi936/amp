use commands::Result;
use models::application::{Application, Preferences};
use util;

pub fn edit(app: &mut Application) -> Result {
    let preference_buffer = Preferences::edit()?;
    util::add_buffer(preference_buffer, app)
}

pub fn reload(app: &mut Application) -> Result {
    app.preferences.borrow_mut().reload()
}
