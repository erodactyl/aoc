type game = {
  dest : int * int;
  curr : int * int;
  da : int * int;
  db : int * int;
  b_count : int;
  a_count : int;
}

let print_game game =
  Printf.printf "dest: %d, %d\n" (fst game.dest) (snd game.dest);
  Printf.printf "curr: %d, %d\n" (fst game.curr) (snd game.curr);
  Printf.printf "b_count: %d\n" game.b_count;
  Printf.printf "a_count: %d\n\n" game.a_count;
  flush stdout
