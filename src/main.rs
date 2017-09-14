// https://sites.google.com/site/musicgapi/technical-documents/wav-file-format#inst

extern crate wavedit;

use wavedit::{ WavFile, InstrumentChunk };

use std::io;
use std::io::{ Write, Cursor, Read, Error, ErrorKind };
use std::fs::File;

fn main() {
    let reader = File::open("resources/smpl_cue.wav").expect("input wav to read correctly.");
    let mut wav = WavFile::read(reader).expect("wav to parse correctly");

    // wav.instrument_chunk = Some(InstrumentChunk {
    //     unshifted_note: 45,
    //     fine_tune: 0,
    //     gain: 0,
    //     low_note: 40,
    //     high_note: 50,
    //     low_vel: 10,
    //     high_vel: 120,
    // });

    use wavedit::{ SamplerChunk, SampleLoop, LoopType };
    wav.sampler_chunk = Some(SamplerChunk {
        manufacturer: 0,
        product: 0,
        sample_period: 10,
        midi_unity_note: 20,
        midi_pitch_fraction: 0,
        smpte_format: 0,
        smpte_offset: 0,
        sample_loops: vec![SampleLoop {
            id: 0,
            loop_type: LoopType::Forward,
            start: 10,
            end: 1000,
            fraction: 0,
            play_count: 0,
        }],
        sampler_data: Vec::new(),
    });

    // wav.sampler_chunk = None;
    // wav.cue_points = Vec::new();

    // let mut s=String::new();
    // print!("Low note [0]: ");
    // let _= io::stdout().flush();
    // io::stdin().read_line(&mut s).expect("Did not enter a correct string");

    let writer = File::create("output.wav").expect("output wav to create correctly.");
    let _ = WavFile::write(writer, wav);
}
