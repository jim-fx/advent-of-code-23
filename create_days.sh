#!/bin/bash
for day in {01..24}; do
  # pad i with 0
  echo $day
  if [ -f "src/day_$day.rs" ]; then
    printf "day_$day.rs already exists\n"
  else 
    touch "src/day_$day.rs"
    echo "pub fn solve(input:String) -> (u32, u32) {
  return (0, 0);
}" > src/day_$day.rs
  fi

  if [ -f "inputs/${day}_test.txt" ]; then
    printf "day_$day.txt already exists\n"
  else 
    touch "inputs/$day.txt"
    touch "inputs/${day}_test.txt"
  fi

done
