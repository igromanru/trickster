extern crate trickster;
use trickster::external::{Process, RegionPermissions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // In order to use parse_maps() it needs to be mutable.
  let mut ctx = Process::new("heap_addr_ex")?;
  ctx.parse_maps()?;

  // Find first occurence of region with name equal
  // to `[heap]` and permissions equal to `rw-p`.
  // This function can also be used with second parameter == None.
  // It will do the same but ignore permissions search filter.
  let heap_region = ctx.region_find_first_by_name(
    "[heap]",
    Some(RegionPermissions {
      readable: true,
      writeable: true,
      executable: false,
      shared: false,
    }),
  )?;

  println!(
    "heap (start -> end): (0x{:x} -> 0x{:x})",
    heap_region.start, heap_region.end
  );
  
  Ok(())
}
