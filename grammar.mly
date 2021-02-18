%{
open Mo_def
open Mo_types
open Mo_values

open Syntax
open Source
open Operator
open Parser_lib


(* Position handling *)

let position_to_pos position =
  (* TBR: Remove assertion once the menhir bug is fixed. *)
  assert (Obj.is_block (Obj.repr position));
  { file = position.Lexing.pos_fname;
    line = position.Lexing.pos_lnum;
    column = position.Lexing.pos_cnum - position.Lexing.pos_bol
  }

let positions_to_region position1 position2 =
  { left = position_to_pos position1;
    right = position_to_pos position2
  }

let at (startpos, endpos) = positions_to_region startpos endpos

let warn_deprecated_obj category at =
  Diag.add_msg (Option.get !msg_store) Diag.{
    sev = Warning;
    cat = "syntax";
    at;
    text = "object syntax is deprecated in this position, use "
      ^ (if category = `Exp then "'{ {...} }'" else "'({...})'")
  }

let warn_deprecated_block at =
  Diag.add_msg (Option.get !msg_store) Diag.{
    sev = Warning;
    cat = "syntax";
    at;
    text = "block syntax is deprecated in this position, use 'do { ... }'"
  }


(* Helpers *)

let scope_bind x at =
  { var = Type.scope_var x @@ at;
    sort = Type.Scope @@ at;
    bound = PrimT "Any" @! at
  } @= at

let ensure_scope_bind var tbs =
  match tbs with
  | tb::_ when tb.it.sort.it = Type.Scope -> tbs
  | _ -> scope_bind var no_region :: tbs

let ensure_async_typ t_opt =
  match t_opt with
  | None -> t_opt
  | Some { it = AsyncT _; _} -> t_opt
  | Some t -> Some (AsyncT(scopeT no_region, t) @! no_region)

let funcT (sort, tbs, t1, t2) =
  match sort.it, t2.it with
  | Type.Local, AsyncT _ -> FuncT (sort, ensure_scope_bind "" tbs, t1, t2)
  | Type.Shared _, _ -> FuncT (sort, ensure_scope_bind "" tbs, t1, t2)
  | _ -> FuncT(sort, tbs, t1, t2)


let dup_var x = VarE (x.it @@ x.at) @? x.at

let name_exp e =
  match e.it with
  | VarE x -> [], e, dup_var x
  | _ ->
    let x = anon_id "val" e.at @@ e.at in
    [LetD (VarP x @! x.at, e) @? e.at], dup_var x, dup_var x

let assign_op lhs rhs_f at =
  let ds, lhs', rhs' =
    match lhs.it with
    | VarE x -> [], lhs, dup_var x
    | DotE (e1, x) ->
      let ds, ex11, ex12 = name_exp e1 in
      ds, DotE (ex11, x) @? lhs.at, DotE (ex12, x.it @@ x.at) @? lhs.at
    | IdxE (e1, e2) ->
      let ds1, ex11, ex12 = name_exp e1 in
      let ds2, ex21, ex22 = name_exp e2 in
      ds1 @ ds2, IdxE (ex11, ex21) @? lhs.at, IdxE (ex12, ex22) @? lhs.at
    | _ ->
      name_exp lhs
  in
  let e = AssignE (lhs', rhs_f rhs') @? at in
  match ds with
  | [] -> e
  | ds -> BlockE (ds @ [ExpD e @? e.at]) @? at

let annot_exp e t_opt =
  match t_opt with
  | None -> e
  | Some t -> AnnotE(e, t) @? span t.at e.at

let annot_pat p t_opt =
  match t_opt with
  | None -> p
  | Some t -> AnnotP(p, t) @! span t.at p.at


let rec normalize_let p e =
    match p.it with
    | AnnotP(p', t) -> p', AnnotE(e, t) @? p.at
    | ParP p' -> normalize_let p' e
    | _ -> (p, e)

let let_or_exp named x e' at =
  if named
  then LetD(VarP(x) @! at, e' @? at) @? at
       (* If you change the above regions,
          modify is_sugared_func_or_module to match *)
  else ExpD(e' @? at) @? at

let field_var_dec d =
  match d.it with
  | VarD(x, e) ->
    let x' = field_id x.it @@ x.at in
    x, x', LetD(VarP(x'.it @@ x'.at) @! x'.at, e) @? d.at
  | _ -> assert false

let field d = {dec = d; vis = Public @@ d.at; stab = None}
let let_field x x' = field (LetD(VarP(x) @! x.at, VarE(x') @? x'.at) @? x.at)
let var_field x x' = field (VarD(x, VarE(x') @? x'.at) @? x.at)

let obj ds' ds at =
  let e' = ObjE(Type.Object @@ at, ds) in
  if ds' = [] then e' else BlockE(ds' @ [ExpD(e' @? at) @? at])

let is_sugared_func_or_module dec = match dec.it with
  | LetD({it = VarP _; _} as pat, exp) ->
    dec.at = pat.at && pat.at = exp.at &&
    (match exp.it with
    | ObjE(sort, _) ->
      sort.it = Type.Module
    | FuncE _ ->
      true
    | _ -> false
    )
  | _ -> false


let func_exp f s tbs p t_opt is_sugar e =
  match s.it, t_opt, e with
  | Type.Local, Some {it = AsyncT _; _}, {it = AsyncE _; _}
  | Type.Shared _, _, _ ->
    FuncE(f, s, ensure_scope_bind "" tbs, p, t_opt, is_sugar, e)
  | _ ->
    FuncE(f, s, tbs, p, t_opt, is_sugar, e)

let desugar_func_body sp x t_opt (is_sugar, e) =
  if not is_sugar then
    false, e (* body declared as EQ e *)
  else (* body declared as immediate block *)
    match sp.it, t_opt with
    | _, Some {it = AsyncT _; _} ->
      true, asyncE (scope_bind x.it e.at) e
    | Type.Shared _, (None | Some { it = TupT []; _}) ->
      true, ignore_asyncE (scope_bind x.it e.at) e
    | _, _ -> (true, e)


let share_typ t =
  match t.it with
  | FuncT ({it = Type.Local; _} as s, tbs, t1, t2) ->
    { t with it = funcT ({s with it = Type.Shared Type.Write}, tbs, t1, t2)}
  | _ -> t

let share_typfield (tf : typ_field) =
  {tf with it = {tf.it with typ = share_typ tf.it.typ}}

let share_exp e =
  match e.it with
  | FuncE (x, ({it = Type.Local; _} as sp), tbs, p,
    ((None | Some { it = TupT []; _ }) as t_opt), true, e) ->
    func_exp x {sp with it = Type.Shared (Type.Write, WildP @! sp.at)} tbs p t_opt true (ignore_asyncE (scope_bind x e.at) e) @? e.at
  | FuncE (x, ({it = Type.Local; _} as sp), tbs, p, t_opt, s, e) ->
    func_exp x {sp with it = Type.Shared (Type.Write, WildP @! sp.at)} tbs p t_opt s e @? e.at
  | _ -> e

let share_dec d =
  match d.it with
  | LetD (p, e) -> LetD (p, share_exp e) @? d.at
  | _ -> d

let share_stab stab_opt dec =
  match stab_opt with
  | None ->
    (match dec.it with
     | VarD _
     | LetD _ ->
       Some (Flexible @@ dec.at)
     | _ -> None)
  | _ -> stab_opt

let share_expfield (ef : exp_field) =
  if ef.it.vis.it = Public
  then
    {ef with it = {ef.it with
      dec = share_dec ef.it.dec;
      stab = share_stab ef.it.stab ef.it.dec}}
  else
    if is_sugared_func_or_module (ef.it.dec) then
      {ef with it =
        {ef.it with stab =
          match ef.it.stab with
          | None -> Some (Flexible @@ ef.it.dec.at)
          | some -> some}
      }
    else ef

%}

%token EOF

%token LET VAR
%token LPAR RPAR LBRACKET RBRACKET LCURLY RCURLY
%token AWAIT ASYNC BREAK CASE CATCH CONTINUE DO LABEL DEBUG
%token IF IGNORE IN ELSE SWITCH LOOP WHILE FOR RETURN TRY THROW
%token ARROW ASSIGN
%token FUNC TYPE OBJECT ACTOR CLASS PUBLIC PRIVATE SHARED SYSTEM QUERY
%token SEMICOLON SEMICOLON_EOL COMMA COLON SUB DOT QUEST
%token AND OR NOT
%token IMPORT MODULE
%token DEBUG_SHOW
%token ASSERT
%token ADDOP SUBOP MULOP DIVOP MODOP POWOP
%token ANDOP OROP XOROP SHLOP USHROP SSHROP ROTLOP ROTROP
%token EQOP NEQOP LEOP LTOP GTOP GEOP
%token HASH
%token EQ LT GT
%token PLUSASSIGN MINUSASSIGN MULASSIGN DIVASSIGN MODASSIGN POWASSIGN CATASSIGN
%token ANDASSIGN ORASSIGN XORASSIGN SHLASSIGN USHRASSIGN SSHRASSIGN ROTLASSIGN ROTRASSIGN
%token NULL
%token FLEXIBLE STABLE
%token<string> DOT_NUM
%token<string> NAT
%token<string> FLOAT
%token<Mo_values.Value.unicode> CHAR
%token<bool> BOOL
%token<string> ID
%token<string> TEXT
%token PRIM
%token UNDERSCORE

%nonassoc RETURN_NO_ARG IF_NO_ELSE LOOP_NO_WHILE
%nonassoc ELSE WHILE

%left COLON
%left OR
%left AND
%nonassoc EQOP NEQOP LEOP LTOP GTOP GEOP
%left ADDOP SUBOP HASH
%left MULOP DIVOP MODOP
%left OROP
%left ANDOP
%left XOROP
%nonassoc SHLOP USHROP SSHROP ROTLOP ROTROP
%left POWOP

%type<Mo_def.Syntax.exp> exp(ob) exp_nullary(ob) exp_plain exp_obj exp_nest deprecated_exp_obj deprecated_exp_block
%type<Mo_def.Syntax.typ_item> typ_item
%type<Mo_def.Syntax.typ> typ_un typ_nullary typ typ_pre
%type<Mo_def.Syntax.vis> vis
%type<Mo_def.Syntax.typ_tag> typ_tag
%type<Mo_def.Syntax.typ_tag list> typ_variant
%type<Mo_def.Syntax.typ_field> typ_field
%type<Mo_def.Syntax.typ_bind> typ_bind
%type<Mo_def.Syntax.typ list> typ_args
%type<Source.region -> Mo_def.Syntax.pat> pat_opt
%type<Mo_def.Syntax.typ_tag list> seplist1(typ_tag,semicolon) seplist(typ_tag,semicolon)
%type<Mo_def.Syntax.typ_item list> seplist(typ_item,COMMA)
%type<Mo_def.Syntax.typ_field list> typ_obj seplist(typ_field,semicolon)
%type<Mo_def.Syntax.typ_bind list> seplist(typ_bind,COMMA)
%type<Mo_def.Syntax.typ list> seplist(typ,COMMA)
%type<Mo_def.Syntax.pat_field list> seplist(pat_field,semicolon)
%type<Mo_def.Syntax.pat list> seplist(pat_bin,COMMA)
%type<Mo_def.Syntax.dec list> seplist(imp,semicolon) seplist(imp,SEMICOLON) seplist(dec_var,semicolon) seplist(dec,semicolon) seplist(dec,SEMICOLON)
%type<Mo_def.Syntax.exp list> seplist(exp_nonvar(ob),COMMA) seplist(exp(ob),COMMA)
%type<(Mo_def.Syntax.dec * Mo_def.Syntax.exp_field) list> seplist(exp_field,semicolon)
%type<Mo_def.Syntax.exp_field list> seplist(dec_field,semicolon) obj_body
%type<Mo_def.Syntax.dec list * Mo_def.Syntax.exp_field list> deprecated_exp_field_list_unamb
%type<Mo_def.Syntax.case list> seplist(case,semicolon)
%type<Mo_def.Syntax.typ option> annot_opt
%type<Mo_def.Syntax.path> path
%type<Mo_def.Syntax.pat> pat pat_un pat_plain pat_nullary pat_bin deprecated_pat_opt
%type<Mo_def.Syntax.pat_field> pat_field
%type<Mo_def.Syntax.typ list option> option(typ_args)
%type<Mo_def.Syntax.exp option> option(exp_nullary(ob))
%type<unit option> option(EQ)
%type<Mo_def.Syntax.exp> exp_un(ob) exp_un(bl) exp_post(ob) exp_post(bl) exp_nullary(bl) exp_nonvar(ob) exp_nonvar(bl) exp_nondec(ob) exp_nondec(bl) block exp_bin(ob) exp_bin(bl) exp(bl)
%type<bool * Mo_def.Syntax.exp> func_body
%type<Mo_def.Syntax.lit> lit
%type<Mo_def.Syntax.dec> dec imp dec_var dec_nonvar
%type<Mo_def.Syntax.dec * Mo_def.Syntax.exp_field> exp_field exp_field_nonvar
%type<Mo_def.Syntax.exp_field> dec_field
%type<Mo_def.Syntax.dec list> deprecated_dec_list_unamb
%type<Mo_def.Syntax.id * Mo_def.Syntax.exp_field list> class_body
%type<Mo_def.Syntax.case> catch case
%type<Mo_def.Syntax.exp> bl ob
%type<Mo_def.Syntax.dec list> import_list
%type<Mo_def.Syntax.inst> inst
%type<Mo_def.Syntax.stab option> stab

%type<unit> start
%start<string -> Mo_def.Syntax.prog> parse_prog
%start<string -> Mo_def.Syntax.prog> parse_prog_interactive
%start<unit> parse_module_header (* Result passed via the Parser_lib.Imports exception *)

%on_error_reduce exp_bin(ob) exp_bin(bl) exp_nondec(bl) exp_nondec(ob)
%%


(* Helpers *)

seplist(X, SEP) :
  | (* empty *) { [] }
  | x=X { [x] }
  | x=X SEP xs=seplist(X, SEP) { x::xs }

seplist1(X, SEP) :
  | x=X { [x] }
  | x=X SEP xs=seplist(X, SEP) { x::xs }


(* Basics *)

%inline semicolon :
  | SEMICOLON
  | SEMICOLON_EOL { () }

%inline id :
  | id=ID { id @@ at $sloc }

%inline typ_id :
  | id=ID { id @= at $sloc }

%inline id_opt :
  | id=id { fun _ _ -> true, id }
  | (* empty *) { fun sort sloc -> false, anon_id sort (at sloc) @@ at sloc }

%inline typ_id_opt :
  | id=typ_id { fun _ _ -> id }
  | (* empty *) { fun sort sloc -> anon_id sort (at sloc) @= at sloc }

%inline var_opt :
  | (* empty *) { Const @@ no_region }
  | VAR { Var @@ at $sloc }

%inline obj_sort :
  | OBJECT { Type.Object @@ at $sloc }
  | ACTOR { Type.Actor @@ at $sloc }
  | MODULE { Type.Module @@ at $sloc }

%inline obj_sort_opt :
  | (* empty *) { Type.Object @@ no_region }
  | s=obj_sort { s }

%inline mode_opt :
  | (* empty *) { Type.Write }
  | QUERY { Type.Query }

%inline func_sort_opt :
  | (* empty *) { Type.Local @@ no_region }
  | SHARED m=mode_opt { Type.Shared m @@ at $sloc }
  | QUERY { Type.Shared Type.Query @@ at $sloc }

%inline shared_pat_opt :
  | (* empty *) { Type.Local @@ no_region }
  | SHARED m=mode_opt op=pat_opt { Type.Shared (m, op (at $sloc)) @@ at $sloc  }
  | QUERY op=pat_opt { Type.Shared (Type.Query, op (at $sloc)) @@ at $sloc }


(* Paths *)

path :
  | x=id
    { IdH x @! at $sloc }
  | p=path DOT x=id
    { DotH (p, x) @! at $sloc }


(* Types *)

typ_obj :
  | LCURLY tfs=seplist(typ_field, semicolon) RCURLY
    { tfs }

typ_variant :
  | LCURLY HASH RCURLY
    { [] }
  | LCURLY tfs=seplist1(typ_tag, semicolon) RCURLY
    { tfs }

typ_nullary :
  | LPAR ts=seplist(typ_item, COMMA) RPAR
    { (match ts with
       | [(Some id, t)] -> NamedT(id, t)
       | [(None, t)] -> ParT t
       | _ -> TupT(ts)) @! at $sloc }
  | p=path tso=typ_args?
    { PathT(p, Lib.Option.get tso []) @! at $sloc }
  | LBRACKET m=var_opt t=typ RBRACKET
    { ArrayT(m, t) @! at $sloc }
  | tfs=typ_obj
    { ObjT(Type.Object @@ at $sloc, tfs) @! at $sloc }
  | tfs=typ_variant
    { VariantT tfs @! at $sloc }

typ_un :
  | t=typ_nullary
    { t }
  | QUEST t=typ_un
    { OptT(t) @! at $sloc }

typ_pre :
  | t=typ_un
    { t }
  | PRIM s=TEXT
    { PrimT(s) @! at $sloc }
  | ASYNC t=typ_pre
    { AsyncT(scopeT (at $sloc), t) @! at $sloc }
  | s=obj_sort tfs=typ_obj
    { let tfs' =
        if s.it = Type.Actor then List.map share_typfield tfs else tfs
      in ObjT(s, tfs') @! at $sloc }

typ :
  | t=typ_pre
    { t }
  | s=func_sort_opt tps=typ_params_opt t1=typ_un ARROW t2=typ
    { funcT(s, tps, t1, t2) @! at $sloc }

typ_item :
  | i=id COLON t=typ { Some i, t }
  | t=typ { None, t }

typ_args :
  | LT ts=seplist(typ, COMMA) GT { ts }

inst :
  | (* empty *)
    { { it = None; at = no_region; note = [] } }
  | LT ts=seplist(typ, COMMA) GT
    { { it = Some ts; at = at $sloc; note = [] } }


%inline typ_params_opt :
  | (* empty *) { [] }
  | LT ts=seplist(typ_bind, COMMA) GT { ts }

typ_field :
  | mut=var_opt x=id COLON t=typ
    { {id = x; typ = t; mut} @@ at $sloc }
  | x=id tps=typ_params_opt t1=typ_nullary COLON t2=typ
    { let t = funcT(Type.Local @@ no_region, tps, t1, t2)
              @! span x.at t2.at in
      {id = x; typ = t; mut = Const @@ no_region} @@ at $sloc }

typ_tag :
  | HASH x=id t=annot_opt
    { {tag = x; typ = Lib.Option.get t (TupT [] @! at $sloc)} @@ at $sloc }

typ_bind :
  | x=id SUB t=typ
    { {var = x; sort = Type.Type @@ no_region; bound = t} @= at $sloc }
  | x=id
    { {var = x; sort = Type.Type @@ no_region; bound = PrimT "Any" @! at $sloc} @= at $sloc }

annot_opt :
  | COLON t=typ { Some t }
  | (* empty *) { None }


(* Expressions *)

lit :
  | NULL { NullLit }
  | b=BOOL { BoolLit b }
  | s=NAT { PreLit (s, Type.Nat) }
  | s=FLOAT { PreLit (s, Type.Float) }
  | c=CHAR { CharLit c }
  | t=TEXT { PreLit (t, Type.Text) }

%inline unop :
  | ADDOP { PosOp }
  | SUBOP { NegOp }
  | XOROP { NotOp }

%inline binop :
  | ADDOP { AddOp }
  | SUBOP { SubOp }
  | MULOP { MulOp }
  | DIVOP { DivOp }
  | MODOP { ModOp }
  | POWOP { PowOp }
  | ANDOP { AndOp }
  | OROP  { OrOp }
  | XOROP { XorOp }
  | SHLOP { ShLOp }
  | USHROP { UShROp }
  | SSHROP { SShROp }
  | ROTLOP { RotLOp }
  | ROTROP { RotROp }
  | HASH { CatOp }

%inline relop :
  | EQOP  { EqOp }
  | NEQOP { NeqOp }
  | LTOP  { LtOp }
  | LEOP  { LeOp }
  | GTOP  { GtOp }
  | GEOP  { GeOp }

%inline unassign :
  | PLUSASSIGN { PosOp }
  | MINUSASSIGN { NegOp }
  | XORASSIGN { NotOp }

%inline binassign :
  | PLUSASSIGN { AddOp }
  | MINUSASSIGN { SubOp }
  | MULASSIGN { MulOp }
  | DIVASSIGN { DivOp }
  | MODASSIGN { ModOp }
  | POWASSIGN { PowOp }
  | ANDASSIGN { AndOp }
  | ORASSIGN { OrOp }
  | XORASSIGN { XorOp }
  | SHLASSIGN { ShLOp }
  | USHRASSIGN { UShROp }
  | SSHRASSIGN { SShROp }
  | ROTLASSIGN { RotLOp }
  | ROTRASSIGN { RotROp }
  | CATASSIGN { CatOp }


exp_obj :
(* Activate when deprecated object syntax is removed
  | LCURLY efs=seplist(exp_field, semicolon) RCURLY
    { ObjE(Type.Object @@ at $sloc, efs) @? at $sloc }
*)
  | LCURLY ds=seplist(dec_var, semicolon) RCURLY
    { let ds', ds = List.fold_right
        (fun d (ds', efs) ->
          let x, x', d' = field_var_dec d in
          d'::ds', (var_field x x' @@ at $sloc)::efs
        ) ds ([], [])
      in obj ds' ds (at $sloc) @? at $sloc }
  | LCURLY ds_efs=deprecated_exp_field_list_unamb RCURLY
    { let ds, efs = ds_efs in obj ds efs (at $sloc) @? at $sloc }

exp_plain :
  | l=lit
    { LitE(ref l) @? at $sloc }
  | LPAR es=seplist(exp(ob), COMMA) RPAR
    { match es with [e] -> e | _ -> TupE(es) @? at $sloc }

exp_nullary(B) :
  | e=B
    { e }
  | e=exp_plain
    { e }
  | x=id
    { VarE(x) @? at $sloc }
  | PRIM s=TEXT
    { PrimE(s) @? at $sloc }

exp_post(B) :
  | e=exp_nullary(B)
    { e }
  | LBRACKET m=var_opt es=seplist(exp_nonvar(ob), COMMA) RBRACKET
    { ArrayE(m, es) @? at $sloc }
  | e1=exp_post(B) LBRACKET e2=exp(ob) RBRACKET
    { IdxE(e1, e2) @? at $sloc }
  | e=exp_post(B) s=DOT_NUM
    { ProjE (e, int_of_string s) @? at $sloc }
  | e=exp_post(B) DOT x=id
    { DotE(e, x) @? at $sloc }
  | e1=exp_post(B) inst=inst e2=exp_nullary(ob)
    { CallE(e1, inst, e2) @? at $sloc }

exp_un(B) :
  | e=exp_post(B)
    { e }
  | HASH x=id
    { TagE (x, TupE([]) @? at $sloc) @? at $sloc }
  | HASH x=id e=exp_nullary(ob)
    { TagE (x, e) @? at $sloc }
  | QUEST e=exp_un(ob)
    { OptE(e) @? at $sloc }
  | op=unop e=exp_un(ob)
    { match op, e.it with
      | (PosOp | NegOp), LitE {contents = PreLit (s, Type.Nat)} ->
        let signed = match op with NegOp -> "-" ^ s | _ -> "+" ^ s in
        LitE(ref (PreLit (signed, Type.Int))) @? at $sloc
      | _ -> UnE(ref Type.Pre, op, e) @? at $sloc
    }
  | op=unassign e=exp_un(ob)
    { assign_op e (fun e' -> UnE(ref Type.Pre, op, e') @? at $sloc) (at $sloc) }
  | ACTOR e=exp_plain
    { ActorUrlE e @? at $sloc }
  | NOT e=exp_un(ob)
    { NotE e @? at $sloc }
  | DEBUG_SHOW e=exp_un(ob)
    { ShowE (ref Type.Pre, e) @? at $sloc }

exp_bin(B) :
  | e=exp_un(B)
    { e }
  | e1=exp_bin(B) op=binop e2=exp_bin(ob)
    { BinE(ref Type.Pre, e1, op, e2) @? at $sloc }
  | e1=exp_bin(B) op=relop e2=exp_bin(ob)
    { RelE(ref Type.Pre, e1, op, e2) @? at $sloc }
  | e1=exp_bin(B) AND e2=exp_bin(ob)
    { AndE(e1, e2) @? at $sloc }
  | e1=exp_bin(B) OR e2=exp_bin(ob)
    { OrE(e1, e2) @? at $sloc }
  | e=exp_bin(B) COLON t=typ
    { AnnotE(e, t) @? at $sloc }

exp_nondec(B) :
  | e=exp_bin(B)
    { e }
  | e1=exp_bin(B) ASSIGN e2=exp(ob)
    { AssignE(e1, e2) @? at $sloc}
  | e1=exp_bin(B) op=binassign e2=exp(ob)
    { assign_op e1 (fun e1' -> BinE(ref Type.Pre, e1', op, e2) @? at $sloc) (at $sloc) }
  | RETURN %prec RETURN_NO_ARG
    { RetE(TupE([]) @? at $sloc) @? at $sloc }
  | RETURN e=exp(ob)
    { RetE(e) @? at $sloc }
  | ASYNC e=exp_nest
    { AsyncE(scope_bind (anon_id "async" (at $sloc)) (at $sloc), e) @? at $sloc }
  | AWAIT e=exp_nest
    { AwaitE(e) @? at $sloc }
  | ASSERT e=exp_nest
    { AssertE(e) @? at $sloc }
  | LABEL x=id rt=annot_opt e=exp_nest
    { let x' = ("continue " ^ x.it) @@ x.at in
      let unit () = TupT [] @! at $sloc in
      let e' =
        match e.it with
        | WhileE (e1, e2) -> WhileE (e1, LabelE (x', unit (), e2) @? e2.at) @? e.at
        | LoopE (e1, eo) -> LoopE (LabelE (x', unit (), e1) @? e1.at, eo) @? e.at
        | ForE (p, e1, e2) -> ForE (p, e1, LabelE (x', unit (), e2) @? e2.at) @? e.at
        | _ -> e
      in
      LabelE(x, Lib.Option.get rt (unit ()), e') @? at $sloc }
  | BREAK x=id eo=exp_nullary(ob)?
    { let e = Lib.Option.get eo (TupE([]) @? at $sloc) in
      BreakE(x, e) @? at $sloc }
  | CONTINUE x=id
    { let x' = ("continue " ^ x.it) @@ x.at in
      BreakE(x', TupE([]) @? no_region) @? at $sloc }
  | DEBUG e=exp_nest
    { DebugE(e) @? at $sloc }
  | IF b=exp_nullary(ob) e1=exp_nest %prec IF_NO_ELSE
    { IfE(b, e1, TupE([]) @? no_region) @? at $sloc }
  | IF b=exp_nullary(ob) e1=exp_nest ELSE e2=exp_nest
    { IfE(b, e1, e2) @? at $sloc }
  | TRY e1=exp_nest c=catch
    { TryE(e1, [c]) @? at $sloc }
(* TODO: enable multi-branch TRY (already supported by compiler)
  | TRY e=exp_nest LCURLY cs=seplist(case, semicolon) RCURLY
    { TryE(e, cs) @? at $sloc }
*)
  | THROW e=exp_nest
    { ThrowE(e) @? at $sloc }
  | SWITCH e=exp_nullary(ob) LCURLY cs=seplist(case, semicolon) RCURLY
    { SwitchE(e, cs) @? at $sloc }
  | WHILE e1=exp_nullary(ob) e2=exp_nest
    { WhileE(e1, e2) @? at $sloc }
  | LOOP e=exp_nest %prec LOOP_NO_WHILE
    { LoopE(e, None) @? at $sloc }
  | LOOP e1=exp_nest WHILE e2=exp_nest
    { LoopE(e1, Some e2) @? at $sloc }
  | FOR LPAR p=pat IN e1=exp(ob) RPAR e2=exp_nest
    { ForE(p, e1, e2) @? at $sloc }
  | IGNORE e=exp_nest
    { IgnoreE(e) @? at $sloc }
  | DO e=exp_nest
    { e }

exp_nonvar(B) :
  | e=exp_nondec(B)
    { e }
  | d=dec_nonvar
    { match d.it with ExpD e -> e | _ -> BlockE([d]) @? at $sloc }

exp(B) :
  | e=exp_nonvar(B)
    { e }
  | d=dec_var
    { match d.it with ExpD e -> e | _ -> BlockE([d]) @? at $sloc }

exp_nest :
  | e=block
    { e }
  | e=exp(bl)
    { e }

block :
(* Activate once deprecated objects are removed
  | LCURLY ds=seplist(dec, semicolon) RCURLY
    { BlockE(ds) @? at $sloc }
*)
  | LCURLY ds=seplist(dec_var, semicolon) RCURLY
    { BlockE(ds) @? at $sloc }
  | LCURLY ds=deprecated_dec_list_unamb RCURLY
    { BlockE(ds) @? at $sloc }

case :
  | CASE p=pat_nullary e=exp_nest
    { {pat = p; exp = e} @@ at $sloc }

catch :
  | CATCH p=pat_nullary e=exp_nest
    { {pat = p; exp = e} @@ at $sloc }

exp_field_nonvar :
  | x=id EQ e=exp(ob)
    { let x' = field_id x.it @@ x.at in
      LetD(VarP(x') @! x'.at, e) @? at $sloc, let_field x x' @@ at $sloc }
(* TODO: activate once blocks and objects are no longer ambiguous
  | x=id
    { let x' = field_id x.it @@ x.at in
      let e = VarE(x.it @@ x.at) @@ x.at in
      LetD(VarP(x') @! x'.at, e) @? at $sloc, let_field x x' @@ at $sloc }
*)

exp_field :
  | d_ef=exp_field_nonvar { d_ef }
  | d=dec_var
    { let x, x', d' = field_var_dec d in
      d', var_field x x' @@ at $sloc }

dec_field :
  | v=vis s=stab d=dec
    { {dec = d; vis = v; stab = s} @@ at $sloc }

vis :
  | (* empty *) { Private @@ no_region }
  | PRIVATE { Private @@ at $sloc }
  | PUBLIC { Public @@ at $sloc }
  | SYSTEM { System @@ at $sloc }

stab :
  | (* empty *) { None }
  | FLEXIBLE { Some (Flexible @@ at $sloc) }
  | STABLE { Some (Stable @@ at $sloc) }


(* Patterns *)

pat_plain :
  | UNDERSCORE
    { WildP @! at $sloc }
  | x=id
    { VarP(x) @! at $sloc }
  | l=lit
    { LitP(ref l) @! at $sloc }
  | LPAR ps=seplist(pat_bin, COMMA) RPAR
    { (match ps with [p] -> ParP(p) | _ -> TupP(ps)) @! at $sloc }

pat_nullary :
  | p=pat_plain
    { p }
  | LCURLY fps=seplist(pat_field, semicolon) RCURLY
    { ObjP(fps) @! at $sloc }

pat_un :
  | p=pat_nullary
    { p }
  | HASH x=id
    { TagP(x, TupP [] @! at $sloc) @! at $sloc }
  | HASH x=id p=pat_nullary
    { TagP(x, p) @! at $sloc }
  | QUEST p=pat_un
    { OptP(p) @! at $sloc }
  | op=unop l=lit
    { match op, l with
      | (PosOp | NegOp), PreLit (s, Type.Nat) ->
        let signed = match op with NegOp -> "-" ^ s | _ -> "+" ^ s in
        LitP(ref (PreLit (signed, Type.Int))) @! at $sloc
      | _ -> SignP(op, ref l) @! at $sloc
    }

pat_bin :
  | p=pat_un
    { p }
  | p1=pat_bin OR p2=pat_bin
    { AltP(p1, p2) @! at $sloc }
  | p=pat_bin COLON t=typ
    { AnnotP(p, t) @! at $sloc }

pat :
  | p=pat_bin
    { p }

pat_field :
  | x=id t=annot_opt
    { {id = x; pat = annot_pat (VarP x @! x.at) t} @@ at $sloc }
  | x=id t=annot_opt EQ p=pat
    { {id = x; pat = annot_pat p t} @@ at $sloc }

pat_opt :
  | p=pat_plain
    { fun sloc -> p }
  | (* empty *)
    { fun sloc -> WildP @! sloc }
  | p=deprecated_pat_opt
    { fun sloc -> p }



(* Declarations *)

dec_var :
  | VAR x=id t=annot_opt EQ e=exp(ob)
    { VarD(x, annot_exp e t) @? at $sloc }

dec_nonvar :
  | LET p=pat EQ e=exp(ob)
    { let p', e' = normalize_let p e in
      LetD (p', e') @? at $sloc }
  | TYPE x=typ_id tps=typ_params_opt EQ t=typ
    { TypD(x, tps, t) @? at $sloc }
  | s=obj_sort xf=id_opt EQ? efs=obj_body
    { let named, x = xf "object" $sloc in
      let e =
        if s.it = Type.Actor then
          AwaitE
            (AsyncE(scope_bind (anon_id "async" (at $sloc)) (at $sloc),
              (ObjE(s, List.map share_expfield efs) @? (at $sloc)))
             @? at $sloc)
        else ObjE(s, efs)
      in
      let_or_exp named x e (at $sloc) }
  | sp=shared_pat_opt FUNC xf=id_opt
      tps=typ_params_opt p=pat_plain t=annot_opt fb=func_body
    { (* This is a hack to support local func declarations that return a computed async.
         These should be defined using RHS syntax EQ e to avoid the implicit AsyncE introduction
         around bodies declared as blocks *)
      let named, x = xf "func" $sloc in
      let is_sugar, e = desugar_func_body sp x t fb in
      let_or_exp named x (func_exp x.it sp tps p t is_sugar e) (at $sloc) }
  | sp=shared_pat_opt s=obj_sort_opt CLASS xf=typ_id_opt
      tps=typ_params_opt p=pat_plain t=annot_opt cb=class_body
    { let x, efs = cb in
      let efs', tps', t' =
        if s.it = Type.Actor then
          (List.map share_expfield efs,
           ensure_scope_bind "" tps,
           (* Not declared async: insert AsyncT but deprecate in typing *)
           ensure_async_typ t)
        else (efs, tps, t)
      in
      ClassD(sp, xf "class" $sloc, tps', p, t', s, x, efs') @? at $sloc }

dec :
  | d=dec_var
    { d }
  | d=dec_nonvar
    { d }
  | e=exp_nondec(ob)
    { ExpD e @? at $sloc }

func_body :
  | EQ e=exp(ob) { (false, e) }
  | e=block { (true, e) }

obj_body :
  | LCURLY efs=seplist(dec_field, semicolon) RCURLY { efs }

class_body :
  | EQ xf=id_opt efs=obj_body { snd (xf "object" $sloc), efs }
  | efs=obj_body { anon_id "object" (at $sloc) @@ at $sloc, efs }


(* Programs *)

imp :
  | IMPORT xf=id_opt EQ? f=TEXT
    { let _, x = xf "import" $sloc in
      let_or_exp true x (ImportE (f, ref Unresolved)) (at $sloc) }

start : (* dummy non-terminal to satisfy ErrorReporting.ml, that requires a non-empty parse stack *)
  | (* empty *) { () }

parse_prog :
  | start is=seplist(imp, semicolon) ds=seplist(dec, semicolon) EOF
    { fun filename -> { it = is @ ds; at = at $sloc ; note = filename} }

parse_prog_interactive :
  | start is=seplist(imp, SEMICOLON) ds=seplist(dec, SEMICOLON) SEMICOLON_EOL
    { fun filename -> { it = is @ ds; at = at $sloc ; note = filename} }

import_list :
  | is=seplist(imp, semicolon) { raise (Imports is) }

parse_module_header :
  | start import_list EOF {}


(* Deprecated block/object syntax *)

(* Default {} to block or object, respectively *)
bl :
  | e=deprecated_exp_obj
    { e }

ob :
  | e=exp_obj
    { e }
  | e=deprecated_exp_block
    { e }

deprecated_pat_opt :
  | LCURLY fps=seplist(pat_field, semicolon) RCURLY
    { warn_deprecated_obj `Pat (at $sloc);
      ObjP(fps) @! at $sloc }

deprecated_exp_obj :
  | LCURLY ds_efs=deprecated_exp_field_list_unamb RCURLY
    { warn_deprecated_obj `Exp (at $sloc);
      let ds, efs = ds_efs in obj ds efs (at $sloc) @? at $sloc }

deprecated_exp_field_list_unamb :  (* does not overlap with dec_list_unamb *)
  | d_ef=exp_field_nonvar
    { let d, ef = d_ef in [d], [ef] }
  | d_ef=exp_field_nonvar semicolon d_efs=seplist(exp_field, semicolon)
    { let d, ef = d_ef and ds, efs = List.split d_efs in d::ds, ef::efs }
  | d=dec_var semicolon ds_efs=deprecated_exp_field_list_unamb
    { let x, x', d' = field_var_dec d and ds, efs = ds_efs in
      d'::ds, (var_field x x' @@ at $sloc)::efs }

deprecated_exp_block :
  | LCURLY ds=deprecated_dec_list_unamb RCURLY
    { warn_deprecated_block (at $sloc);
      BlockE(ds) @? at $sloc }

deprecated_dec_list_unamb :  (* does not overlap with exp_field_list_unamb *)
  | e=exp_nondec(ob)
    { [ExpD e @? e.at] }
  | d=dec_nonvar
    { [d] }
  | e=exp_nondec(ob) semicolon ds=seplist(dec, semicolon)
    { (ExpD e @? e.at) :: ds }
  | d=dec_nonvar semicolon ds=seplist(dec, semicolon)
    { d::ds }
  | d=dec_var semicolon ds=deprecated_dec_list_unamb
    { d::ds }

%%
