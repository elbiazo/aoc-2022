use eframe::egui;
use egui::{
    plot::{Arrows, GridMark, Line, Plot, PlotPoint, PlotPoints, Points},
    Color32,
};
use itertools::Itertools;
use std::{fmt::Debug, time::Duration};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Debug, Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Snake {
    dirs: Vec<Direction>,
    cur_loc: usize,
    locations: Vec<Location>,
    tail_locations: Vec<Location>,
    step: bool,
    run: bool,
}

impl Snake {
    fn new() -> Self {
        let dirs = include_str!("../test.txt")
            .lines()
            .map(|l| {
                l.split(' ')
                    .tuple_windows()
                    .map(|(d, n)| -> Direction {
                        let n = n.parse().unwrap();
                        match d {
                            "U" => Direction::Up(n),
                            "D" => Direction::Down(n),
                            "L" => Direction::Left(n),
                            "R" => Direction::Right(n),
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect_vec();
        Self {
            dirs,
            cur_loc: 0,
            locations: vec![Location::default()],
            tail_locations: vec![Location::default()],
            step: false,
            run: false,
        }
    }
    // checks if two loc is adjacent as well as covered
    fn adj(&self, tail: Location, head: Location) -> bool {
        let x = tail.x as i32 - head.x as i32;
        let y = tail.y as i32 - head.y as i32;
        if x.abs() + y.abs() == 1 {
            true
        } else {
            false
        }
    }

    fn update(&mut self, dir: Direction) {
        let last_loc = self.locations.last().unwrap();
        match dir {
            Direction::Up(n) => {
                let locs = (1..=n)
                    .map(|i| Location::new(last_loc.x, last_loc.y + i))
                    .collect_vec();

                self.locations.extend(locs);
            }
            Direction::Down(n) => {
                let locs = (1..=n)
                    .map(|i| Location::new(last_loc.x, last_loc.y - i))
                    .collect_vec();

                self.locations.extend(locs);
            }
            Direction::Left(n) => {
                let locs = (1..=n)
                    .map(|i| Location::new(last_loc.x - i, last_loc.y))
                    .collect_vec();

                self.locations.extend(locs);
            }
            Direction::Right(n) => {
                let locs = (1..=n)
                    .map(|i| Location::new(last_loc.x + i, last_loc.y))
                    .collect_vec();

                self.locations.extend(locs);
            }
        }
    }
    fn get_points(&self) -> Vec<[f64; 2]> {
        self.locations
            .iter()
            .map(|l| [l.x as f64, l.y as f64])
            .collect()
    }
    fn get_tails(&self) -> Vec<[f64; 2]> {
        self.tail_locations
            .iter()
            .map(|l| [l.x as f64, l.y as f64])
            .collect()
    }

    fn update_all(&mut self) {
        for dir in self.dirs.clone() {
            self.update(dir);
        }
    }
}

// impl Debug for Snake {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     }
// }
fn main() {
    let mut snake = Snake::new();
    snake.update_all();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    // eframe::run_native("AoC 2022 — Day 9", options, Box::new(|_cc| Box::new(snake)));
}

impl eframe::App for Snake {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("inst").show(ctx, |ui| {
            ui.heading("Instructions:");
            if ui.button("Step").clicked() {
                self.step = true;
            }
            if ui.button("run").clicked() {
                self.run = true;
            }
            if ui.button("pause").clicked() {
                self.run = false;
            }
            if ui.button("reset").clicked() {
                self.run = false;
                *self = Self::new();
                self.update_all();
            }
            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                for dir in &self.dirs {
                    let arrow = match dir {
                        Direction::Up(d) => format!("up: {}", d),
                        Direction::Down(d) => format!("down: {}", d),
                        Direction::Left(d) => format!("left: {}", d),
                        Direction::Right(d) => format!("right: {}", d),
                    };
                    ui.label(arrow);
                }
            });
        });
        if self.step == true && self.run != true {
            if self.cur_loc < self.locations.len() - 1 {
                self.cur_loc += 1;
            }

            self.step = false;
        }

        if self.run {
            if self.cur_loc < self.locations.len() - 1 {
                self.cur_loc += 1;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Snake");

            Plot::new("my_plot").show(ui, |plot_ui| {
                let points = self.get_points()[..=self.cur_loc].to_vec();
                for (i, p) in points.into_iter().enumerate() {
                    let point = Points::new(vec![p]);
                    if i == self.cur_loc {
                        plot_ui.points(point.radius(5.0).color(Color32::GREEN));
                    } else {
                        plot_ui.points(point.radius(5.0).color(Color32::RED));
                    }
                }
                plot_ui.points(
                    Points::new(self.get_tails())
                        .radius(5.0)
                        .color(Color32::BLUE),
                );
            });
            ctx.request_repaint_after(Duration::from_millis(500));
        });
    }
}
