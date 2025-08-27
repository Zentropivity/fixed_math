#!/usr/bin/nu

use _wolfram *

const regex_big = '([0-9]+\.?[0-9]+)×10\^([-+]?[0-9]+)'
const regex_small = '([0-9]+\.?[0-9]+)'

def query_calculation (i: int) {
  print -n "."
  let res = wolfram $"N[atan\(2^[-($i)]\) * 180 / PI, 42]"

  let res = ($res.pods | where title == "Result")
  let res = ($res.subpods.0.plaintext)

  let parsed = $res | parse --regex $regex_big

  mut parsed_str = ""
  if ($parsed | length) == 0 {
    $parsed_str = $res | parse --regex $regex_small | get 0.capture0
  } else {
    let parts = $parsed | get 0
    $parsed_str = $"($parts.capture0)e($parts.capture1)"
  }
  return $parsed_str
}

let results = (0..128 | each { query_calculation $in })

print "\n"

return ($results | str join "\n")
