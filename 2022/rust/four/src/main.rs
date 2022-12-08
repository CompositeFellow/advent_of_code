
fn main (){
  let mut p1_total:u32 = 0;
  let input: Vec<&str> = include_str!("input.txt").lines().collect();
  let x: Vec<Vec<&str>> = input.into_iter().map(|l| l.split(",").collect()).collect();
  let y: Vec<Vec<Vec<&str>>>= x.iter()
    .map(|l| {
      l.iter().map(|n| n.split("-").collect()).collect()
    })
    .collect();

  for group in y{
    let a1:u32 = group[0][0].parse().unwrap();
    let a2:u32 = group[0][1].parse().unwrap();
    let b1:u32 = group[1][0].parse().unwrap();
    let b2:u32 = group[1][1].parse().unwrap();
    
    // verifies if one group is in range of the other 
    if a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2{
      p1_total += 1;
    }
  }
  
  println!("Part 1 sum: {}", p1_total);

}
