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

fn typ_field(p: &mut Parser) {
    let c = p.checkpoint();
    if opt_mutability(p) || (p.at(Ident) && p.nth_at(1, Colon)) {
        p.bump(Ident);
        p.bump(Colon);
        typ(p);
        p.finish_at(c, TypField);
    } else {
        p.bump(Ident);
        opt_typ_params(p);
        typ_nullary(p);
        p.bump(Colon);
        typ(p);
        p.finish_at(c, TypFieldFunc);
    }
}

/// Does not create its own Node (so the level above can include object sorts)
fn typ_obj(p: &mut Parser) {
    if !p.eat(LBrace) {
        // TODO Error
        unreachable!("typ_obj")
    }
    while p.at(Ident) || p.at(VarKw) {
        typ_field(p);
        p.eat(Comma);
    }
    if !p.eat(RBrace) {
        // TODO: error
    }
}

fn typ_variant(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(LBrace) {
        // TODO Error
        unreachable!("typ_variant")
    }
    if p.at(Hash) && p.nth_at(1, RBrace) {
        p.bump(Hash);
        p.bump(RBrace);
        return p.finish_at(c, VariantT);
    }
    while p.at(Hash) {
        typ_tag(p);
        p.eat(Comma);
    }
    if !p.eat(RBrace) {
        // TODO: error
    }
    p.finish_at(c, VariantT)
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
        if p.at(Arrow) {
            p.finish_at(c, FuncArg);
            p.bump(Arrow);
            let c1 = p.checkpoint();
            typ(p);
            p.finish_at(c1, FuncResult);
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
        LBrace => {
            if p.nth_at(1, Hash) {
                typ_variant(p)
            } else {
                let c = p.checkpoint();
                typ_obj(p);
                p.finish_at(c, ObjT)
            }
        }
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

fn typ_pre(p: &mut Parser) {
    match p.current() {
        AsyncKw => {
            let c = p.checkpoint();
            p.bump(AsyncKw);
            typ_pre(p);
            p.finish_at(c, AsyncT)
        }
        // TODO: Only allow in privileged mode
        PrimKw => {
            let c = p.checkpoint();
            p.bump(PrimKw);
            p.bump(Ident);
            p.finish_at(c, PrimT)
        }
        ObjectKw | ActorKw | ModuleKw => {
            let c = p.checkpoint();
            p.bump_any();
            p.finish_at(c, ObjSort);
            typ_obj(p);
            p.finish_at(c, ObjT)
        }
        _ => typ_un(p),
    }
}

fn starts_pre(kind: SyntaxKind) -> bool {
    matches!(kind, AsyncKw | ObjectKw | ActorKw | ModuleKw | PrimKw)
}

pub(super) fn typ(p: &mut Parser) {
    let c = p.checkpoint();
    let fs = opt_func_sort(p);
    let tp = opt_typ_params(p);
    if fs || tp {
        typ_un(p);
        p.finish_at(c, FuncArg);
        // TODO: Error
        p.bump(Arrow);
        let c1 = p.checkpoint();
        typ(p);
        p.finish_at(c1, FuncResult);
        p.finish_at(c, FuncT);
    } else if starts_pre(p.current()) {
        typ_pre(p)
    } else {
        typ_un(p);
        if p.at(Arrow) {
            p.finish_at(c, FuncArg);
            // TODO: Error
            p.bump(Arrow);
            let c1 = p.checkpoint();
            typ(p);
            p.finish_at(c1, FuncResult);
            p.finish_at(c, FuncT);
        }
    }
}
