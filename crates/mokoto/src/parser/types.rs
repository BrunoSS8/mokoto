use super::*;

fn opt_mutability(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(VarKw) {
        p.finish_at(c, MutModifier)
    }
}

fn opt_typ_args(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(Lt) {
        typ(p);
        while p.eat(Comma) {
            typ(p);
        }
        if !p.eat(Gt) {
            // TODO Error
        }
        p.finish_at(c, TypArgs)
    }
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
    match p.peek() {
        LParen => paren_or_tuple_typ(p),
        LBracket => array_typ(p),
        Ident => {
            let c = p.checkpoint();
            path(p);
            opt_typ_args(p);
            p.finish_at(c, PathT)
        }
        _ => unreachable!("What"),
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
    typ_un(p);
    if p.eat(Arrow) {
        typ(p);
        p.finish_at(c, FuncT);
    }
}
