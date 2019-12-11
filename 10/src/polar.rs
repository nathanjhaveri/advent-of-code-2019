type PPt = f32;
pub struct PolarPt {
    theta: PPt,
    dist: PPt,
}

impl PolarPt {
    pub fn from_coords(x: i32, y: i32) -> PolarPt {
        let x = x as PPt;
        let y = y as PPt;

        let dist = (x.powi(2) + y.powi(2)).sqrt();
        let theta = (x / y).atan();

        PolarPt { theta, dist }
    }
}
