use std::{
    hint::unreachable_unchecked,
    io::{self, BufRead, ErrorKind},
    num::NonZeroU8,
};

pub fn part1_inlined_input(input: &str) -> u32 {
    input
        .lines()
        .map(|bruh| {
            let first = bruh.as_bytes().iter().find(|x| x.is_ascii_digit());
            let second = bruh.as_bytes().iter().rfind(|x| x.is_ascii_digit());

            // We assume input is always correct format with 2 digits at start and end
            let first = unsafe { *first.unwrap_unchecked() } - b'0';
            let second = unsafe { *second.unwrap_unchecked() } - b'0';

            first as u32 * 10 + second as u32
        })
        .sum()
}

pub fn part1_readbuf(mut buf: impl BufRead) -> u32 {
    let needle = jetscii::bytes!(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'\n');

    let mut total = 0;
    let mut first = Option::<NonZeroU8>::None;
    let mut last = Option::<NonZeroU8>::None;

    while let Some(byte) = unsafe { read_until(&mut buf, &needle).unwrap_unchecked() } {
        if byte == b'\n' {
            let (first, last) = match (first.take(), last.take()) {
                (Some(first), None) => (first, first),
                (Some(first), Some(last)) => (first, last),
                _ => unsafe { unreachable_unchecked() },
            };

            let calibration_value = first.get() as u32 * 10 + last.get() as u32;
            total += calibration_value;
            continue;
        }

        let num = unsafe { NonZeroU8::new_unchecked(byte - b'0') };

        match (first, last) {
            (None, Some(_)) => unsafe { unreachable_unchecked() },
            (None, None) => first = Some(num),
            (Some(_), _) => last = Some(num),
        }
    }

    total
}

pub fn read_until<R: BufRead + ?Sized, F: Fn(u8) -> bool>(
    r: &mut R,
    needle: &jetscii::Bytes<F>,
) -> io::Result<Option<u8>> {
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            match needle.find(available) {
                Some(i) => (Some(available[i]), i + 1),
                None => (None, available.len()),
            }
        };
        r.consume(used);
        if done.is_some() || used == 0 {
            return Ok(done);
        }
    }
}
