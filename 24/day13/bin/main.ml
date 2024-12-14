open Parse
open Game

let move (x, y) (dx, dy) = (x + dx, y + dy)
let diff (x, y) (curr_x, curr_y) = (x - curr_x, y - curr_y)
let get_game_cost { a_count; b_count; _ } = (a_count * 3) + b_count

let rec calculate_cost
    ({ dest = (dest_x, dest_y) as dest; db = dbx, dby; da = dax, day; _ } as
     game) =
  let game = fill_with_bs game in
  let game = add_as game in

  print_endline "\nFilled\n\n";

  let rec calculate_cost
      ({ curr = (curr_x, curr_y) as curr; a_count; b_count; _ } as game) =
    print_game game;
    if dest = curr then Some (get_game_cost game)
    else if curr_x > dest_x || curr_y > dest_y then
      if b_count = 0 then None
      else
        let curr = move curr (-dbx, -dby) in
        calculate_cost { game with curr; b_count = b_count - 1 }
    else
      let curr = move curr (dax, day) in
      calculate_cost { game with curr; a_count = a_count + 1 }
  in
  match game with None -> None | Some game -> calculate_cost game

and fill_with_bs ({ dest = dest_x, dest_y; db = dbx, dby; _ } as game) =
  (* fill with only bs *)
  let bxs = dest_x / dbx in
  let bys = dest_y / dby in
  let b_count = if bxs < bys then bxs + 1 else bys + 1 in
  let curr = (b_count * dbx, b_count * dby) in
  { game with curr; b_count }

and add_as ({ dest; curr; da = dax, day; _ } as game) =
  print_game game;
  if dest = curr then Some game
  else
    let diff_x, diff_y = diff dest curr in
    let a_add = if diff_x > 0 then diff_x / dax else diff_y / day in
    if a_add = 0 then Some game
    else
      (* fill with as until it's almost gone over on both axis *)
      let curr = move game.curr (a_add * dax, a_add * day) in
      remove_bs { game with curr; a_count = game.a_count + a_add }

and remove_bs ({ dest; curr; db = dbx, dby; _ } as game) =
  print_game game;
  if dest = curr then Some game
  else
    let diff_x, diff_y = diff dest curr in
    let b_remove = if diff_x < 0 then -diff_x / dbx else -diff_y / dby in
    if b_remove = 0 then Some game
    else if b_remove > game.b_count then None
    else
      let curr = move game.curr (b_remove * -dbx, b_remove * -dby) in
      add_as { game with curr; b_count = game.b_count - b_remove }

let sum_costs acc game =
  acc
  +
  match calculate_cost game with
  | None ->
      print_endline "No solution";
      0
  | Some cost ->
      Printf.printf "Cost: %d\n" cost;
      cost

let () =
  let games = parse "input.txt" in
  match games with
  | _ :: _ :: _ :: game :: _ ->
      let _ = calculate_cost game in
      ()
  | _ -> failwith "Invalid"
(*let cost = List.fold_left sum_costs 0 games in*)
(*print_endline (string_of_int cost)*)
