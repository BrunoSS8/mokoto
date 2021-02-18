use super::*;

fn opt_mutability(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(VAR_KW) {
        p.finish_at(c, MutModifier);
        return true;
    }
    false
}

fn opt_typ_params(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(L_ANGLE) {
        typ_bind(p);
        while p.eat(COMMA) {
            typ_bind(p);
        }
        if !p.eat(R_ANGLE) {
            // TODO Error
        }
        p.finish_at(c, TYPE_PARAMS);
        return true;
    }
    false
}

fn opt_typ_args(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(L_ANGLE) {
        typ(p);
        while p.eat(COMMA) {
            typ(p);
        }
        if !p.eat(R_ANGLE) {
            // TODO Error
        }
        p.finish_at(c, TYPE_ARGS);
        return true;
    }
    false
}

fn path(p: &mut Parser) {
    assert!(p.at(IDENT));
    let c = p.checkpoint();
    p.bump(IDENT);
    while p.eat(DOT) {
        // TODO: Error
        p.bump(IDENT);
    }
    p.finish_at(c, PATH)
}

fn array_typ(p: &mut Parser) {
    assert!(p.at(L_BRACKET));
    let c = p.checkpoint();
    p.bump(L_BRACKET);
    opt_mutability(p);
    typ(p);
    p.bump(R_BRACKET);
    p.finish_at(c, ARRAY_TYPE)
}

fn opt_func_sort(p: &mut Parser) -> bool {
    let c = p.checkpoint();
    if p.eat(SHARED_KW) {
        p.eat(QUERY_KW);
        p.finish_at(c, FUNC_SORT);
        true
    } else if p.eat(QUERY_KW) {
        p.finish_at(c, FUNC_SORT);
        true
    } else {
        false
    }
}

fn paren_or_tuple_typ(p: &mut Parser) {
    assert!(p.at(L_PAREN));
    let c = p.checkpoint();
    p.bump(L_PAREN);
    if p.eat(R_PAREN) {
        p.finish_at(c, TUPLE_TYPE);
        return;
    }
    typ_item(p);
    if p.eat(R_PAREN) {
        p.finish_at(c, PAREN_TYPE);
        return;
    }
    while p.eat(COMMA) {
        typ_item(p);
    }
    // TODO Error
    p.bump(R_PAREN);
    p.finish_at(c, TUPLE_TYPE)
}

pub(crate) fn opt_annot(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(COLON) {
        typ(p);
        p.finish_at(c, TYPE_ANNOTATION)
    }
}

fn typ_tag(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(HASH) {
        // TODO: error
    }
    if !p.eat(IDENT) {
        // TODO: error
    }
    opt_annot(p);
    p.finish_at(c, TYPE_TAG);
}

fn typ_field(p: &mut Parser) {
    let c = p.checkpoint();
    if opt_mutability(p) || (p.at(IDENT) && p.nth_at(1, COLON)) {
        p.bump(IDENT);
        p.bump(COLON);
        typ(p);
        p.finish_at(c, TYPE_FIELD);
    } else {
        p.bump(IDENT);
        opt_typ_params(p);
        typ_nullary(p);
        p.bump(COLON);
        typ(p);
        p.finish_at(c, TYPE_FIELD_FUNC);
    }
}

/// Does not create its own Node (so the level above can include object sorts)
fn typ_obj(p: &mut Parser) {
    if !p.eat(L_BRACE) {
        // TODO Error
        unreachable!("typ_obj")
    }
    while p.at(IDENT) || p.at(VAR_KW) {
        typ_field(p);
        p.eat(COMMA);
    }
    if !p.eat(R_BRACE) {
        // TODO: error
    }
}

fn typ_variant(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(L_BRACE) {
        // TODO Error
        unreachable!("typ_variant")
    }
    if p.at(HASH) && p.nth_at(1, R_BRACE) {
        p.bump(HASH);
        p.bump(R_BRACE);
        return p.finish_at(c, VARIANT_TYPE);
    }
    while p.at(HASH) {
        typ_tag(p);
        p.eat(COMMA);
    }
    if !p.eat(R_BRACE) {
        // TODO: error
    }
    p.finish_at(c, VARIANT_TYPE)
}

fn typ_bind(p: &mut Parser) {
    let c = p.checkpoint();
    if !p.eat(IDENT) {
        // TODO: Error
    }
    if p.eat(SUB) {
        typ(p)
    }
    p.finish_at(c, TYPE_BIND)
}

fn typ_item(p: &mut Parser) {
    let c = p.checkpoint();
    if p.at(IDENT) && p.nth_at(1, COLON) {
        p.bump(IDENT);
        p.finish_at(c, NAME);
        p.bump(COLON);
        typ(p);
        p.finish_at(c, NAMED_TYPE);
    } else if p.at(IDENT) {
        typ_nullary(p);
        if p.at(ARROW) {
            p.finish_at(c, FUNC_ARG);
            p.bump(ARROW);
            let c1 = p.checkpoint();
            typ(p);
            p.finish_at(c1, FUNC_RESULT);
            p.finish_at(c, FUNC_TYPE);
        }
    } else {
        typ(p);
    }
}

fn typ_nullary(p: &mut Parser) {
    match p.current() {
        L_PAREN => paren_or_tuple_typ(p),
        L_BRACKET => array_typ(p),
        IDENT => {
            let c = p.checkpoint();
            path(p);
            opt_typ_args(p);
            p.finish_at(c, PATH_TYPE)
        }
        L_BRACE => {
            if p.nth_at(1, HASH) {
                typ_variant(p)
            } else {
                let c = p.checkpoint();
                typ_obj(p);
                p.finish_at(c, OBJECT_TYPE)
            }
        }
        // TODO: Error
        _ => unreachable!("Unexpected token in typ_nullary"),
    }
}

fn typ_un(p: &mut Parser) {
    let c = p.checkpoint();
    if p.eat(QUESTION) {
        typ_un(p);
        p.finish_at(c, OPTIONAL_TYPE);
    } else {
        typ_nullary(p)
    }
}

fn typ_pre(p: &mut Parser) {
    match p.current() {
        ASYNC_KW => {
            let c = p.checkpoint();
            p.bump(ASYNC_KW);
            typ_pre(p);
            p.finish_at(c, ASYNC_TYPE)
        }
        // TODO: Only allow in privileged mode
        PRIM_KW => {
            let c = p.checkpoint();
            p.bump(PRIM_KW);
            p.bump(IDENT);
            p.finish_at(c, PRIM_TYPE)
        }
        OBJECT_KW | ACTOR_KW | MODULE_KW => {
            let c = p.checkpoint();
            p.bump_any();
            p.finish_at(c, OBJECT_SORT);
            typ_obj(p);
            p.finish_at(c, OBJECT_TYPE)
        }
        _ => typ_un(p),
    }
}

const STARTS_PRE: TokenSet = TokenSet::new(&[ASYNC_KW, OBJECT_KW, ACTOR_KW, MODULE_KW, PRIM_KW]);

pub(super) fn typ(p: &mut Parser) {
    let c = p.checkpoint();
    let fs = opt_func_sort(p);
    let tp = opt_typ_params(p);
    if fs || tp {
        typ_un(p);
        p.finish_at(c, FUNC_ARG);
        // TODO: Error
        p.bump(ARROW);
        let c1 = p.checkpoint();
        typ(p);
        p.finish_at(c1, FUNC_RESULT);
        p.finish_at(c, FUNC_TYPE);
    } else if STARTS_PRE.contains(p.current()) {
        typ_pre(p)
    } else {
        typ_un(p);
        if p.at(ARROW) {
            p.finish_at(c, FUNC_ARG);
            // TODO: Error
            p.bump(ARROW);
            let c1 = p.checkpoint();
            typ(p);
            p.finish_at(c1, FUNC_RESULT);
            p.finish_at(c, FUNC_TYPE);
        }
    }
}
