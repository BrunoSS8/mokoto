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

fn path_or_name(p: &mut Parser) -> Option<Checkpoint> {
    assert!(p.at(Ident));
    let c = p.checkpoint();
    p.bump(Ident);
    if !p.at(Dot) {
        return Some(c);
    }
    while p.eat(Dot) {
        // TODO: Error
        p.bump(Ident);
    }
    p.finish_at(c, Path);
    None
}

fn path(p: &mut Parser) {
    path_or_name(p).map(|c| p.finish_at(c, Path));
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

fn typ_field(p: &mut Parser) {
    let c = p.checkpoint();
    let has_mut = opt_mutability(p);
    if !p.eat(Ident) {
        // TODO: error
    }
    let typ_params = opt_typ_params(p);
    if !p.eat(Colon) {
        // TODO: error
    }
    typ(p);
    p.finish_at(c, TypField);
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
    if p.at(Ident) {
        match path_or_name(p) {
            Some(c_label) if p.at(Colon) => {
                p.finish_at(c_label, Name);
                p.bump(Colon);
                typ(p);
                p.finish_at(c, NamedT);
            }
            Some(c_path) => {
                p.finish_at(c_path, Path);
                opt_typ_args(p);
                p.finish_at(c, PathT);
                if p.eat(Arrow) {
                    typ(p);
                    p.finish_at(c, FuncT);
                }
            }
            None => {
                opt_typ_args(p);
                p.finish_at(c, PathT);
            }
        };
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
