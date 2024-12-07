fn swap<T: Copy>(slice: &mut [T], n1: usize, n2: usize) {
    let tmp: T = slice[n1];

    slice[n1] = slice[n2];
    slice[n2] = tmp;
}

pub fn sort_shell(mass: &mut [i64]) {
    let mut s = mass.len() / 2;

    while s > 0 {
        for i in s..mass.len() {
            let mut j = i - s;

            loop {
                if mass[j] > mass[j + s] {
                    swap::<i64>(mass, j, j + s);
                }

                if j >= s {
                    j = j - s;
                } else {
                    break;
                }
            }
        }
        s = s / 2;
    }
}
