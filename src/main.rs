use macroquad::math::Vec2;
use macroquad::prelude::*;
const RADIUS: f32 = 4.;
#[macroquad::main("physique-simulation-2")]
async fn main() -> anyhow::Result<()> {
    let mut entity = vec![];

    loop {
        clear_background(BLACK);

        draw_text(
            &format!("{}, {}", get_fps(), entity.len()),
            10.,
            100.,
            50.,
            WHITE,
        );

        if let Some(ball) = creat_ball() {
            entity.push(ball);
        }

        for _i in 0..7 {
            colition(&mut entity);
            apply_contraint(&mut entity);
            for i in &mut entity {
                verlet(i);
                draw_circle(i.pos.x, i.pos.y, RADIUS, RED);
            }
        }
        next_frame().await
    }
}

fn creat_ball() -> Option<Ball> {
    if is_key_down(KeyCode::Space) {
        let ball = Ball {
            der_pos: Vec2::new(screen_width() / 2. + 100., screen_height() / 2.),
            pos: Vec2::new(screen_width() / 2. + 100., screen_height() / 2.),
            accel: Vec2::ZERO,
        };

        return Some(ball);
    }

    if bouton(0., screen_height() - 50., 50., 50., WHITE) {
        let ball = Ball {
            der_pos: Vec2::new(screen_width() / 2. + 100., screen_height() / 2.),
            pos: Vec2::new(screen_width() / 2. + 100., screen_height() / 2.),
            accel: Vec2::ZERO,
        };

        return Some(ball);
    }

    return None;
}

fn verlet(ball: &mut Ball) {
    ball.accel = Vec2::new(0., 1000.);
    let velocity = ball.pos - ball.der_pos;
    ball.der_pos = ball.pos;
    ball.pos = ball.pos + velocity + ball.accel * get_frame_time() / 7. * get_frame_time() / 7.;
    ball.accel = Vec2::new(0., 0.);
}

fn apply_contraint(entity: &mut Vec<Ball>) {
    let position = Vec2::new(screen_width() / 2., 100.);
    let radius = 600.;
    draw_circle(position[0], position[1], radius, WHITE);
    for i in entity {
        let to_object = i.pos - position;
        let dist = pythagor(i.pos, position);

        if dist > radius - RADIUS {
            let n = to_object / dist;
            i.pos = position + n * (radius - RADIUS)
        }
    }
}

fn colition(entity: &mut Vec<Ball>) {
    for i in 0..entity.len() {
        for a in 0..entity.len() as usize {
            if entity[i] != entity[a] {
                let colision_axis = entity[a].pos - entity[i].pos;
                let dist = pythagor(entity[a].pos, entity[i].pos);

                if dist < RADIUS * 2. {
                    let n = colision_axis / dist;
                    let delta = RADIUS * 2. - dist;
                    let avencer = 0.5 * delta * n;
                    entity[i].pos -= avencer;
                    entity[a].pos += avencer;
                }
            }
        }
    }
}

fn bouton(x: f32, y: f32, w: f32, h: f32, color: Color) -> bool {
    draw_rectangle(x, y, w, h, color);
    let touchet = touches();
    for touch in touchet {
        if x < touch.position[0]
            && touch.position[0] < x + w
            && y < touch.position[1]
            && touch.position[1] < y + h
        {
            return true;
        }
    }

    return false;
}

fn pythagor(pos1: Vec2, pos2: Vec2) -> f32 {
    let vector_dist = pos2 - pos1;
    let hypotenuse_carre = vector_dist[0] * vector_dist[0] + vector_dist[1] * vector_dist[1];
    let norme = hypotenuse_carre.sqrt();
    return norme;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ball {
    pos: Vec2,
    der_pos: Vec2,
    accel: Vec2,
}
