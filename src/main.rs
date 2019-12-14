use std::fs::File;
use std::io::{BufWriter, Write};

extern crate algorithms;
use algorithms::{k_means, KPoint, NPoint};

fn write_results(k_points: &Vec<KPoint>, n_points: &Vec<NPoint>) {
    let f = File::create("points.json").expect("Can not open file");
    let mut writer = BufWriter::new(f);
    writer.write("{\"k_points\":[".as_bytes());
    let mut i = 0usize;
    while i < k_points.len() {
        writer.write(k_points[i].to_json_string().as_bytes());
        i += 1;
        if i < k_points.len() {
            writer.write(",".as_bytes());
        } else {
            writer.write("],".as_bytes());
        }
    }
    writer.write("\"n_points\":[".as_bytes());
    i = 0usize;
    while i < n_points.len() {
        writer.write(n_points[i].to_json_string().as_bytes());
        i += 1;
        if i < n_points.len() {
            writer.write(",".as_bytes());
        } else {
            writer.write("]}".as_bytes());
        }
    }
    writer.flush();
}

fn main() {
    //println!("{} {} {} {} {}", 0.3 as i32, 0.5 as i32, 1.1 as i32, 2.3 as i32, 540.3 as i32);
    let (k_points, n_points) = k_means(1000000usize, 50usize, 600, 600, 0.0001f64);
    write_results(&k_points, &n_points);
}
