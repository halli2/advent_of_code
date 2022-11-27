use std::{borrow::Cow, collections::HashSet};

use chumsky::prelude::*;
use wgpu::util::DeviceExt;

use crate::{
    gpu::{cast_bytes, Viewer},
    AdventSolver,
};

pub struct DayThree {}

#[derive(Debug, Clone, Copy)]
enum Instr {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
struct House {
    pub north: i32,
    pub east: i32,
}

fn parser() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
    use Instr::*;
    choice((
        just('<').to(West),
        just('>').to(East),
        just('^').to(North),
        just('v').to(South),
    ))
    .repeated()
    .then_ignore(end())
}

fn eval(ast: &[Instr]) -> i32 {
    use Instr::*;
    let mut houses = HashSet::new();
    let mut current_house = House { north: 0, east: 0 };
    houses.insert(current_house);
    let mut count = 1;
    for symbol in ast {
        match symbol {
            North => {
                current_house.north += 1;
            }
            East => {
                current_house.east += 1;
            }
            South => {
                current_house.north -= 1;
            }
            West => {
                current_house.east -= 1;
            }
        }
        if houses.insert(current_house) {
            count += 1;
        }
    }
    count
}

fn eval_2(ast: &[Instr]) -> i32 {
    use Instr::*;
    let mut houses = HashSet::new();
    let mut santa = House { north: 0, east: 0 };
    let mut robot = House { north: 0, east: 0 };
    houses.insert(santa);
    let mut count = 1;
    for (index, symbol) in ast.into_iter().enumerate() {
        let (n, e) = match symbol {
            North => (1, 0),
            East => (0, 1),
            South => (-1, 0),
            West => (0, -1),
        };
        if index % 2 == 0 {
            santa.north += n;
            santa.east += e;
            if houses.insert(santa) {
                count += 1;
            }
        } else {
            robot.north += n;
            robot.east += e;
            if houses.insert(robot) {
                count += 1;
            }
        }
    }
    count
}

impl AdventSolver for DayThree {
    fn part_one(&self, input: &str) -> String {
        match parser().parse(input.trim()) {
            Ok(ast) => eval(&ast).to_string(),
            Err(errs) => {
                errs.into_iter().for_each(|e| println!("{:?}", e));
                "Error".to_string()
            }
        }
    }

    fn part_two(&self, input: &str) -> String {
        match parser().parse(input.trim()) {
            Ok(ast) => eval_2(&ast).to_string(),
            Err(errs) => {
                errs.into_iter().for_each(|e| println!("{:?}", e));
                "Error".to_string()
            }
        }
    }

    fn visualize(&self, input: &str) {
        let viewer = Viewer::new((200, 200)).unwrap();
        let device = &viewer.ctx.device;

        let inp = input.trim().chars().collect::<Vec<_>>();
        let data = unsafe { cast_bytes(&inp) };

        let src_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::STORAGE,
        });

        let house_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { cast_bytes(&[100_u32, 100_u32, 0_u32]) },
            usage: wgpu::BufferUsages::STORAGE,
        });

        let compute_pipeline = viewer.ctx.compute_pipeline(
            Cow::Borrowed(include_str!("../shaders/cs_2015_3.wgsl")),
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: src_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: house_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&viewer.texture_view),
                },
            ],
        );

        viewer.run(move |device, queue| {
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut cpass =
                    encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
                cpass.set_pipeline(&compute_pipeline.raw);
                cpass.set_bind_group(0, &compute_pipeline.bind_group, &[]);
                cpass.dispatch_workgroups(1, 1, 1);
            }
            queue.submit(Some(encoder.finish()));
        });
    }
}
