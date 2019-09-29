extern crate structopt;
use structopt::StructOpt;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod generator;

#[derive(StructOpt)]
#[structopt(name = "Turning code generator")]
/// Script to generate lathe turning GCode
struct Cli {
    #[structopt(short = "i", long = "initial-od")]
    /// Initial outside dimension [REQUIRED]
    initial_od: f32,
    #[structopt(short = "f", long = "final-od")]
    /// Final outside dimension [REQUIRED]
    final_od: f32,
    #[structopt(short = "c", long = "cut-depth")]
    /// Roughing cut depth (DOC) [REQUIRED]
    doc: f32,
    #[structopt(short = "s", long = "finish-cut-depth", default_value = "0")]
    /// Finishing cut depth (DOC) [REQUIRED]
    finish_doc: f32,
    #[structopt(short = "z", long = "z-begin", default_value = "0")]
    /// Z value that turning begins at
    start_z: f32,
    #[structopt(short = "l", long = "length")]
    /// Length of cut in negative Z direction [REQUIRED]
    length: f32,
    #[structopt(short = "r", long = "feed-rate")]
    /// Feed rate [REQUIRED]
    feed: f32,
    #[structopt(long = "rpm", default_value = "0")]
    /// Spindle RPM
    rpm: i32,
    #[structopt(long = "dir")]
    /// Spindle direction. This will add M4 - counter clockwise rotation. M3 is default
    dir: bool,
    #[structopt(long = "constant-surface")]
    /// Use constant surface instead of RPM - G96. Default is RPM mode - G97
    surface_mode: bool,
    #[structopt(long = "ss", default_value = "0")]
    /// Surface Speed, Use --rpm to set max rpm
    surface_speed: f32,
    #[structopt(short = "t", long = "tool", default_value = "0")]
    /// Tool number to use
    tool: i32,
    #[structopt(long = "inch")]
    /// Use inch as unit of measure - G20. Default is mm - G21
    inch: bool,
    #[structopt(long = "diameter")]
    /// Use diameter mode - G7. Default is radius mode - G8
    diameter_mode: bool,
    #[structopt(short = "g", long = "clearance", default_value = "0")]
    /// clearance for tool travel between passes. Defaults to zero which uses DOC as clearance value.
    clearance: f32,
    #[structopt(long = "chamfer")]
    /// Chamfer part after cut - not yet implemented
    chamfer: bool,
    #[structopt(short = "n", long = "file-name", default_value = "output.ngc")]
    /// Filename for output
    fname: String,
}

fn write_file(gcode: String, name: String) {
    let path = Path::new(&name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(gcode.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() {
    let args = Cli::from_args();
    // Check args

    let job = generator::JobParams{
        job_type: generator::JobType::Turning,
        start_depth: args.initial_od,
        finish_depth: args.final_od,
        step: args.doc,
        finish_step: args.finish_doc,
        start_cut: args.start_z,
        length: args.length,
        feed: args.feed
    };

    let machine = generator::MachineParams{
        rpm: args.rpm,
        spindle_cw: args.dir,
        units_inch: args.inch,
        tool_num: args.tool,
        radius_mode: !args.diameter_mode
    };

    let g_code = generator::generate_gcode(&job, &machine);
    println!("{}", g_code);
}

// fn main1() {
//     let args = Cli::from_args();
//     // add .ngc to filename
//     // consider a radius bool cl arg for clarity
//     // pass args to function, but do checking here.
//
//     // Options to add
//     // - clearance
//     // - different feedrate for final pass
//
//     if args.final_od > args.initial_od {
//         println!("Final outside dimension must be smaller than initial outside dimension.");
//         println!("Initial OD: {}", args.initial_od);
//         println!("Final   OD: {}", args.final_od);
//         std::process::exit(0);
//     }
//
//     generate_gcode(
//         args.initial_od,
//         args.final_od,
//         args.doc,
//         args.finish_doc,
//         args.start_z,
//         args.length,
//         args.feed,
//         args.rpm,
//         args.dir,
//         args.surface_mode,
//         args.surface_speed,
//         args.tool,
//         args.inch,
//         args.diameter_mode,
//         args.clearance,
//         args.chamfer,
//         args.fname,
//     );
// }
//
// fn generate_gcode(
//     initial_od: f32,
//     final_od: f32,
//     doc: f32,
//     finish_doc: f32,
//     start_z: f32,
//     length: f32,
//     feed: f32,
//     rpm: i32,
//     dir: bool,          // true for counter clock wise spindle direction
//     surface_mode: bool, // true for constant surface
//     surface_speed: f32,
//     tool: i32,
//     inch: bool,          // true for inch
//     diameter_mode: bool, // true for diameter - G7
//     clearance: f32,
//     chamfer: bool,
//     fname: String,
// ) {
//     // G & M codes from command line arguments:
//     let dir: &str = if dir { "M4" } else { "M3" };
//     let spindle_code: String = match surface_mode {
//         true => format!("G96 D{} S{} {}\n", rpm, surface_speed, dir),
//         false => format!("G97 S{} {}\n", rpm, dir),
//         _ => String::from("ERROR"),
//     };
//     let tool_code = format!("M6 T{} G43\n", tool);
//     let units: &str = if inch { "G20" } else { "G21" };
//     let lathe_mode: &str = if diameter_mode { "G7" } else { "G8" };
//
//     // G64 - path blending, G18 - Plane select
//     let start_code = format!("G90 {} G64 G18 {}\n", units, lathe_mode);
//     let finish_code = "M2";
//
//     // Get pass depths
//     let mut pass_depths = calculate_passes(initial_od, final_od, doc, finish_doc);
//     let clearance: f32 = if clearance == 0.0 {
//         doc + initial_od
//     } else {
//         clearance + initial_od
//     };
//
//     // generate GCode
//     let mut code = generate_surfacing_code(&mut pass_depths, start_z, length, feed, clearance);
//     code = start_code + &tool_code + &spindle_code + &code + &finish_code;
//
//     println!("{}", code);
//
//     write_file(code, fname);
// }



// fn generate_surfacing_code(
//     pass_depths: &mut Vec<f32>,
//     start_z: f32,
//     length: f32,
//     feed: f32,
//     clearance: f32,
// ) -> String {
//     // Generate G gcode
//     let mut code = String::new();
//
//     while !pass_depths.is_empty() {
//         let x_val = pass_depths.pop().unwrap(); // Get depth from stack
//         code += &format!("G0 Z{:.3} \n   X{:.3}\n", start_z, x_val); // Rapid to start
//         code += &format!("G1 Z-{:.3} F{} \n   X{:.3} \n", length, feed, clearance); // Linear cut
//     }
//     code
// }
//
// fn calculate_passes(initial_od: f32, final_od: f32, doc: f32, finish_doc: f32) -> Vec<f32> {
//     // Find amount of material to remove
//     let roughing_length = initial_od - (final_od + finish_doc);
//     let roughing_passes: i32 = { roughing_length / doc }.ceil() as i32;
//
//     // Create array -> first value finish_doc, add 'doc' to prev element 'roughing_passes' times
//     // Can turn this into a stack since you will be reversing it
//     let mut pass_depths = vec![finish_doc + final_od];
//     for i in 0..roughing_passes {
//         let prev_pass = pass_depths[i as usize];
//         pass_depths.push(prev_pass + doc);
//     }
//     return pass_depths;
// }

// fn generate_facing_code(
//     face_depths: &mut Vec<f32>,
//     start_x: f32,
//     feed: f32,
//     clearance: f32,
// ) -> String {
//     //Generate facing passes here
// }
