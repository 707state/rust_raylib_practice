use clap::Parser;
use rand::Rng;
use raylib::prelude::*;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100000)]
    count: i32,
    #[arg(short, long, default_value_t = 800)]
    screen_height: i32,
    #[arg(short, long, default_value_t = 800)]
    screen_width: i32,
}
pub struct Particle {
    pos: Vector2,
    vel: Vector2,
    color: Color,
}
impl Particle {
    pub fn new(screen_width: i32, screen_height: i32) -> Particle {
        let mut rng = rand::thread_rng();
        Particle {
            pos: Vector2::new(
                rng.gen_range(0..screen_width) as f32,
                rng.gen_range(0..screen_height) as f32,
            ),
            vel: Vector2::new(
                rng.gen_range(-100..=100) as f32 / 100.0,
                rng.gen_range(-100..=100) as f32 / 100.0,
            ),
            color: Color::new(0, 0, 0, 100),
        }
    }
    pub fn with_values(pos: Vector2, vel: Vector2, color: Color) -> Particle {
        Particle { pos, vel, color }
    }

    // 计算到其他位置的距离
    fn get_dist(&self, other_pos: Vector2) -> f32 {
        let dx = self.pos.x - other_pos.x;
        let dy = self.pos.y - other_pos.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }
    // 计算标准化的方向向量
    fn get_normal(&self, other_pos: Vector2) -> Vector2 {
        let dist = self.get_dist(other_pos).max(0.5);
        let dx = self.pos.x - other_pos.x;
        let dy = self.pos.y - other_pos.y;
        Vector2::new(dx / dist, dy / dist)
    }

    // 吸引到某个位置
    pub fn attract(&mut self, pos_to_attract: Vector2, _multiplier: f32) {
        let normal = self.get_normal(pos_to_attract);
        let dist = self.get_dist(pos_to_attract).max(0.5);

        self.vel.x -= normal.x / dist;
        self.vel.y -= normal.y / dist;
    }

    // 施加摩擦力
    pub fn do_friction(&mut self, amount: f32) {
        self.vel.x *= amount;
        self.vel.y *= amount;
    }

    // 移动粒子并处理屏幕边界
    pub fn move_particle(&mut self, screen_width: i32, screen_height: i32) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.x < 0.0 {
            self.pos.x += screen_width as f32;
        }
        if self.pos.x >= screen_width as f32 {
            self.pos.x -= screen_width as f32;
        }
        if self.pos.y < 0.0 {
            self.pos.y += screen_height as f32;
        }
        if self.pos.y >= screen_height as f32 {
            self.pos.y -= screen_height as f32;
        }
    }

    // 绘制像素
    pub fn draw_pixel(&self, d: &mut RaylibDrawHandle) {
        d.draw_pixel_v(self.pos, self.color);
    }
}
fn main() {
    // 初始化屏幕大小

    let args = Args::parse();
    let particle_count = args.count;
    let screen_height = args.screen_height;
    let screen_width = args.screen_width;
    let mut particles: Vec<Particle> = (0..particle_count)
        .map(|_| Particle::new(screen_width, screen_height))
        .collect();

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Raylib Particle System")
        .build();

    rl.set_target_fps(60);

    // 主循环
    while !rl.window_should_close() {
        let mouse_pos = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);

        // 更新每个粒子的状态
        for particle in &mut particles {
            particle.attract(mouse_pos, 1.0);
            particle.do_friction(0.99);
            particle.move_particle(screen_width, screen_height);
        }

        // 绘制每个粒子
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for particle in &particles {
            particle.draw_pixel(&mut d);
        }

        d.draw_fps(10, 10);
    }
}
