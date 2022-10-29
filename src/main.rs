use anyhow::{anyhow, Context, Result};
use log::*;
use mpris::{Metadata, MetadataValue, PlayerFinder};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize)]
struct TrackEvent {
    player: String,
    artist: String,
    title: String,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let player_finder = PlayerFinder::new().context("Could not connect to D-Bus")?;
    let player = player_finder.find_active().context("No player found")?;
    debug!("Showing event stream for player {}", &player.identity());

    let metadata = get_metadata(&player)?;
    debug!("Metadata: {metadata:?}");
    output_track(&metadata, true);

    let events = player.events().expect("Could not start event stream");
    let start = Instant::now();

    for event in events {
        match event {
            Ok(event) => {
                match &event {
                    mpris::Event::TrackChanged(metadata) => {
                        output_track(&metadata, true);
                    }
                    _ => (),
                }
                debug!("{}: {:#?}", format_elapsed(start.elapsed()), event);
            }
            Err(err) => {
                error!("D-Bus error: {}. Aborting.", err);
                break;
            }
        }
    }
    Ok(())
}

fn output_track(metadata: &Metadata, emit_json_stdout: bool) {
    let artist = if let Some(artists) = metadata.get("xesam:artist") {
        if let Some(v) = artists.as_str_array() {
            v.join(", ")
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    let title = if let Some(MetadataValue::String(title)) = metadata.get("xesam:title") {
        title.to_string()
    } else {
        "".to_string()
    };

    info!("{artist}::::{title}");

    if emit_json_stdout {
        let data = TrackEvent {
            player: artist.clone(),
            artist: artist.clone(),
            title: title.clone(),
        };
        let json = serde_json::to_string(&data).unwrap();
        println!("{json}");
    }
}

fn get_metadata(player: &mpris::Player) -> Result<mpris::Metadata> {
    info!(
        "Found {identity} (on bus {bus_name})",
        bus_name = player.bus_name(),
        identity = player.identity(),
    );

    let metadata = player
        .get_metadata()
        .map_err(|e| anyhow!("{e:?}"))
        .context("Could not get metadata for player")?;
    Ok(metadata)
}

fn format_elapsed(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let seconds_left = seconds - (60 * minutes);
    let ms = duration.subsec_millis();
    format!("{:02}:{:02}.{:3}", minutes, seconds_left, ms)
}
