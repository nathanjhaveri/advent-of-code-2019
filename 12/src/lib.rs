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

pub fn velocity(moons: &mut [&mut Moon]) {
    for moon in moons {
        for dim in 0..DIMMENSION {
            moon.pos[dim] += moon.vel[dim];
        }
    }
}

pub fn gravity(moons: &mut [&mut Moon]) {
    let count = moons.len();
    for i in 0..count {
        for j in i..count {
            let mut_moons = moons.as_mut_ptr();
            unsafe {
                let first: *mut &mut Moon = mut_moons.add(i);
                let second: *mut &mut Moon = mut_moons.add(j);
                for dim in 0..DIMMENSION {
                    if (*first).pos[dim] < (*second).pos[dim] {
                        (*first).vel[dim] -= 1;
                        (*second).vel[dim] += 1;
                    } else if (*second).pos[dim] < (*first).pos[dim] {
                        (*first).vel[dim] += 1;
                        (*second).vel[dim] -= 1;
                    }
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

pub fn total_energy(moon: &Moon) -> Int {
    potential_energy(moon) + kenetic_energy(moon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        //        <x=-1, y=0, z=2>
        //<x=2, y=-10, z=-7>
        //<x=4, y=-8, z=8>
        //<x=3, y=5, z=-1>

        let mut io = Moon::new([-1, 0, 2]);
        let mut europa = Moon::new([2, -10, -7]);
        let mut ganymede = Moon::new([4, -8, 8]);
        let mut callisto = Moon::new([3, 5, -1]);
        let mut moons = [&mut io, &mut europa, &mut ganymede, &mut callisto];

        for _ in 0..10 {
            gravity(&mut moons);
            velocity(&mut moons);

            println!("moons: {:?}", moons);
        }

        let mut enery = 0;
        for moon in moons.iter() {
            enery += total_energy(moon);
        }
        assert_eq!(1, enery);
    }

    #[test]
    fn it_works() {
        // <x=1, y=2, z=-9>
        // <x=-1, y=-9, z=-4>
        // <x=17, y=6, z=8>
        // <x=12, y=4, z=2>

        let mut io = Moon::new([1, 2, 9]);
        let mut europa = Moon::new([1, -9, -4]);
        let mut ganymede = Moon::new([17, 6, 8]);
        let mut callisto = Moon::new([12, 4, 2]);
        let mut moons = [&mut io, &mut europa, &mut ganymede, &mut callisto];

        for _ in 0..1000 {
            gravity(&mut moons);
            velocity(&mut moons);
        }

        let mut enery = 0;
        for moon in moons.iter() {
            enery += total_energy(moon);
        }
        assert_eq!(1, enery);
    }
}
