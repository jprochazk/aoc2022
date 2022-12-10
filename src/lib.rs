use std::time::Instant;

pub fn time<F, H, R>(f: F, then: H)
where
  F: FnOnce() -> R,
  H: FnOnce(R),
{
  let start = Instant::now();
  let r = f();
  let time = Instant::now() - start;
  then(r);
  println!("Done in {} microseconds\n", time.as_micros());
}
