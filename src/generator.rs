// TO-DO:
// - Threading
// - Spindle surface mode
// - Boring
// - Additional machine parameters

const CLEARANCE: f32 = 1.0;

pub enum JobType {
    Boring, // Not yet implemented
    Facing,
    FaceBoring,
    Turning,
    Drilling, //Not yet implemented
              // Threading,
}

pub struct JobParams {
    pub job_type: JobType,
    pub start_depth: f32,  // OD of material
    pub finish_depth: f32, // Final OD
    pub step: f32,         // DOC
    pub finish_step: f32,  // finishing DOC
    pub start_cut: f32,    // Start of Z
    pub length: f32,       // -ve implies cutting dir is away from chuck
    pub feed: f32,         // Feedrate
}

pub struct MachineParams {
    pub rpm: i32,
    pub spindle_cw: bool, // false -> CCW
    pub units_inch: bool, // true -> mm, default: mm
    pub tool_num: i32,
    pub radius_mode: bool, // false -> diameter mode (G7), default: (G8)
                           // clearance override
                           // chamfer parts - need cutter comp?
                           // surface mode
                           // surface speed
}

fn calculate_pass_depths(job: &JobParams) -> Vec<f32> {
    // Create vec with [finish, finish + finish_step]
    // Use while loop and add step size to previous val until previous val > start
    let mut last = job.finish_depth + job.finish_step;
    let mut passes = vec![job.finish_depth, last];
    while last < job.start_depth {
        last += job.step;
        passes.push(last);
    }
    passes
}

fn generate_z_cut(pass_depths: Vec<f32>, job: &JobParams) -> String {
    let mut code = String::new();
    let mut pass_depths = pass_depths;
    pass_depths.reverse();

    for i in pass_depths {
        code += &format!("G0 Z{:.3} \n   X{:.3}\n", job.start_cut + CLEARANCE, i); // Rapid to start + clearance in Z
        code += &format!(
            "G1 Z{:.3} F{} \n   X{:.3} \n",
            job.start_cut - job.length,
            job.feed,
            job.start_depth + CLEARANCE
        ); // Linear cut in Z axis + feed out
    }
    code
}

fn generate_x_cut(pass_depths: Vec<f32>, job: &JobParams) -> String {
    // Pass depths are in Z axis
    let mut code = String::new();
    let mut pass_depths = pass_depths;
    pass_depths.reverse();

    for i in pass_depths {
        code += &format!("G0 X{:.3}\n   Z{:.3}\n", job.start_cut, i + CLEARANCE); // Rapid to start + clearance in Z
        code += &format!(
            "G1 Z{:.3} F{}\n   X{:.3}\n   Z{:.3} \n",
            i,
            job.feed,
            job.start_cut - job.length,
            job.start_depth + CLEARANCE
        ); // Linear cut in X axis + feed out
    }
    code
}

fn turning_generation(job: &JobParams) -> String {
    // Pass depths correspond to X, cut in Z
    let passes = calculate_pass_depths(job);
    generate_z_cut(passes, job)
}

// fn threading_generation(job: &JobParams) -> String {
//
// }

// fn boring_generation(job: JobParams, machine: MachineParams) -> String {
//     // Pass depths correspond to X, cut in Z
//     // Use G85?, change X between cycles
//     // Use G89
//     let passes = calculate_pass_depths(job);
// }

fn facing_generation(job: &JobParams) -> String {
    // Pass depths correspond to Z, cut in X
    // Do the following outside before the data is passed by referenceL
    // let mut job1 = job.clone();
    // job1.length = job1.start_cut + 0.2;
    let passes = calculate_pass_depths(job);
    generate_x_cut(passes, job)
}

fn face_boring_generation(job: &JobParams) -> String {
    // Pass depths correspond to Z, cut in X
    let passes = calculate_pass_depths(job);
    generate_x_cut(passes, job)
}

fn machine_settings_generation(machine: &MachineParams) -> String {
    // Set up machine options (units, diameter/radius etc.)
    let units: &str = if machine.units_inch { "G20" } else { "G21" };
    let lathe_mode: &str = if machine.radius_mode { "G8" } else { "G7" };
    let tool_code = format!("M6 T{} G43\n", machine.tool_num);
    let dir: &str = if machine.spindle_cw { "M3" } else { "M4" };
    let spindle_code: String = format!("G97 S{} {}", machine.rpm, dir);

    // let spindle_code: String = match surface_mode {
    //     true => format!("G96 D{} S{} {}\n", rpm, surface_speed, dir),
    //     false => format!("G97 S{} {}\n", rpm, dir),
    //     _ => String::from("ERROR"),
    // };

    // G64 - path blending, G18 - Plane select
    let machine_code = format!(
        "G90 {} G64 G18 {}\n{}{}\n",
        units, lathe_mode, tool_code, spindle_code
    );
    machine_code
}

pub fn generate_gcode(job: &JobParams, machine: &MachineParams) -> String {
    let cut_code: String = match job.job_type {
        JobType::Boring => "ERROR: Boring functionality not yet available".to_string(), //boring_generation(job, machine),
        JobType::Facing => facing_generation(job),
        JobType::FaceBoring => face_boring_generation(job),
        JobType::Turning => turning_generation(job),
        JobType::Drilling => "ERROR: Drilling functionality not yet available".to_string(),
    };
    let machine_setings = machine_settings_generation(machine);
    format!("{}\n{}\nM2", machine_setings, cut_code)
}

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
