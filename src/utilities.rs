struct Dane {
    time1: (u32, u32, u32, u32, u32, f32),
    time2: (u32, u32, u32, u32, u32, f32),
    time3: (u32, u32, u32, u32, u32, f32),
    ra1: (u32, u32, f32),
    ra2: (u32, u32, f32),
    ra3: (u32, u32, f32),
    dec1: (u32, u32, f32),
    dec2: (u32, u32, f32),
    dec3: (u32, u32, f32),
}


fn file_config(args: &[String]) -> (&str ,String) {

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

(file_path, contents)
}
