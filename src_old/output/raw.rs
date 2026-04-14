use crate::{CliError, Infos};

pub fn error(error: CliError) {
    eprintln!("{}", error);
}

pub fn infos(infos: Infos) {
    println!(
        "played_media_type: {}, played_media_state: {}, played_media_id: {}, played_media_context_id: {}, played_media_position: {}, time_shifting_state: {}, mac_address: {}, wol_support: {}, friendly_name: {}, active_standby_state: {}, npvr_support: {}",
        infos.played_media_type,
        infos.played_media_state,
        infos.played_media_id,
        infos.played_media_context_id,
        infos.played_media_position,
        infos.time_shifting_state,
        infos.mac_address,
        infos.wol_support,
        infos.friendly_name,
        infos.active_standby_state,
        infos.npvr_support,
    );
}
