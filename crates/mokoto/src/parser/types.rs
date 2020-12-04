use super::*;

fn opt_mutability(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(VarKw) {
        p.finish_at(c, MutModifier);
        return true;
    }
    false
}

fn opt_typ_params(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(Lt) {
        typ_bind(p);
        while p.eat(Comma) {
            typ_bind(p);
        }
        if !p.eat(Gt) {
            // TODO Error
        }
        p.finish_at(c, TypParams);
        return true;
    }
    false
}

fn opt_typ_args(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(Lt) {
        typ(p);
        while p.eat(Comma) {
            typ(p);
        }
        if !p.eat(Gt) {
            // TODO Error
        }
        p.finish_at(c, TypArgs);
        return true;
    }
    false
}

fn path(p: &mut Parser) {
    assert!(p.at(Ident));
    let c = p.checkpoint();
    p.bump(Ident);
    while p.eat(Dot) {
        // TODO: Error
        p.bump(Ident);
    }
    p.finish_at(c, Path)
}

fn array_typ(p: &mut Parser) {
    assert!(p.at(LBracket));
    let c = p.checkpoint();
    p.bump(LBracket);
    opt_mutability(p);
    typ(p);
    p.bump(RBracket);
    p.finish_at(c, ArrayT)
}

fn opt_func_sort(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(SharedKw) {
        p.eat(QueryKw);
        p.finish_at(c, FuncSort);
        true
    } else if p.eat(QueryKw) {
        p.finish_at(c, FuncSort);
        true
    } else {
        false
    }
}

fn paren_or_tuple_typ(p: &mut Parser) {
    assert!(p.at(LParen));
    let c = p.checkpoint();
    p.bump(LParen);
    if p.eat(RParen) {
        p.finish_at(c, TupT);
        return;
    }
    typ_item(p);
    if p.eat(RParen) {
        p.finish_at(c, ParenT);
        return;
    }
    while p.eat(Comma) {
        typ_item(p);
    }
    // TODO Error
    p.bump(RParen);
    p.finish_at(c, TupT)
}

fn opt_annot(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(Colon) {
        typ(p);
        p.finish_at(c, TypAnnot)
    }
}

fn typ_tag(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(Hash) {
        // TODO: error
    }
    if !p.eat(Ident) {
        // TODO: error
    }
    opt_annot(p);
    p.finish_at(c, TypTag);
}

// typ_field :
//   | mut=var_opt x=id COLON t=typ
//     { {id = x; typ = t; mut} @@ at $sloc }
//   | x=id tps=typ_params_opt t1=typ_nullary COLON t2=typ
//     { let t = funcT(Type.Local @@ no_region, tps, t1, t2)
//               @! span x.at t2.at in
//       {id = x; typ = t; mut = Const @@ no_region} @@ at $sloc }
fn typ_field(p: &mut Parser) {
    let c = p.checkpoint();
    if opt_mutability(p) {
        p.bump(Ident);
        p.bump(Colon);
        typ(p);
        p.finish_at(c, TypField);
    } else {
        p.bump(Ident);
        if opt_typ_params(p) || !p.at(Colon) {
            typ_nullary(p);
            p.bump(Colon);
            typ(p);
            p.finish_at(c, TypFieldFunc);
        } else {
            p.bump(Colon);
            typ(p);
            p.finish_at(c, TypField);
        }
    }
}

fn typ_obj_or_variant(p: &mut Parser) {
    assert!(p.at(LBrace));
    let c = p.checkpoint();
    p.bump(LBrace);
    match p.current() {
        Hash => {
            while p.at(Hash) {
                typ_tag(p);
                p.eat(Comma);
            }
            if !p.eat(RBrace) {
                // TODO: error
            }
            p.finish_at(c, VariantT)
        }
        Ident | VarKw => {
            while p.at(Ident) || p.at(VarKw) {
                typ_field(p);
                p.eat(Comma);
            }
            if !p.eat(RBrace) {
                // TODO: error
            }
            p.finish_at(c, ObjT)
        }
        RBrace => {
            p.bump(RBrace);
            p.finish_at(c, ObjT)
        }
        _ => {
            // TODO: error
            unreachable!("Not a valid start to a object or variant")
        }
    }
}

fn typ_bind(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(Ident) {
        // TODO: Error
    }
    if p.eat(Sub) {
        typ(p)
    }
    p.finish_at(c, TypBind)
}

fn typ_item(p: &mut Parser) {
    let c = p.checkpoint();
    if p.at(Ident) && p.nth_at(1, Colon) {
        p.bump(Ident);
        p.finish_at(c, Name);
        p.bump(Colon);
        typ(p);
        p.finish_at(c, NamedT);
    } else if p.at(Ident) {
        typ_nullary(p);
        if p.eat(Arrow) {
            typ(p);
            p.finish_at(c, FuncT);
        }
    } else {
        typ(p);
    }
}

fn typ_nullary(p: &mut Parser) {
    match p.current() {
        LParen => paren_or_tuple_typ(p),
        LBracket => array_typ(p),
        Ident => {
            let c = p.checkpoint();
            path(p);
            opt_typ_args(p);
            p.finish_at(c, PathT)
        }
        LBrace => typ_obj_or_variant(p),
        // TODO: Error
        _ => unreachable!("Unexpected token in typ_nullary"),
    }
}

fn typ_un(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(Question) {
        typ_un(p);
        p.finish_at(c, OptionalT);
    } else {
        typ_nullary(p)
    }
}

pub(super) fn typ(p: &mut Parser) {
    let c = p.checkpoint();
    let fs = opt_func_sort(p);
    let tp = opt_typ_params(p);
    if fs || tp {
        typ_un(p);
        // TODO: Error
        p.bump(Arrow);
        typ(p);
        p.finish_at(c, FuncT);
    } else {
        typ_un(p);
        if p.eat(Arrow) {
            typ(p);
            p.finish_at(c, FuncT);
        }
    }
}
