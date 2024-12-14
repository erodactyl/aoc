open Game

let rec parse filename =
  let file_contents = In_channel.with_open_text filename In_channel.input_all in
  let lines = String.split_on_char '\n' file_contents in
  let games = parse_games lines in
  List.rev games

and parse_games lines =
  let rec parse_games lines games =
    match lines with
    | button_a :: button_b :: prize :: _ :: rest ->
        let ax, ay = parse_line button_a in
        let bx, by = parse_line button_b in
        let px, py = parse_line prize in
        let game =
          {
            dest = (px + 10000000000000, py + 10000000000000);
            curr = (0, 0);
            da = (ax, ay);
            db = (bx, by);
            a_count = 0;
            b_count = 0;
          }
        in
        parse_games rest (game :: games)
    | _ -> games
  in
  parse_games lines []

and parse_line line =
  let data =
    match String.split_on_char ':' line with
    | _ :: data :: _ -> data
    | _ -> failwith "can't parse button"
  in

  let x, y =
    match String.split_on_char ',' data with
    | x :: y :: _ -> (x, y)
    | _ -> failwith "can't parse button"
  in

  let x = String.sub x 3 (String.length x - 3) |> int_of_string in
  let y = String.sub y 3 (String.length y - 3) |> int_of_string in
  (x, y)
