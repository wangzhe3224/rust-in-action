use byteorder::LittleEndian;

// use libactionkv::ActionKV;


#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
}

