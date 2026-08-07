use hvz::human::Human;
use hvz::position::Position;
use hvz::world::World;
use hvz::zombie::Zombie;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 20;
const HUMAN_COUNT: usize = 30;
const ZOMBIE_COUNT: usize = 15;

fn random_positions(
  count: usize,
  width: u32,
  height: u32,
  occupied: &mut HashSet<(i32, i32)>,
  rng: &mut impl Rng,
) -> Vec<Position> {
  let mut positions = Vec::with_capacity(count);
  while positions.len() < count {
    let x = rng.gen_range(0..width as i32);
    let y = rng.gen_range(0..height as i32);
    if occupied.insert((x, y)) {
      positions.push(Position::new(x, y));
    }
  }
  positions
}

fn run_game() {
  let mut rng = rand::thread_rng();
  let mut occupied = HashSet::new();

  let human_positions = random_positions(HUMAN_COUNT, WIDTH, HEIGHT, &mut occupied, &mut rng);
  let zombie_positions = random_positions(ZOMBIE_COUNT, WIDTH, HEIGHT, &mut occupied, &mut rng);

  let mut world = World::new(WIDTH, HEIGHT);
  for pos in human_positions {
    world.add_human(Human::new(pos));
  }
  for pos in zombie_positions {
    world.add_zombie(Zombie::new(pos));
  }

  world.run();
}

fn run_bench() {
  const GRID_W: u32 = 1000;
  const GRID_H: u32 = 1000;
  const SPACING: u32 = 30;
  const ZOMBIE_COUNT: usize = 500;
  const TICKS: u32 = 100;

  let mut rng = StdRng::seed_from_u64(42);
  let mut occupied = HashSet::new();

  let mut world = World::new(GRID_W, GRID_H);
  let mut human_count = 0;
  let mut x = 0;
  while x < GRID_W {
    let mut y = 0;
    while y < GRID_H {
      let pos = Position::new(x as i32, y as i32);
      occupied.insert((x as i32, y as i32));
      world.add_human(Human::new(pos));
      human_count += 1;
      y += SPACING;
    }
    x += SPACING;
  }

  for _ in 0..ZOMBIE_COUNT {
    let pos = loop {
      let x = rng.gen_range(0..GRID_W as i32);
      let y = rng.gen_range(0..GRID_H as i32);
      if occupied.insert((x, y)) {
        break Position::new(x, y);
      }
    };
    world.add_zombie(Zombie::new(pos));
  }

  eprintln!("Grid: {}x{}", GRID_W, GRID_H);
  eprintln!(
    "Humans: {}, Zombies: {} (seeded random)",
    human_count, ZOMBIE_COUNT
  );
  eprintln!("---");

  let mut total_movement = Duration::ZERO;
  let mut total_infection = Duration::ZERO;
  let mut total_decay = Duration::ZERO;

  let start = Instant::now();
  for tick in 0..TICKS {
    let (dm, di, dd) = world.process_turn();
    total_movement += dm;
    total_infection += di;
    total_decay += dd;
    let h = world.count_humans();
    let z = world.count_zombies();
    eprintln!("Tick {:2}: {:4} humans, {:4} zombies", tick + 1, h, z);
  }
  let elapsed = start.elapsed();
  eprintln!("---");
  eprintln!("Total: {:?}", elapsed);
  eprintln!("Per tick: {:?}", elapsed / TICKS);
  eprintln!("---");
  eprintln!("Averages over {} ticks:", TICKS);
  eprintln!("  movement: {:?}", total_movement / TICKS);
  eprintln!("  infection: {:?}", total_infection / TICKS);
  eprintln!("  decay: {:?}", total_decay / TICKS);
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.iter().any(|a| a == "bench" || a == "--bench") {
    run_bench();
  } else {
    run_game();
  }
}
