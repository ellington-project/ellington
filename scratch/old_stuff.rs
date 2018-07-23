
#[flame]
fn process_library(library: Library) -> () {
    info!("Successfully parsed {} tracks.", library.tracks.len());
    // Iterate over the tracks.
    for track in library.tracks {
        // flame::start("process_track");

        // Match the tracks that contain ellington data
        match track.ellington_data() {
            // If we have ellington data
            Some(ed) => {
                info!("Track: {}", track);
                info!("Bpm: {:?}", track.bpm());
                info!("Comment: {:#?}", track.comments());
                info!("Ed: {:#?}", ed);

                let mut call = FfmpegCommand::default(&track.location());
                let mut child = call.run();

                let cbpm = {
                    let sox_stream = match &mut child.stdout {
                        Some(s) => Some(AudioStream::from_stream(s)),
                        None => None,
                    }.unwrap();

                    let calculated_bpm =
                        BpmTools::default().analyse(sox_stream);

                    calculated_bpm
                };

                child.wait().expect("failed to wait on child");

                info!("Calculated ffmpeg bpm: {}", cbpm);

                let mut call = SoxCommand::default(&track.location());
                let mut child = call.run();

                let cbpm = {
                    let sox_stream = match &mut child.stdout {
                        Some(s) => Some(AudioStream::from_stream(s)),
                        None => None,
                    }.unwrap();

                    let calculated_bpm =
                        BpmTools::default().analyse(sox_stream);

                    calculated_bpm
                };

                child.wait().expect("failed to wait on child");

                info!("Calculated sox bpm: {}", cbpm);

                // build some ellington data
                // let new_data = EllingtonData {
                //     algs: Some (
                //         vec![BpmInfo{
                //             bpm: cbpm as i64,
                //             alg: "naive".to_string()
                //         }]
                //     )
                // };

                // match track.write_data(new_data) {
                //     Some(_) => info!("Successfully written data."),
                //     None => info!("Failed to write id3 data for some reason.")
                // }

                info!("===== ===== ===== ===== =====\n");
            }
            _ => {
                info!("Ignore... {:?}", track.name());
            }
        }

        // flame::end("process_track");
    }
}