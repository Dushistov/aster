use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ptr::P;

use invoke::{Invoke, Identity};

use str::ToInternedString;

//////////////////////////////////////////////////////////////////////////////

pub struct LitBuilder<F=Identity> {
    callback: F,
    span: Span,
}

impl LitBuilder {
    pub fn builder() -> LitBuilder {
        LitBuilder::new_with_callback(Identity)
    }
}

impl<F> LitBuilder<F>
    where F: Invoke<P<ast::Lit>>,
{
    pub fn new_with_callback(callback: F) -> Self {
        LitBuilder {
            callback: callback,
            span: DUMMY_SP,
        }
    }

    pub fn span(mut self, span: Span) -> LitBuilder<F> {
        self.span = span;
        self
    }

    pub fn build_lit(self, lit: ast::Lit_) -> F::Result {
        self.callback.invoke(P(ast::Lit {
            span: self.span,
            node: lit,
        }))
    }

    pub fn bool(self, value: bool) -> F::Result {
        self.build_lit(ast::LitBool(value))
    }

    pub fn int(self, value: i64, ty: ast::IntTy) -> F::Result {
        let sign = ast::Sign::new(value);
        self.build_lit(ast::LitInt(value as u64, ast::LitIntType::SignedIntLit(ty, sign)))
    }

    pub fn isize(self, value: isize) -> F::Result {
        self.int(value as i64, ast::IntTy::TyIs(false))
    }

    pub fn i8(self, value: i8) -> F::Result {
        self.int(value as i64, ast::IntTy::TyI8)
    }

    pub fn i16(self, value: i16) -> F::Result {
        self.int(value as i64, ast::IntTy::TyI16)
    }

    pub fn i32(self, value: i32) -> F::Result {
        self.int(value as i64, ast::IntTy::TyI32)
    }

    pub fn i64(self, value: i64) -> F::Result {
        self.int(value, ast::IntTy::TyI64)
    }

    pub fn uint(self, value: u64, ty: ast::UintTy) -> F::Result {
        self.build_lit(ast::LitInt(value, ast::LitIntType::UnsignedIntLit(ty)))
    }

    pub fn usize(self, value: usize) -> F::Result {
        self.uint(value as u64, ast::UintTy::TyUs(false))
    }

    pub fn u8(self, value: u8) -> F::Result {
        self.uint(value as u64, ast::UintTy::TyU8)
    }

    pub fn u16(self, value: u16) -> F::Result {
        self.uint(value as u64, ast::UintTy::TyU16)
    }

    pub fn u32(self, value: u32) -> F::Result {
        self.uint(value as u64, ast::UintTy::TyU32)
    }

    pub fn u64(self, value: u64) -> F::Result {
        self.uint(value, ast::UintTy::TyU64)
    }

    pub fn str<S>(self, value: S) -> F::Result
        where S: ToInternedString,
    {
        let value = value.into_interned_string();
        self.build_lit(ast::LitStr(value, ast::CookedStr))
    }
}
