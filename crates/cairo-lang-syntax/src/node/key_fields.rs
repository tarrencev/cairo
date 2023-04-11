// Autogenerated file. To regenerate, please run `cargo run --bin generate-syntax`.
use super::ids::GreenId;
use super::kind::SyntaxKind;
/// Gets the vector of children ids that are the indexing key for this SyntaxKind.
/// Each SyntaxKind has some children that are defined in the spec to be its indexing key
/// for its stable pointer. See [super::stable_ptr].
pub fn get_key_fields(kind: SyntaxKind, children: Vec<GreenId>) -> Vec<GreenId> {
    match kind {
        SyntaxKind::Trivia => vec![],
        SyntaxKind::ExprList => vec![],
        SyntaxKind::ArgNameClause => vec![],
        SyntaxKind::OptionArgNameClauseEmpty => vec![],
        SyntaxKind::Arg => vec![],
        SyntaxKind::ArgClauseNamed => vec![],
        SyntaxKind::ArgClauseUnnamed => vec![],
        SyntaxKind::ArgClauseFieldInitShorthand => vec![],
        SyntaxKind::ExprFieldInitShorthand => vec![],
        SyntaxKind::ArgList => vec![],
        SyntaxKind::ExprMissing => vec![],
        SyntaxKind::PathSegmentSimple => vec![],
        SyntaxKind::OptionTerminalColonColonEmpty => vec![],
        SyntaxKind::PathSegmentWithGenericArgs => vec![],
        SyntaxKind::ExprPath => vec![],
        SyntaxKind::ExprParenthesized => vec![],
        SyntaxKind::ExprUnary => vec![],
        SyntaxKind::ExprBinary => vec![],
        SyntaxKind::ExprTuple => vec![],
        SyntaxKind::ExprFunctionCall => vec![],
        SyntaxKind::ArgListParenthesized => vec![],
        SyntaxKind::ExprStructCtorCall => vec![],
        SyntaxKind::ExprBlock => vec![],
        SyntaxKind::ExprMatch => vec![],
        SyntaxKind::MatchArms => vec![],
        SyntaxKind::MatchArm => vec![],
        SyntaxKind::ExprIf => vec![],
        SyntaxKind::ExprLoop => vec![],
        SyntaxKind::ElseClause => vec![],
        SyntaxKind::OptionElseClauseEmpty => vec![],
        SyntaxKind::ExprErrorPropagate => vec![],
        SyntaxKind::ExprIndexed => vec![],
        SyntaxKind::ExprInlineMacro => vec![],
        SyntaxKind::StructArgExpr => vec![],
        SyntaxKind::OptionStructArgExprEmpty => vec![],
        SyntaxKind::StructArgSingle => vec![/* identifier */ children[0]],
        SyntaxKind::StructArgTail => vec![],
        SyntaxKind::StructArgList => vec![],
        SyntaxKind::ArgListBraced => vec![],
        SyntaxKind::PatternIdentifier => vec![/* name */ children[1]],
        SyntaxKind::PatternStruct => vec![],
        SyntaxKind::PatternStructParamList => vec![],
        SyntaxKind::PatternTuple => vec![],
        SyntaxKind::PatternList => vec![],
        SyntaxKind::PatternStructParamWithExpr => vec![],
        SyntaxKind::PatternEnum => vec![],
        SyntaxKind::TypeClause => vec![],
        SyntaxKind::OptionTypeClauseEmpty => vec![],
        SyntaxKind::ReturnTypeClause => vec![],
        SyntaxKind::OptionReturnTypeClauseEmpty => vec![],
        SyntaxKind::StatementList => vec![],
        SyntaxKind::StatementMissing => vec![],
        SyntaxKind::StatementLet => vec![/* pattern */ children[1]],
        SyntaxKind::OptionTerminalSemicolonEmpty => vec![],
        SyntaxKind::StatementExpr => vec![],
        SyntaxKind::StatementReturn => vec![],
        SyntaxKind::StatementBreak => vec![],
        SyntaxKind::Param => vec![/* name */ children[1]],
        SyntaxKind::ModifierList => vec![],
        SyntaxKind::ParamList => vec![],
        SyntaxKind::ImplicitsClause => vec![],
        SyntaxKind::ImplicitsList => vec![],
        SyntaxKind::OptionImplicitsClauseEmpty => vec![],
        SyntaxKind::OptionTerminalNoPanicEmpty => vec![],
        SyntaxKind::FunctionSignature => vec![],
        SyntaxKind::Member => vec![/* name */ children[0]],
        SyntaxKind::MemberList => vec![],
        SyntaxKind::ItemList => vec![],
        SyntaxKind::Attribute => vec![],
        SyntaxKind::AttributeList => vec![],
        SyntaxKind::ItemModule => vec![/* name */ children[2]],
        SyntaxKind::ModuleBody => vec![],
        SyntaxKind::OptionAttributeArgsEmpty => vec![],
        SyntaxKind::AttributeArgs => vec![],
        SyntaxKind::AttributeArgList => vec![],
        SyntaxKind::FunctionDeclaration => vec![/* name */ children[1]],
        SyntaxKind::ItemConstant => vec![/* name */ children[2]],
        SyntaxKind::FunctionWithBody => vec![/* declaration */ children[1]],
        SyntaxKind::ItemExternFunction => vec![/* declaration */ children[2]],
        SyntaxKind::ItemExternType => vec![/* name */ children[3]],
        SyntaxKind::ItemTrait => vec![/* name */ children[2]],
        SyntaxKind::TraitBody => vec![],
        SyntaxKind::TraitItemList => vec![],
        SyntaxKind::TraitItemFunction => vec![/* declaration */ children[1]],
        SyntaxKind::ItemImpl => vec![/* name */ children[2]],
        SyntaxKind::ImplBody => vec![],
        SyntaxKind::ItemStruct => vec![/* name */ children[2]],
        SyntaxKind::ItemEnum => vec![/* name */ children[2]],
        SyntaxKind::ItemTypeAlias => vec![/* name */ children[2]],
        SyntaxKind::ItemUse => vec![/* name */ children[2]],
        SyntaxKind::GenericArgExpr => vec![],
        SyntaxKind::GenericArgs => vec![],
        SyntaxKind::GenericArgList => vec![],
        SyntaxKind::OptionWrappedGenericParamListEmpty => vec![],
        SyntaxKind::WrappedGenericParamList => vec![],
        SyntaxKind::GenericParamList => vec![],
        SyntaxKind::GenericParamType => vec![/* name */ children[0]],
        SyntaxKind::GenericParamConst => vec![/* name */ children[1]],
        SyntaxKind::GenericParamImpl => vec![/* name */ children[1]],
        SyntaxKind::TokenIdentifier => vec![],
        SyntaxKind::TerminalIdentifier => vec![],
        SyntaxKind::TokenLiteralNumber => vec![],
        SyntaxKind::TerminalLiteralNumber => vec![],
        SyntaxKind::TokenShortString => vec![],
        SyntaxKind::TerminalShortString => vec![],
        SyntaxKind::TokenConst => vec![],
        SyntaxKind::TerminalConst => vec![],
        SyntaxKind::TokenElse => vec![],
        SyntaxKind::TerminalElse => vec![],
        SyntaxKind::TokenEnum => vec![],
        SyntaxKind::TerminalEnum => vec![],
        SyntaxKind::TokenExtern => vec![],
        SyntaxKind::TerminalExtern => vec![],
        SyntaxKind::TokenFalse => vec![],
        SyntaxKind::TerminalFalse => vec![],
        SyntaxKind::TokenFunction => vec![],
        SyntaxKind::TerminalFunction => vec![],
        SyntaxKind::TokenIf => vec![],
        SyntaxKind::TerminalIf => vec![],
        SyntaxKind::TokenLoop => vec![],
        SyntaxKind::TerminalLoop => vec![],
        SyntaxKind::TokenImpl => vec![],
        SyntaxKind::TerminalImpl => vec![],
        SyntaxKind::TokenImplicits => vec![],
        SyntaxKind::TerminalImplicits => vec![],
        SyntaxKind::TokenLet => vec![],
        SyntaxKind::TerminalLet => vec![],
        SyntaxKind::TokenMatch => vec![],
        SyntaxKind::TerminalMatch => vec![],
        SyntaxKind::TokenModule => vec![],
        SyntaxKind::TerminalModule => vec![],
        SyntaxKind::TokenMut => vec![],
        SyntaxKind::TerminalMut => vec![],
        SyntaxKind::TokenNoPanic => vec![],
        SyntaxKind::TerminalNoPanic => vec![],
        SyntaxKind::TokenOf => vec![],
        SyntaxKind::TerminalOf => vec![],
        SyntaxKind::TokenRef => vec![],
        SyntaxKind::TerminalRef => vec![],
        SyntaxKind::TokenReturn => vec![],
        SyntaxKind::TerminalReturn => vec![],
        SyntaxKind::TokenBreak => vec![],
        SyntaxKind::TerminalBreak => vec![],
        SyntaxKind::TokenStruct => vec![],
        SyntaxKind::TerminalStruct => vec![],
        SyntaxKind::TokenTrait => vec![],
        SyntaxKind::TerminalTrait => vec![],
        SyntaxKind::TokenTrue => vec![],
        SyntaxKind::TerminalTrue => vec![],
        SyntaxKind::TokenType => vec![],
        SyntaxKind::TerminalType => vec![],
        SyntaxKind::TokenUse => vec![],
        SyntaxKind::TerminalUse => vec![],
        SyntaxKind::TokenAnd => vec![],
        SyntaxKind::TerminalAnd => vec![],
        SyntaxKind::TokenAndAnd => vec![],
        SyntaxKind::TerminalAndAnd => vec![],
        SyntaxKind::TokenArrow => vec![],
        SyntaxKind::TerminalArrow => vec![],
        SyntaxKind::TokenAt => vec![],
        SyntaxKind::TerminalAt => vec![],
        SyntaxKind::TokenBadCharacters => vec![],
        SyntaxKind::TerminalBadCharacters => vec![],
        SyntaxKind::TokenColon => vec![],
        SyntaxKind::TerminalColon => vec![],
        SyntaxKind::TokenColonColon => vec![],
        SyntaxKind::TerminalColonColon => vec![],
        SyntaxKind::TokenComma => vec![],
        SyntaxKind::TerminalComma => vec![],
        SyntaxKind::TokenDiv => vec![],
        SyntaxKind::TerminalDiv => vec![],
        SyntaxKind::TokenDivEq => vec![],
        SyntaxKind::TerminalDivEq => vec![],
        SyntaxKind::TokenDot => vec![],
        SyntaxKind::TerminalDot => vec![],
        SyntaxKind::TokenDotDot => vec![],
        SyntaxKind::TerminalDotDot => vec![],
        SyntaxKind::TokenEndOfFile => vec![],
        SyntaxKind::TerminalEndOfFile => vec![],
        SyntaxKind::TokenEq => vec![],
        SyntaxKind::TerminalEq => vec![],
        SyntaxKind::TokenEqEq => vec![],
        SyntaxKind::TerminalEqEq => vec![],
        SyntaxKind::TokenGE => vec![],
        SyntaxKind::TerminalGE => vec![],
        SyntaxKind::TokenGT => vec![],
        SyntaxKind::TerminalGT => vec![],
        SyntaxKind::TokenHash => vec![],
        SyntaxKind::TerminalHash => vec![],
        SyntaxKind::TokenLBrace => vec![],
        SyntaxKind::TerminalLBrace => vec![],
        SyntaxKind::TokenLBrack => vec![],
        SyntaxKind::TerminalLBrack => vec![],
        SyntaxKind::TokenLE => vec![],
        SyntaxKind::TerminalLE => vec![],
        SyntaxKind::TokenLParen => vec![],
        SyntaxKind::TerminalLParen => vec![],
        SyntaxKind::TokenLT => vec![],
        SyntaxKind::TerminalLT => vec![],
        SyntaxKind::TokenMatchArrow => vec![],
        SyntaxKind::TerminalMatchArrow => vec![],
        SyntaxKind::TokenMinus => vec![],
        SyntaxKind::TerminalMinus => vec![],
        SyntaxKind::TokenMinusEq => vec![],
        SyntaxKind::TerminalMinusEq => vec![],
        SyntaxKind::TokenMod => vec![],
        SyntaxKind::TerminalMod => vec![],
        SyntaxKind::TokenModEq => vec![],
        SyntaxKind::TerminalModEq => vec![],
        SyntaxKind::TokenMul => vec![],
        SyntaxKind::TerminalMul => vec![],
        SyntaxKind::TokenMulEq => vec![],
        SyntaxKind::TerminalMulEq => vec![],
        SyntaxKind::TokenNeq => vec![],
        SyntaxKind::TerminalNeq => vec![],
        SyntaxKind::TokenNot => vec![],
        SyntaxKind::TerminalNot => vec![],
        SyntaxKind::TokenOr => vec![],
        SyntaxKind::TerminalOr => vec![],
        SyntaxKind::TokenOrOr => vec![],
        SyntaxKind::TerminalOrOr => vec![],
        SyntaxKind::TokenPlus => vec![],
        SyntaxKind::TerminalPlus => vec![],
        SyntaxKind::TokenPlusEq => vec![],
        SyntaxKind::TerminalPlusEq => vec![],
        SyntaxKind::TokenQuestionMark => vec![],
        SyntaxKind::TerminalQuestionMark => vec![],
        SyntaxKind::TokenRBrace => vec![],
        SyntaxKind::TerminalRBrace => vec![],
        SyntaxKind::TokenRBrack => vec![],
        SyntaxKind::TerminalRBrack => vec![],
        SyntaxKind::TokenRParen => vec![],
        SyntaxKind::TerminalRParen => vec![],
        SyntaxKind::TokenSemicolon => vec![],
        SyntaxKind::TerminalSemicolon => vec![],
        SyntaxKind::TokenUnderscore => vec![],
        SyntaxKind::TerminalUnderscore => vec![],
        SyntaxKind::TokenXor => vec![],
        SyntaxKind::TerminalXor => vec![],
        SyntaxKind::SyntaxFile => vec![],
        SyntaxKind::TokenSingleLineComment => vec![],
        SyntaxKind::TokenWhitespace => vec![],
        SyntaxKind::TokenNewline => vec![],
        SyntaxKind::TokenMissing => vec![],
        SyntaxKind::TokenSkipped => vec![],
    }
}
