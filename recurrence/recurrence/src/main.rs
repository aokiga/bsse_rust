mod cycle_buf;

use cycle_buf::CycleBuffer;

macro_rules! recurrence {
    ($seq:ident[$index:ident]: $type:ty = $($init:expr), *; ...; $calc:expr) => {{
        struct Recurrence {
            cycle_buffer: CycleBuffer<$type>,
            pos: usize
        }

        impl Iterator for Recurrence {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                let $index = self.pos;
                self.pos += 1;
                if $index < self.cycle_buffer.len() {
                    let res = self.cycle_buffer[$index];
                    return Some(res);
                }
                let $seq = &mut self.cycle_buffer;
                let value = $calc;
                $seq.push(value);
                return Some(value);
            }
        }

        let mut cycle_buffer = CycleBuffer::new(Some(64));
        $({
            cycle_buffer.push($init);
        })*

        Recurrence {
            cycle_buffer: cycle_buffer,
            pos: 0,
        }
    }}
}

fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-1] + a[n-2]];

    for n in fib.take(10) { print!("{} ", n); }
    // 0 1 1 2 3 5 8 13 21 34

    println!();

    let other = recurrence![f[i]: f64 = 1.0; ...; f[i-1] * i as f64];
    for n in other.take(10) { print!("{} ", n); }
    //1 1 2 6 24 120 720 5040 40320 362880
}