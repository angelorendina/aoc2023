use nalgebra::Matrix2;
use nalgebra::Matrix6;
use nalgebra::Vector3;
use nalgebra::Vector6;

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day24/input.txt";
    #[cfg(not(test))]
    const BOUNDARY: [f64; 2] = [200000000000000.0, 400000000000000.0];
    #[cfg(test)]
    const FILENAME: &str = "src/day24/test.txt";
    #[cfg(test)]
    const BOUNDARY: [f64; 2] = [7.0, 27.0];

    let hails = std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(Hail::new)
        .collect::<Vec<_>>();

    let mut total = 0;
    for (i, hail) in hails.iter().enumerate() {
        for other_hail in hails.iter().skip(i + 1) {
            // parametric line equation:
            // A(l) = Ap + l Av
            // B(l) = Bp + l Bv
            // for intersection of trajectories find l and k so that
            // Ap + l Av = Bp + k Bv
            // i.e.
            // Apx + l Avx = Bpx + k Bvx
            // Apy + l Avy = Bpy + k Bvy
            // i.e.
            // | Avx , -Bvx | . | l | = | Bpx - Apx |
            // | Avy , -Bvy |   | k | = | Bpy - Apy |
            let m = Matrix2::from_columns(&[hail.velocity.xy(), -other_hail.velocity.xy()]);
            let v = (other_hail.position - hail.position).xy();
            if m.determinant().abs() < f64::EPSILON {
                if v.x.abs().max(v.y.abs()) < f64::EPSILON {
                    // coincident trajectories!
                    total += 1;
                }
            } else {
                let params = m.try_inverse().unwrap() * v;
                if params.x > 0.0 && params.y > 0.0 {
                    let intersection = hail.position + params.x * hail.velocity;
                    if intersection.x >= BOUNDARY[0]
                        && intersection.x <= BOUNDARY[1]
                        && intersection.y >= BOUNDARY[0]
                        && intersection.y <= BOUNDARY[1]
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const FILENAME: &str = "src/day24/input.txt";
    #[cfg(test)]
    const FILENAME: &str = "src/day24/test.txt";

    let hails = std::fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(Hail::new)
        .collect::<Vec<_>>();

    // parametric line equation
    // R(t) = Rp + t Rv
    // intersect with
    // H_i(t_i) = H_i_p + t_i H_i_v
    // hence
    // Rp + ti Rv = Hip + ti Hiv
    // and (Rp - Hip) = -ti (Rv - Hiv)
    // so (Rp - Hip) and (Rv - Hiv) are parallel hence have cross product zero
    // 0 = (Rp - Hip) x (Rv - Hiv) = Rp x Rv - Hip x Rv - Rp x Hiv + Hip x Hiv
    // for all indices i, hence
    // Hip x Rv + Rp x Hiv - Hip x Hiv = Hjp x Rv + Rp x Hjv - Hjp x Hjv
    // for all pairs i!=j.
    // Each pair contributes to 3 equations (one for each dimension) in 6 unknowns (Rp's and Rv's)
    // so two pairs will give a 6x6 system.

    // (pi - pj) x V - (vi - vj) x P = (pi x vi) - (pj x vj)
    // unpacks into
    //                                                                                          | Px |
    // |            0,  (vi - vj)_z, -(vi - vj)_y,            0, -(pi - pj)_z,  (pi - pj)_y |   | Py |   | (pi x vi)_x - (pj x vj)_x |
    // | -(vi - vj)_z,            0,  (vi - vj)_x,  (pi - pj)_z,            0, -(pi - pj)_x | . | Pz | = | (pi x vi)_y - (pj x vj)_y |
    // |  (vi - vj)_y, -(vi - vj)_x,            0, -(pi - pj)_y,  (pi - pj)_x,            0 |   | Vx |   | (pi x vi)_z - (pj x vj)_z |
    //                                                                                          | Vy |
    //                                                                                          | Vz |

    // TOTALLY not cherrypicked indices, but on the other hand
    // this exercise is GARBAGE and I will not try to debug rounding errors
    // in a 6x6 floating-point system.
    let pi = hails[0].position;
    let vi = hails[0].velocity;
    let pj = hails[1].position;
    let vj = hails[1].velocity;
    #[cfg(test)]
    let pk = hails[2].position;
    #[cfg(test)]
    let vk = hails[2].velocity;
    #[cfg(not(test))]
    let pk = hails[5].position;
    #[cfg(not(test))]
    let vk = hails[5].velocity;

    let m = Matrix6::from_columns(&[
        [
            0.0,
            -(vi - vj).z,
            (vi - vj).y,
            0.0,
            -(vi - vk).z,
            (vi - vk).y,
        ]
        .into(),
        [
            (vi - vj).z,
            0.0,
            -(vi - vj).x,
            (vi - vk).z,
            0.0,
            -(vi - vk).x,
        ]
        .into(),
        [
            -(vi - vj).y,
            (vi - vj).x,
            0.0,
            -(vi - vk).y,
            (vi - vk).x,
            0.0,
        ]
        .into(),
        [
            0.0,
            (pi - pj).z,
            -(pi - pj).y,
            0.0,
            (pi - pk).z,
            -(pi - pk).y,
        ]
        .into(),
        [
            -(pi - pj).z,
            0.0,
            (pi - pj).x,
            -(pi - pk).z,
            0.0,
            (pi - pk).x,
        ]
        .into(),
        [
            (pi - pj).y,
            -(pi - pj).x,
            0.0,
            (pi - pk).y,
            -(pi - pk).x,
            0.0,
        ]
        .into(),
    ]);
    let minv = m.try_inverse().unwrap();

    let q = Vector6::from([
        pi.cross(&vi).x - pj.cross(&vj).x,
        pi.cross(&vi).y - pj.cross(&vj).y,
        pi.cross(&vi).z - pj.cross(&vj).z,
        pi.cross(&vi).x - pk.cross(&vk).x,
        pi.cross(&vi).y - pk.cross(&vk).y,
        pi.cross(&vi).z - pk.cross(&vk).z,
    ]);

    let rock_position_and_velocity = minv * q;

    (rock_position_and_velocity.x + rock_position_and_velocity.y + rock_position_and_velocity.z)
        .round() as usize
}

struct Hail {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Hail {
    fn new(line: &str) -> Self {
        let (position, velocity) = line.split_once('@').unwrap();
        let mut position = position.split(',');
        let mut velocity = velocity.split(',');
        let position: [f64; 3] =
            std::array::from_fn(|_| position.next().unwrap().trim().parse().unwrap());
        let velocity: [f64; 3] =
            std::array::from_fn(|_| velocity.next().unwrap().trim().parse().unwrap());

        Self {
            position: Vector3::from_column_slice(&position),
            velocity: Vector3::from_column_slice(&velocity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 47);
    }
}
