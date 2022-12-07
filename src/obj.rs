use std::{io::{Error, Read}, fs::File};

use crate::{triangle::{self, Triangle}, point::{self, Point}};

pub struct Obj {
    name: String,
    pub points: Vec<point::Point>,
    pub mesh: Vec<triangle::Triangle>,
    normals: Vec<point::Point>,
    textures: Vec<point::Point>,
    materials: String,
    pub pos: Point
}

struct Vertex {
    point: Point,
    texture: String,
    normal: String
}

impl Obj {
    pub fn new(file_obj: String, pos: Point) -> Obj {
        let result: Result<Obj, std::io::Error> = parse_obj(file_obj, pos);
        if result.is_err() {
            println!("{:?}", result.as_ref().err())
        } else {
            println!("all good");
        }

        return result.unwrap();
    }
}

// this can not validate that a proper shape will be made, however it can check nothing is bad
fn parse_obj(file: String, pos: Point) -> Result<Obj, Error> {
    let mut name: String = "untitled".to_string();
    let mut points: Vec<Point> = Vec::new();
    let mut mesh: Vec<Triangle> = Vec::new();
    let mut normals: Vec<Point> = Vec::new();
    let mut textures: Vec<Point> = Vec::new();

    let mut f = File::open(file)?;
    let mut buffer = [0; 1];
    let mut line: String;
    
    // read exactly 1 byte
    loop {
        let possible_err = f.read_exact(&mut buffer);
        // before it even adds anything. for safety
        if possible_err.is_err() {
            break;
        }
        
        line = std::str::from_utf8(&buffer).unwrap().to_string();

        // read a line
        loop {
            f.read_exact(&mut buffer)?;
    
            line += std::str::from_utf8(&buffer).unwrap();
    
            if buffer[0] == 0x0A { // \n or EOF
                break;
            }
        }

        let parts: Vec<&str> = line.split(" ").collect();
        // if the line is empty, just stop
        if parts.len() < 1 {
            continue;
        }

        // do certain stuff based on the command
        match parts[0] {
            "#" => continue, // if its a comment, dont read it
            "o" => name = parts[1].to_string(),
            "v" => points.push(line_to_point(parts)),
            "vn" => normals.push(line_to_point(parts)),
            "vt" => textures.push(line_to_point(parts)),
            "usemtl" => println!("{}", parts[1]),
            "f" => mesh.append(&mut line_to_face(parts, points.as_ref())),
            _ => println!("? {}", parts[0])
        }
    }

    let obj: Obj = Obj {
        name: name.to_string(),
        points: points,
        mesh: mesh,
        normals,
        textures,
        materials: "".to_string(),
        pos
    };

    return Ok(obj);
}

// also handles vn and vt
fn line_to_point(parts: Vec<&str>) -> Point {
    // let magic = 3.0;

    // y, z are special, so is w
    let y = if parts.len() >= 3 { parts[2].trim().parse().unwrap() } else { 0.0 };
    let z = if parts.len() == 4 { parts[3].trim().parse().unwrap() } else { 0.0 }; // + magic; // shouldnt be more, by std
    Point { x: parts[1].parse().unwrap(), y, z, w: 1.0 }
}

fn line_to_face(parts: Vec<&str>, points: &Vec<point::Point>) -> Vec<Triangle> {
    let mut faces: Vec<Triangle> = Vec::new();
    // if its less than 3 i dont even think its worth it
    if parts.len() < 4 {
        return faces;
    }

    let v1 = part_to_vertex(parts[1].to_string(), points.as_ref()).point;
    let mut v_prev = part_to_vertex(parts[2].trim().to_string(), points.as_ref()).point;
    let v_cur = part_to_vertex(parts[3].trim().to_string(), points.as_ref()).point;

    faces.push(Triangle { // first triangle
        a: v1,
        b: v_cur,
        c: v_prev
    });

    // do this for any vertex after the third
    for j in 4..parts.len() {
        v_prev = part_to_vertex(parts[j-1].to_string(), points.as_ref()).point;
        faces.push(Triangle { a: part_to_vertex(parts[j].trim().to_string(), points.as_ref()).point, b: v1, c: v_prev })
    }

    return faces;
}

fn part_to_vertex(part: String, points: &Vec<Point>) -> Vertex {
    let attrs: Vec<&str> = part.split("/").collect();
    let i: usize = match attrs[0].parse() {
        Err(e) => panic!("error on \"{}\" : {}", part, e),
        Ok(i) => i
    };

    let point = points[i-1];

    return Vertex {
        point: point,
        texture: "todo!()".to_string(),
        normal: "todo!()".to_string(),
    };
}
