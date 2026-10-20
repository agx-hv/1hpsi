const PSI_BREAKPOINTS: [f32; 8] = [0.0, 50.0, 100.0, 200.0, 300.0, 400.0, 500.0, f32::INFINITY];
const PM25_BREAKPOINTS: [f32; 8] = [0.0, 12.0, 55.0, 150.0, 250.0, 350.0, 500.0, f32::INFINITY];

pub fn pm25_to_psi(pm25: f32) -> Option<f32> {
    if pm25 > 500.0 {
        return Some(pm25);
    };

    let mut pm25_lower_bound = 0.0;
    let mut pm25_upper_bound = f32::INFINITY;
    let mut psi_lower_bound = 0.0;
    let mut psi_upper_bound = f32::INFINITY;

    for idx in 0..7 {
        pm25_lower_bound = PM25_BREAKPOINTS[idx];
        pm25_upper_bound = PM25_BREAKPOINTS[idx + 1];
        psi_lower_bound = PSI_BREAKPOINTS[idx];
        psi_upper_bound = PSI_BREAKPOINTS[idx + 1];
        if (pm25_lower_bound..=pm25_upper_bound).contains(&pm25) {
            break;
        }
    }

    match ilerp(pm25_lower_bound, pm25_upper_bound, pm25) {
        Some(t) => Some(lerp(psi_lower_bound, psi_upper_bound, t)),
        None => None,
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn ilerp(a: f32, b: f32, x: f32) -> Option<f32> {
    match a == b {
        true => None,
        false => Some((x - a) / (b - a)),
    }
}
