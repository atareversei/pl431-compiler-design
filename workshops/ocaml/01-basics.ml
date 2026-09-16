let x = 1;;
(* FUNCTIONS *)
let increment x = x + 1;;
increment 0;;
increment(0);;
increment(increment 0);;


(*We can use directives in the top level REPL with `#`*)

(*loads the ml file into repl*)
(** #use "./path/to/file.ml";;*)

(*identifiers*)
let number a_'12' = 10;;

(*numbers*)
10 / 5;;
10 mod 7;;

(*floats*)
(*note that operators have a dot after them*)
3. -. 2.12;;
2.2 *. 1.2;; 

(*booleans*)
true && false;;

(*characters*)
'a';;

(*strings*)
"name";;
"firstname" ^ "lastname";; (*concatenation*)
"whatever".[0];; (*access to characters*)

(*conditionals*)
if 3 > 2 then "nice!" else "wrong";;

(*functions*)
let sum a b = a + b;;
let mul = fun a b -> a * b;;
(*recursive*)
let rec fact n = if n = 0 then 1 else n * fact(n-1);;
let rec powr x y = if y = 1 then x else x * powr x (y-1);;

(*partial application*)
let add_x x = fun y -> x + y;;

(*io*)
let age = read_int in
let name = read_line in
let weight = read_float in

print_string "whatever";
print_endline "hello world";
print_float 3.;
print_int 12;
(* these function return the type unit which is equivalent of Void in Java or None in Python. This type is used mainly for functions that have side effects. *)

Printf.printf "%s: %F\n%!" "width" 12.14;; (* the '!' is used for flushing out the buffer*)

(*lists*)
let list = [];; (*empty list called 'nil'*)
let item = 1;;
item::list;; (*prepending the element to list*)

let rec acc l = 
  match l with
  | [] -> 0
  | h::t -> h + acc t;;

let rec print_list l =
  match l with
  | [] -> ()
  | head::tail -> Printf.printf "%d " head;
    print_list tail