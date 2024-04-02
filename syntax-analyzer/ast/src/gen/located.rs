// File automatically generated by ast/asdl_rs.py.

pub type Mod = crate::generic::Mod<SourceRange>;

pub type ModModule = crate::generic::ModModule<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for ModModule {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for ModModule {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ModInteractive = crate::generic::ModInteractive<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for ModInteractive {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for ModInteractive {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ModExpression = crate::generic::ModExpression<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for ModExpression {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for ModExpression {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ModFunctionType = crate::generic::ModFunctionType<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for ModFunctionType {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for ModFunctionType {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for Mod {
    fn range(&self) -> SourceRange {
        match self {
            Self::Module(node) => node.range(),
            Self::Interactive(node) => node.range(),
            Self::Expression(node) => node.range(),
            Self::FunctionType(node) => node.range(),
        }
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for Mod {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::Module(node) => node.range_mut(),
            Self::Interactive(node) => node.range_mut(),
            Self::Expression(node) => node.range_mut(),
            Self::FunctionType(node) => node.range_mut(),
        }
    }
}

pub type Stmt = crate::generic::Stmt<SourceRange>;

pub type StmtFunctionDef = crate::generic::StmtFunctionDef<SourceRange>;

impl Located for StmtFunctionDef {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtFunctionDef {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAsyncFunctionDef = crate::generic::StmtAsyncFunctionDef<SourceRange>;

impl Located for StmtAsyncFunctionDef {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAsyncFunctionDef {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtClassDef = crate::generic::StmtClassDef<SourceRange>;

impl Located for StmtClassDef {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtClassDef {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtReturn = crate::generic::StmtReturn<SourceRange>;

impl Located for StmtReturn {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtReturn {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtDelete = crate::generic::StmtDelete<SourceRange>;

impl Located for StmtDelete {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtDelete {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAssign = crate::generic::StmtAssign<SourceRange>;

impl Located for StmtAssign {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAssign {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtTypeAlias = crate::generic::StmtTypeAlias<SourceRange>;

impl Located for StmtTypeAlias {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtTypeAlias {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAugAssign = crate::generic::StmtAugAssign<SourceRange>;

impl Located for StmtAugAssign {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAugAssign {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAnnAssign = crate::generic::StmtAnnAssign<SourceRange>;

impl Located for StmtAnnAssign {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAnnAssign {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtFor = crate::generic::StmtFor<SourceRange>;

impl Located for StmtFor {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtFor {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAsyncFor = crate::generic::StmtAsyncFor<SourceRange>;

impl Located for StmtAsyncFor {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAsyncFor {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtWhile = crate::generic::StmtWhile<SourceRange>;

impl Located for StmtWhile {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtWhile {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtIf = crate::generic::StmtIf<SourceRange>;

impl Located for StmtIf {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtIf {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtWith = crate::generic::StmtWith<SourceRange>;

impl Located for StmtWith {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtWith {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAsyncWith = crate::generic::StmtAsyncWith<SourceRange>;

impl Located for StmtAsyncWith {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAsyncWith {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtMatch = crate::generic::StmtMatch<SourceRange>;

impl Located for StmtMatch {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtMatch {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtRaise = crate::generic::StmtRaise<SourceRange>;

impl Located for StmtRaise {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtRaise {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtTry = crate::generic::StmtTry<SourceRange>;

impl Located for StmtTry {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtTry {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtTryStar = crate::generic::StmtTryStar<SourceRange>;

impl Located for StmtTryStar {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtTryStar {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtAssert = crate::generic::StmtAssert<SourceRange>;

impl Located for StmtAssert {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtAssert {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtImport = crate::generic::StmtImport<SourceRange>;

impl Located for StmtImport {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtImport {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtImportFrom = crate::generic::StmtImportFrom<SourceRange>;

impl Located for StmtImportFrom {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtImportFrom {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtGlobal = crate::generic::StmtGlobal<SourceRange>;

impl Located for StmtGlobal {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtGlobal {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtNonlocal = crate::generic::StmtNonlocal<SourceRange>;

impl Located for StmtNonlocal {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtNonlocal {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtExpr = crate::generic::StmtExpr<SourceRange>;

impl Located for StmtExpr {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtExpr {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtPass = crate::generic::StmtPass<SourceRange>;

impl Located for StmtPass {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtPass {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtBreak = crate::generic::StmtBreak<SourceRange>;

impl Located for StmtBreak {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtBreak {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type StmtContinue = crate::generic::StmtContinue<SourceRange>;

impl Located for StmtContinue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for StmtContinue {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

impl Located for Stmt {
    fn range(&self) -> SourceRange {
        match self {
            Self::FunctionDef(node) => node.range(),
            Self::AsyncFunctionDef(node) => node.range(),
            Self::ClassDef(node) => node.range(),
            Self::Return(node) => node.range(),
            Self::Delete(node) => node.range(),
            Self::Assign(node) => node.range(),
            Self::TypeAlias(node) => node.range(),
            Self::AugAssign(node) => node.range(),
            Self::AnnAssign(node) => node.range(),
            Self::For(node) => node.range(),
            Self::AsyncFor(node) => node.range(),
            Self::While(node) => node.range(),
            Self::If(node) => node.range(),
            Self::With(node) => node.range(),
            Self::AsyncWith(node) => node.range(),
            Self::Match(node) => node.range(),
            Self::Raise(node) => node.range(),
            Self::Try(node) => node.range(),
            Self::TryStar(node) => node.range(),
            Self::Assert(node) => node.range(),
            Self::Import(node) => node.range(),
            Self::ImportFrom(node) => node.range(),
            Self::Global(node) => node.range(),
            Self::Nonlocal(node) => node.range(),
            Self::Expr(node) => node.range(),
            Self::Pass(node) => node.range(),
            Self::Break(node) => node.range(),
            Self::Continue(node) => node.range(),
        }
    }
}

impl LocatedMut for Stmt {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::FunctionDef(node) => node.range_mut(),
            Self::AsyncFunctionDef(node) => node.range_mut(),
            Self::ClassDef(node) => node.range_mut(),
            Self::Return(node) => node.range_mut(),
            Self::Delete(node) => node.range_mut(),
            Self::Assign(node) => node.range_mut(),
            Self::TypeAlias(node) => node.range_mut(),
            Self::AugAssign(node) => node.range_mut(),
            Self::AnnAssign(node) => node.range_mut(),
            Self::For(node) => node.range_mut(),
            Self::AsyncFor(node) => node.range_mut(),
            Self::While(node) => node.range_mut(),
            Self::If(node) => node.range_mut(),
            Self::With(node) => node.range_mut(),
            Self::AsyncWith(node) => node.range_mut(),
            Self::Match(node) => node.range_mut(),
            Self::Raise(node) => node.range_mut(),
            Self::Try(node) => node.range_mut(),
            Self::TryStar(node) => node.range_mut(),
            Self::Assert(node) => node.range_mut(),
            Self::Import(node) => node.range_mut(),
            Self::ImportFrom(node) => node.range_mut(),
            Self::Global(node) => node.range_mut(),
            Self::Nonlocal(node) => node.range_mut(),
            Self::Expr(node) => node.range_mut(),
            Self::Pass(node) => node.range_mut(),
            Self::Break(node) => node.range_mut(),
            Self::Continue(node) => node.range_mut(),
        }
    }
}

pub type Expr = crate::generic::Expr<SourceRange>;

pub type ExprBoolOp = crate::generic::ExprBoolOp<SourceRange>;

impl Located for ExprBoolOp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprBoolOp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprNamedExpr = crate::generic::ExprNamedExpr<SourceRange>;

impl Located for ExprNamedExpr {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprNamedExpr {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprBinOp = crate::generic::ExprBinOp<SourceRange>;

impl Located for ExprBinOp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprBinOp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprUnaryOp = crate::generic::ExprUnaryOp<SourceRange>;

impl Located for ExprUnaryOp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprUnaryOp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprLambda = crate::generic::ExprLambda<SourceRange>;

impl Located for ExprLambda {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprLambda {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprIfExp = crate::generic::ExprIfExp<SourceRange>;

impl Located for ExprIfExp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprIfExp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprDict = crate::generic::ExprDict<SourceRange>;

impl Located for ExprDict {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprDict {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprSet = crate::generic::ExprSet<SourceRange>;

impl Located for ExprSet {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprSet {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprListComp = crate::generic::ExprListComp<SourceRange>;

impl Located for ExprListComp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprListComp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprSetComp = crate::generic::ExprSetComp<SourceRange>;

impl Located for ExprSetComp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprSetComp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprDictComp = crate::generic::ExprDictComp<SourceRange>;

impl Located for ExprDictComp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprDictComp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprGeneratorExp = crate::generic::ExprGeneratorExp<SourceRange>;

impl Located for ExprGeneratorExp {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprGeneratorExp {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprAwait = crate::generic::ExprAwait<SourceRange>;

impl Located for ExprAwait {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprAwait {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprYield = crate::generic::ExprYield<SourceRange>;

impl Located for ExprYield {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprYield {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprYieldFrom = crate::generic::ExprYieldFrom<SourceRange>;

impl Located for ExprYieldFrom {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprYieldFrom {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprCompare = crate::generic::ExprCompare<SourceRange>;

impl Located for ExprCompare {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprCompare {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprCall = crate::generic::ExprCall<SourceRange>;

impl Located for ExprCall {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprCall {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprFormattedValue = crate::generic::ExprFormattedValue<SourceRange>;

impl Located for ExprFormattedValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprFormattedValue {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprJoinedStr = crate::generic::ExprJoinedStr<SourceRange>;

impl Located for ExprJoinedStr {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprJoinedStr {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprConstant = crate::generic::ExprConstant<SourceRange>;

impl Located for ExprConstant {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprConstant {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprAttribute = crate::generic::ExprAttribute<SourceRange>;

impl Located for ExprAttribute {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprAttribute {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprSubscript = crate::generic::ExprSubscript<SourceRange>;

impl Located for ExprSubscript {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprSubscript {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprStarred = crate::generic::ExprStarred<SourceRange>;

impl Located for ExprStarred {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprStarred {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprName = crate::generic::ExprName<SourceRange>;

impl Located for ExprName {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprName {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprList = crate::generic::ExprList<SourceRange>;

impl Located for ExprList {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprList {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprTuple = crate::generic::ExprTuple<SourceRange>;

impl Located for ExprTuple {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprTuple {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExprSlice = crate::generic::ExprSlice<SourceRange>;

impl Located for ExprSlice {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExprSlice {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

impl Located for Expr {
    fn range(&self) -> SourceRange {
        match self {
            Self::BoolOp(node) => node.range(),
            Self::NamedExpr(node) => node.range(),
            Self::BinOp(node) => node.range(),
            Self::UnaryOp(node) => node.range(),
            Self::Lambda(node) => node.range(),
            Self::IfExp(node) => node.range(),
            Self::Dict(node) => node.range(),
            Self::Set(node) => node.range(),
            Self::ListComp(node) => node.range(),
            Self::SetComp(node) => node.range(),
            Self::DictComp(node) => node.range(),
            Self::GeneratorExp(node) => node.range(),
            Self::Await(node) => node.range(),
            Self::Yield(node) => node.range(),
            Self::YieldFrom(node) => node.range(),
            Self::Compare(node) => node.range(),
            Self::Call(node) => node.range(),
            Self::FormattedValue(node) => node.range(),
            Self::JoinedStr(node) => node.range(),
            Self::Constant(node) => node.range(),
            Self::Attribute(node) => node.range(),
            Self::Subscript(node) => node.range(),
            Self::Starred(node) => node.range(),
            Self::Name(node) => node.range(),
            Self::List(node) => node.range(),
            Self::Tuple(node) => node.range(),
            Self::Slice(node) => node.range(),
        }
    }
}

impl LocatedMut for Expr {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::BoolOp(node) => node.range_mut(),
            Self::NamedExpr(node) => node.range_mut(),
            Self::BinOp(node) => node.range_mut(),
            Self::UnaryOp(node) => node.range_mut(),
            Self::Lambda(node) => node.range_mut(),
            Self::IfExp(node) => node.range_mut(),
            Self::Dict(node) => node.range_mut(),
            Self::Set(node) => node.range_mut(),
            Self::ListComp(node) => node.range_mut(),
            Self::SetComp(node) => node.range_mut(),
            Self::DictComp(node) => node.range_mut(),
            Self::GeneratorExp(node) => node.range_mut(),
            Self::Await(node) => node.range_mut(),
            Self::Yield(node) => node.range_mut(),
            Self::YieldFrom(node) => node.range_mut(),
            Self::Compare(node) => node.range_mut(),
            Self::Call(node) => node.range_mut(),
            Self::FormattedValue(node) => node.range_mut(),
            Self::JoinedStr(node) => node.range_mut(),
            Self::Constant(node) => node.range_mut(),
            Self::Attribute(node) => node.range_mut(),
            Self::Subscript(node) => node.range_mut(),
            Self::Starred(node) => node.range_mut(),
            Self::Name(node) => node.range_mut(),
            Self::List(node) => node.range_mut(),
            Self::Tuple(node) => node.range_mut(),
            Self::Slice(node) => node.range_mut(),
        }
    }
}

pub type ExprContext = crate::generic::ExprContext;

pub type ExprContextLoad = crate::generic::ExprContextLoad;

pub type ExprContextStore = crate::generic::ExprContextStore;

pub type ExprContextDel = crate::generic::ExprContextDel;

pub type BoolOp = crate::generic::BoolOp;

pub type BoolOpAnd = crate::generic::BoolOpAnd;

pub type BoolOpOr = crate::generic::BoolOpOr;

pub type Operator = crate::generic::Operator;

pub type OperatorAdd = crate::generic::OperatorAdd;

pub type OperatorSub = crate::generic::OperatorSub;

pub type OperatorMult = crate::generic::OperatorMult;

pub type OperatorMatMult = crate::generic::OperatorMatMult;

pub type OperatorDiv = crate::generic::OperatorDiv;

pub type OperatorMod = crate::generic::OperatorMod;

pub type OperatorPow = crate::generic::OperatorPow;

pub type OperatorLShift = crate::generic::OperatorLShift;

pub type OperatorRShift = crate::generic::OperatorRShift;

pub type OperatorBitOr = crate::generic::OperatorBitOr;

pub type OperatorBitXor = crate::generic::OperatorBitXor;

pub type OperatorBitAnd = crate::generic::OperatorBitAnd;

pub type OperatorFloorDiv = crate::generic::OperatorFloorDiv;

pub type UnaryOp = crate::generic::UnaryOp;

pub type UnaryOpInvert = crate::generic::UnaryOpInvert;

pub type UnaryOpNot = crate::generic::UnaryOpNot;

pub type UnaryOpUAdd = crate::generic::UnaryOpUAdd;

pub type UnaryOpUSub = crate::generic::UnaryOpUSub;

pub type CmpOp = crate::generic::CmpOp;

pub type CmpOpEq = crate::generic::CmpOpEq;

pub type CmpOpNotEq = crate::generic::CmpOpNotEq;

pub type CmpOpLt = crate::generic::CmpOpLt;

pub type CmpOpLtE = crate::generic::CmpOpLtE;

pub type CmpOpGt = crate::generic::CmpOpGt;

pub type CmpOpGtE = crate::generic::CmpOpGtE;

pub type CmpOpIs = crate::generic::CmpOpIs;

pub type CmpOpIsNot = crate::generic::CmpOpIsNot;

pub type CmpOpIn = crate::generic::CmpOpIn;

pub type CmpOpNotIn = crate::generic::CmpOpNotIn;

pub type Comprehension = crate::generic::Comprehension<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for Comprehension {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for Comprehension {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ExceptHandler = crate::generic::ExceptHandler<SourceRange>;

pub type ExceptHandlerExceptHandler = crate::generic::ExceptHandlerExceptHandler<SourceRange>;

impl Located for ExceptHandlerExceptHandler {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for ExceptHandlerExceptHandler {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

impl Located for ExceptHandler {
    fn range(&self) -> SourceRange {
        match self {
            Self::ExceptHandler(node) => node.range(),
        }
    }
}

impl LocatedMut for ExceptHandler {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::ExceptHandler(node) => node.range_mut(),
        }
    }
}

pub type PythonArguments = crate::generic::PythonArguments<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for PythonArguments {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for PythonArguments {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type Arg = crate::generic::Arg<SourceRange>;

impl Located for Arg {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for Arg {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type Keyword = crate::generic::Keyword<SourceRange>;

impl Located for Keyword {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for Keyword {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type Alias = crate::generic::Alias<SourceRange>;

impl Located for Alias {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for Alias {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type WithItem = crate::generic::WithItem<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for WithItem {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for WithItem {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type MatchCase = crate::generic::MatchCase<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for MatchCase {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for MatchCase {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type Pattern = crate::generic::Pattern<SourceRange>;

pub type PatternMatchValue = crate::generic::PatternMatchValue<SourceRange>;

impl Located for PatternMatchValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchValue {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchSingleton = crate::generic::PatternMatchSingleton<SourceRange>;

impl Located for PatternMatchSingleton {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchSingleton {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchSequence = crate::generic::PatternMatchSequence<SourceRange>;

impl Located for PatternMatchSequence {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchSequence {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchMapping = crate::generic::PatternMatchMapping<SourceRange>;

impl Located for PatternMatchMapping {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchMapping {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchClass = crate::generic::PatternMatchClass<SourceRange>;

impl Located for PatternMatchClass {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchClass {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchStar = crate::generic::PatternMatchStar<SourceRange>;

impl Located for PatternMatchStar {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchStar {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchAs = crate::generic::PatternMatchAs<SourceRange>;

impl Located for PatternMatchAs {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchAs {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type PatternMatchOr = crate::generic::PatternMatchOr<SourceRange>;

impl Located for PatternMatchOr {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for PatternMatchOr {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

impl Located for Pattern {
    fn range(&self) -> SourceRange {
        match self {
            Self::MatchValue(node) => node.range(),
            Self::MatchSingleton(node) => node.range(),
            Self::MatchSequence(node) => node.range(),
            Self::MatchMapping(node) => node.range(),
            Self::MatchClass(node) => node.range(),
            Self::MatchStar(node) => node.range(),
            Self::MatchAs(node) => node.range(),
            Self::MatchOr(node) => node.range(),
        }
    }
}

impl LocatedMut for Pattern {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::MatchValue(node) => node.range_mut(),
            Self::MatchSingleton(node) => node.range_mut(),
            Self::MatchSequence(node) => node.range_mut(),
            Self::MatchMapping(node) => node.range_mut(),
            Self::MatchClass(node) => node.range_mut(),
            Self::MatchStar(node) => node.range_mut(),
            Self::MatchAs(node) => node.range_mut(),
            Self::MatchOr(node) => node.range_mut(),
        }
    }
}

pub type TypeIgnore = crate::generic::TypeIgnore<SourceRange>;

pub type TypeIgnoreTypeIgnore = crate::generic::TypeIgnoreTypeIgnore<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for TypeIgnoreTypeIgnore {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for TypeIgnoreTypeIgnore {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for TypeIgnore {
    fn range(&self) -> SourceRange {
        match self {
            Self::TypeIgnore(node) => node.range(),
        }
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for TypeIgnore {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::TypeIgnore(node) => node.range_mut(),
        }
    }
}

pub type TypeParam = crate::generic::TypeParam<SourceRange>;

pub type TypeParamTypeVar = crate::generic::TypeParamTypeVar<SourceRange>;

impl Located for TypeParamTypeVar {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for TypeParamTypeVar {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type TypeParamParamSpec = crate::generic::TypeParamParamSpec<SourceRange>;

impl Located for TypeParamParamSpec {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for TypeParamParamSpec {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type TypeParamTypeVarTuple = crate::generic::TypeParamTypeVarTuple<SourceRange>;

impl Located for TypeParamTypeVarTuple {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl LocatedMut for TypeParamTypeVarTuple {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

impl Located for TypeParam {
    fn range(&self) -> SourceRange {
        match self {
            Self::TypeVar(node) => node.range(),
            Self::ParamSpec(node) => node.range(),
            Self::TypeVarTuple(node) => node.range(),
        }
    }
}

impl LocatedMut for TypeParam {
    fn range_mut(&mut self) -> &mut SourceRange {
        match self {
            Self::TypeVar(node) => node.range_mut(),
            Self::ParamSpec(node) => node.range_mut(),
            Self::TypeVarTuple(node) => node.range_mut(),
        }
    }
}

pub type Arguments = crate::generic::Arguments<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for Arguments {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for Arguments {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}

pub type ArgWithDefault = crate::generic::ArgWithDefault<SourceRange>;

#[cfg(feature = "all-nodes-with-ranges")]
impl Located for ArgWithDefault {
    fn range(&self) -> SourceRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl LocatedMut for ArgWithDefault {
    fn range_mut(&mut self) -> &mut SourceRange {
        &mut self.range
    }
}