type Int = i32;
const DIMMENSION: usize = 3;
type Space = [Int; DIMMENSION];

#[derive(Debug)]
pub struct Moon {
    pos: Space,
    vel: Space,
}

impl Moon {
    pub fn new(initial_pos: Space) -> Self {
        Self {
            pos: initial_pos,
            vel: [0, 0, 0],
        }
    }
}

pub fn velocity(moons: &mut [Moon]) {
    for moon in moons {
        for dim in 0..DIMMENSION {
            moon.pos[dim] += moon.vel[dim];
        }
    }
}

pub fn gravity(moons: &mut [Moon]) {
    let count = moons.len();
    for i in 0..count {
        for j in i..count {
            for dim in 0..DIMMENSION {
                if moons[i].pos[dim] < moons[j].pos[dim] {
                    moons[i].vel[dim] += 1;
                    moons[j].vel[dim] -= 1;
                } else if moons[j].pos[dim] < moons[i].pos[dim] {
                    moons[i].vel[dim] -= 1;
                    moons[j].vel[dim] += 1;
                }
            }
        }
    }
}

fn potential_energy(moon: &Moon) -> Int {
    moon.pos.iter().map(|&p| p.abs()).sum()
}

fn kenetic_energy(moon: &Moon) -> Int {
    moon.vel.iter().map(|&p| p.abs()).sum()
}

fn total_energy_moon(moon: &Moon) -> Int {
    potential_energy(moon) * kenetic_energy(moon)
}

pub fn advance_state(time: Int, moons: &mut [Moon]) {
    for _ in 0..time {
        gravity(moons);
        velocity(moons);
    }
}

pub fn total_energy_universe(moons: &[Moon]) -> Int {
    moons.iter().map(|moon| total_energy_moon(moon)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // <x=-1, y=0, z=2>
        // <x=2, y=-10, z=-7>
        // <x=4, y=-8, z=8>
        // <x=3, y=5, z=-1>

        let io = Moon::new([-1, 0, 2]);
        let europa = Moon::new([2, -10, -7]);
        let ganymede = Moon::new([4, -8, 8]);
        let callisto = Moon::new([3, 5, -1]);
        let mut moons = [io, europa, ganymede, callisto];
        advance_state(10, &mut moons);
        assert_eq!(179, total_energy_universe(&moons));
    }

    #[test]
    fn example_2() {
        // <x=-8, y=-10, z=0>
        // <x=5, y=5, z=10>
        // <x=2, y=-7, z=3>
        // <x=9, y=-8, z=-3>

        let io = Moon::new([-8, -10, 0]);
        let europa = Moon::new([5, 5, 10]);
        let ganymede = Moon::new([2, -7, 3]);
        let callisto = Moon::new([9, -8, -3]);
        let mut moons = [io, europa, ganymede, callisto];

        for i in 0..100 {
            if i % 10 == 0 {
                println!("Step: {}", i);
                for (i, moon) in moons.iter().enumerate() {
                    println!("moon {} {:?}", i, moon);
                }
            }

            gravity(&mut moons);
            velocity(&mut moons);
        }

        let mut enery = 0;
        for moon in moons.iter() {
            enery += total_energy_moon(moon);
        }
        assert_eq!(1940, enery);
    }

    #[test]
    fn twelve_1() {
        // <x=1, y=2, z=-9>
        // <x=-1, y=-9, z=-4>
        // <x=17, y=6, z=8>
        // <x=12, y=4, z=2>
        let io = Moon::new([1, 2, -9]);
        let europa = Moon::new([-1, -9, -4]);
        let ganymede = Moon::new([17, 6, 8]);
        let callisto = Moon::new([12, 4, 2]);
        let mut moons = [io, europa, ganymede, callisto];

        for _ in 0..1000 {
            gravity(&mut moons);
            velocity(&mut moons);
        }

        let mut enery = 0;
        for moon in moons.iter() {
            enery += total_energy_moon(moon);
        }
        assert_eq!(7471, enery);
    }
}
