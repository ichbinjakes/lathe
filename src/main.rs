use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod generator;

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

// fn get_string() -> io::Result<String> {
//     let mut buffer = String::new();
//
//     io::stdin().read_line(&mut buffer)?;
//
//     Ok(buffer)
// }

fn get_string(s: &str) -> String {
    //https://doc.rust-lang.org/std/io/type.Result.html
    let mut input = String::new();
    println!("{}", s);
    io::stdin().read_line(&mut input);
    String::from(input.trim())
}

fn get_bool(s: &str) -> bool {
    let input = get_string(s);
    let mut bool_val: Option<bool> = match input.as_ref() {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    };
    while bool_val == None {
        bool_val = Some(get_bool(s));
    }
    bool_val.unwrap()
}

fn get_f32(s: &str) -> f32 {
    let input = get_string(s);
    input.parse::<f32>().unwrap()
}

fn get_i32(s: &str) -> i32 {
    let input = get_string(s);
    input.parse::<i32>().unwrap()
}

fn main() {
    // Check args
    let input = get_string("Job type:    ");
    let job_type: generator::JobType = match input.trim().to_lowercase().as_ref() {
        "boring" => generator::JobType::Boring,
        "facing" => generator::JobType::Facing,
        "faceboring" => generator::JobType::FaceBoring,
        "turning" => generator::JobType::Turning,
        "drilling" => generator::JobType::Drilling,
        _ => {
            println!("Error: job type not recognised:");
            println!("  {}", input);
            std::process::exit(0)
        }
    };


    let radius = get_bool("Are you entering values as radius? (y/n):      ");
    let depth_start = get_f32("Start  depth:    ");
    let depth_end = get_f32("Finish depth:    ");
    let step = get_f32("Step   size :    ");
    let finish_step = get_f32("Finish step :    ");
    let start_cut = get_f32("Start  cut  :    ");
    let length = get_f32("Length      :    ");
    let feed = get_f32("Feed rate   :    ");

    let job = generator::JobParams {
        job_type: job_type,
        start_depth: depth_start,
        finish_depth: depth_end,
        step: step,
        finish_step: finish_step,
        start_cut: start_cut,
        length: length,
        feed: feed,
    };

    let rpm = get_i32("RPM:    ");
    let cw_dir = get_bool("Spindle CW (y/n):      ");
    let use_inch = !get_bool("Use mm     (y/n):      ");
    let tool = get_i32("Tool num        :      ");

    let machine = generator::MachineParams {
        rpm: rpm,
        spindle_cw: cw_dir,
        units_inch: use_inch,
        tool_num: tool,
        radius_mode: radius,
    };

    let g_code = generator::generate_gcode(&job, &machine);
    println!("{}", g_code);
}

// println!("Spindle CW (y/n):      ");
// io::stdin().read_line(&mut input);
// let cw_dir = match input.as_ref() {
//     "y" => true,
//     "n" => false,
//     _ => {
//         print!("Error: spindle dir not recognised!");
//         std::process::exit(0)
//     }
// };
// println!("Use mm     (y/n):      ");
// io::stdin().read_line(&mut input);
// let use_inch =  match input.as_ref() {
//     "y" => false,
//     "n" => true,
//     _ => {
//         print!("Error: units not recognised!");
//         std::process::exit(0)
//     }
// };
// println!("Use radius (y/n):      ");
// io::stdin().read_line(&mut input);
// let radius =  match input.as_ref() {
//     "y" => true,
//     "n" => false,
//     _ => {
//         print!("Error: spindle dir not recognised!");
//         std::process::exit(0)
//     }
// };

// ------------------------------------------

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
