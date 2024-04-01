use tauri::State;

use crate::{config::Config, ConfigHolderState, MaaZResult};

#[tauri::command]
pub async fn get_config(config_holder: State<'_, ConfigHolderState>) -> MaaZResult<Config> {
    let config_holder = config_holder.lock().await;
    Ok(config_holder.config())
}

macro_rules! config_writer {
    ($setter_name:ident,$field_type:tt,$writer:expr) => {
        #[tauri::command]
        pub async fn $setter_name(
            value: $field_type,
            config_holder: State<'_, ConfigHolderState>,
        ) -> MaaZResult<()> {
            let mut config_holder = config_holder.lock().await;
            config_holder.write(|config| $writer(config, value))?;
            Ok(())
        }
    };

    ($setter_name:ident,$sub_config:ident, $field:ident,$field_type:tt) => {
        #[tauri::command]
        pub async fn $setter_name(
            value: $field_type,
            config_holder: State<'_, ConfigHolderState>,
        ) -> MaaZResult<()> {
            let mut config_holder = config_holder.lock().await;
            config_holder.write(|config| {
                config.$sub_config.$field = value.into();
            })?;
            Ok(())
        }
    };
}

config_writer!(set_client_type, start_up, client_type, String);
