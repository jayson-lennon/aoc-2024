long-bench DAY TIME:
  cargo bench --bench aoc -- {{DAY}} --measurement-time {{TIME}}

bench DAY="":
  cargo bench --bench aoc -- {{DAY}}

watch DAY="":
  watchexec --clear --debounce 200ms cargo nextest run {{DAY}}

pgo DAY="":
  cargo pgo bench {{DAY}}
  cargo pgo optimize bench {{DAY}}
