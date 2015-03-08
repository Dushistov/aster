use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span, respan};
use syntax::ptr::P;

use invoke::{Invoke, Identity};

use expr::ExprBuilder;
use ident::ToIdent;
use item::ItemBuilder;
use pat::PatBuilder;
use ty::TyBuilder;

//////////////////////////////////////////////////////////////////////////////

pub struct StmtBuilder<F=Identity> {
    callback: F,
    span: Span,
}

impl StmtBuilder {
    pub fn new() -> StmtBuilder {
        StmtBuilder::new_with_callback(Identity)
    }
}

impl<F> StmtBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    pub fn new_with_callback(callback: F) -> Self {
        StmtBuilder {
            callback: callback,
            span: DUMMY_SP,
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn build_stmt(self, stmt_: ast::Stmt_) -> F::Result {
        self.callback.invoke(P(respan(self.span, stmt_)))
    }

    pub fn build_let(self,
                 pat: P<ast::Pat>,
                 ty: Option<P<ast::Ty>>,
                 init: Option<P<ast::Expr>>) -> F::Result {
        let local = ast::Local {
            pat: pat,
            ty: ty,
            init: init,
            id: ast::DUMMY_NODE_ID,
            span: self.span,
            source: ast::LocalSource::LocalLet,
        };

        let decl = respan(self.span, ast::Decl_::DeclLocal(P(local)));

        self.build_stmt(ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID))
    }

    pub fn let_(self) -> PatBuilder<Self> {
        PatBuilder::new_with_callback(self)
    }

    pub fn let_id<I>(self, id: I) -> ExprBuilder<StmtLetIdBuilder<F>>
        where I: ToIdent,
    {
        let id = id.to_ident();
        ExprBuilder::new_with_callback(StmtLetIdBuilder(self, id))
    }

    pub fn build_expr(self, expr: P<ast::Expr>) -> F::Result {
        self.build_stmt(ast::Stmt_::StmtExpr(expr, ast::DUMMY_NODE_ID))
    }

    pub fn expr(self) -> ExprBuilder<StmtExprBuilder<F>> {
        ExprBuilder::new_with_callback(StmtExprBuilder(self))
    }

    pub fn semi(self) -> ExprBuilder<StmtSemiBuilder<F>> {
        ExprBuilder::new_with_callback(StmtSemiBuilder(self))
    }

    pub fn build_item(self, item: P<ast::Item>) -> F::Result {
        let decl = respan(self.span, ast::Decl_::DeclItem(item));
        self.build_stmt(ast::StmtDecl(P(decl), ast::DUMMY_NODE_ID))
    }

    pub fn item(self) -> ItemBuilder<StmtItemBuilder<F>> {
        ItemBuilder::new_with_callback(StmtItemBuilder(self))
    }
}

impl<F> Invoke<P<ast::Pat>> for StmtBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = StmtLetBuilder<F>;

    fn invoke(self, pat: P<ast::Pat>) -> StmtLetBuilder<F> {
        StmtLetBuilder {
            builder: self,
            pat: pat,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtLetIdBuilder<F>(StmtBuilder<F>, ast::Ident);

impl<F> Invoke<P<ast::Expr>> for StmtLetIdBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.0.let_().id(self.1).build_expr(expr)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtExprBuilder<F>(StmtBuilder<F>);

impl<F> Invoke<P<ast::Expr>> for StmtExprBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.0.build_expr(expr)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtSemiBuilder<F>(StmtBuilder<F>);

impl<F> Invoke<P<ast::Expr>> for StmtSemiBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.0.build_stmt(ast::Stmt_::StmtSemi(expr, ast::DUMMY_NODE_ID))
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtLetBuilder<F> {
    builder: StmtBuilder<F>,
    pat: P<ast::Pat>,
}

impl<F> StmtLetBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    fn build_ty(self, ty: P<ast::Ty>) -> StmtLetTyBuilder<F> {
        StmtLetTyBuilder {
            builder: self.builder,
            pat: self.pat,
            ty: ty,
        }
    }

    pub fn ty(self) -> TyBuilder<Self> {
        TyBuilder::new_with_callback(self)
    }

    pub fn build_expr(self, expr: P<ast::Expr>) -> F::Result {
        self.builder.build_let(self.pat, None, Some(expr))
    }

    pub fn expr(self) -> ExprBuilder<Self> {
        ExprBuilder::new_with_callback(self)
    }

    pub fn build(self) -> F::Result {
        self.builder.build_let(self.pat, None, None)
    }
}

impl<F> Invoke<P<ast::Ty>> for StmtLetBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = StmtLetTyBuilder<F>;

    fn invoke(self, ty: P<ast::Ty>) -> StmtLetTyBuilder<F> {
        self.build_ty(ty)
    }
}

impl<F> Invoke<P<ast::Expr>> for StmtLetBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.build_expr(expr)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtLetTyBuilder<F> {
    builder: StmtBuilder<F>,
    pat: P<ast::Pat>,
    ty: P<ast::Ty>,
}

impl<F> StmtLetTyBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    pub fn expr(self) -> ExprBuilder<Self> {
        ExprBuilder::new_with_callback(self)
    }

    pub fn build(self) -> F::Result {
        self.builder.build_let(self.pat, Some(self.ty), None)
    }
}

impl<F> Invoke<P<ast::Expr>> for StmtLetTyBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.builder.build_let(self.pat, Some(self.ty), Some(expr))
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct StmtItemBuilder<F>(StmtBuilder<F>);

impl<F> Invoke<P<ast::Item>> for StmtItemBuilder<F>
    where F: Invoke<P<ast::Stmt>>,
{
    type Result = F::Result;

    fn invoke(self, item: P<ast::Item>) -> F::Result {
        self.0.build_item(item)
    }
}

