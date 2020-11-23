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

fn path_typ(p: &mut Parser) {
    assert!(p.at(Ident));
    let c = p.checkpoint();
    p.bump(Ident);
    while p.at(Dot) {
        p.bump(Dot);
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

fn typ_nullary(p: &mut Parser) {
    match p.peek() {
        LBracket => array_typ(p),
        Ident => {
            let c = p.checkpoint();
            path_typ(p);
            opt_typ_args(p);
            p.finish_at(c, PathT)

        },
        _ => unreachable!()
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
    typ_un(p)
}
