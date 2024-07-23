//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    AT,
    POUND,
    TILDE,
    QUESTION,
    DOLLAR,
    AMP,
    PIPE,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    UNDERSCORE,
    DOT,
    DOT2,
    DOT3,
    DOT2EQ,
    COLON,
    COLON2,
    EQ,
    EQ2,
    FAT_ARROW,
    BANG,
    NEQ,
    MINUS,
    THIN_ARROW,
    LTEQ,
    GTEQ,
    PLUSEQ,
    MINUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AMP2,
    PIPE2,
    SHL,
    SHR,
    SHLEQ,
    SHREQ,
    BIGAND,
    BIGOR,
    EQUIV,
    IMPLY,
    EXPLY,
    EQEQEQ,
    NEEQ,
    ExtEq,
    ExtNe,
    ExtDeepEq,
    ExtDeepNe,
    ABSTRACT_KW,
    AS_KW,
    ASYNC_KW,
    AWAIT_KW,
    BECOME_KW,
    BOX_KW,
    BREAK_KW,
    CONST_KW,
    CONTINUE_KW,
    CRATE_KW,
    DO_KW,
    DYN_KW,
    ELSE_KW,
    ENUM_KW,
    EXTERN_KW,
    FALSE_KW,
    FINAL_KW,
    FN_KW,
    FOR_KW,
    IF_KW,
    IMPL_KW,
    IN_KW,
    LET_KW,
    LOOP_KW,
    MACRO_KW,
    MATCH_KW,
    MOD_KW,
    MOVE_KW,
    MUT_KW,
    OVERRIDE_KW,
    PRIV_KW,
    PUB_KW,
    REF_KW,
    RETURN_KW,
    SELF_KW,
    SELF_TYPE_KW,
    STATIC_KW,
    STRUCT_KW,
    SUPER_KW,
    TRAIT_KW,
    TRUE_KW,
    TRY_KW,
    TYPE_KW,
    TYPEOF_KW,
    UNSAFE_KW,
    UNSIZED_KW,
    USE_KW,
    VIRTUAL_KW,
    WHERE_KW,
    WHILE_KW,
    YIELD_KW,
    VERUS_KW,
    SPEC_KW,
    PROOF_KW,
    REQUIRES_KW,
    ENSURES_KW,
    CHECKED_KW,
    RECOMMENDS_KW,
    DECREASES_KW,
    EXEC_KW,
    OPEN_KW,
    CLOSED_KW,
    TRACKED_KW,
    GHOST_KW,
    INVARIANT_EXCEPT_BREAK_KW,
    INVARIANT_KW,
    ASSERT_KW,
    ASSUME_KW,
    IMPLIES_KW,
    BY_KW,
    FORALL_KW,
    EXISTS_KW,
    CHOOSE_KW,
    VIA_KW,
    WHEN_KW,
    TRIGGER_KW,
    GLOBAL_KW,
    BROADCAST_KW,
    GROUP_KW,
    IS_KW,
    AUTO_KW,
    BUILTIN_KW,
    DEFAULT_KW,
    EXISTENTIAL_KW,
    UNION_KW,
    RAW_KW,
    MACRO_RULES_KW,
    YEET_KW,
    OFFSET_OF_KW,
    ASM_KW,
    FORMAT_ARGS_KW,
    INT_NUMBER,
    FLOAT_NUMBER,
    CHAR,
    BYTE,
    STRING,
    BYTE_STRING,
    C_STRING,
    ERROR,
    IDENT,
    WHITESPACE,
    LIFETIME_IDENT,
    COMMENT,
    SHEBANG,
    SOURCE_FILE,
    STRUCT,
    UNION,
    ENUM,
    FN,
    RET_TYPE,
    EXTERN_CRATE,
    MODULE,
    USE,
    STATIC,
    CONST,
    TRAIT,
    TRAIT_ALIAS,
    IMPL,
    TYPE_ALIAS,
    MACRO_CALL,
    MACRO_RULES,
    MACRO_ARM,
    TOKEN_TREE,
    MACRO_DEF,
    PAREN_TYPE,
    TUPLE_TYPE,
    MACRO_TYPE,
    NEVER_TYPE,
    PATH_TYPE,
    PTR_TYPE,
    ARRAY_TYPE,
    SLICE_TYPE,
    REF_TYPE,
    INFER_TYPE,
    FN_PTR_TYPE,
    FOR_TYPE,
    IMPL_TRAIT_TYPE,
    DYN_TRAIT_TYPE,
    OR_PAT,
    PAREN_PAT,
    REF_PAT,
    BOX_PAT,
    IDENT_PAT,
    WILDCARD_PAT,
    REST_PAT,
    PATH_PAT,
    RECORD_PAT,
    RECORD_PAT_FIELD_LIST,
    RECORD_PAT_FIELD,
    TUPLE_STRUCT_PAT,
    TUPLE_PAT,
    SLICE_PAT,
    RANGE_PAT,
    LITERAL_PAT,
    MACRO_PAT,
    CONST_BLOCK_PAT,
    TUPLE_EXPR,
    ARRAY_EXPR,
    PAREN_EXPR,
    PATH_EXPR,
    CLOSURE_EXPR,
    IF_EXPR,
    WHILE_EXPR,
    LOOP_EXPR,
    FOR_EXPR,
    CONTINUE_EXPR,
    BREAK_EXPR,
    LABEL,
    BLOCK_EXPR,
    STMT_LIST,
    RETURN_EXPR,
    BECOME_EXPR,
    YIELD_EXPR,
    YEET_EXPR,
    LET_EXPR,
    UNDERSCORE_EXPR,
    MACRO_EXPR,
    MATCH_EXPR,
    MATCH_ARM_LIST,
    MATCH_ARM,
    MATCH_GUARD,
    RECORD_EXPR,
    RECORD_EXPR_FIELD_LIST,
    RECORD_EXPR_FIELD,
    OFFSET_OF_EXPR,
    ASM_EXPR,
    FORMAT_ARGS_EXPR,
    FORMAT_ARGS_ARG,
    CALL_EXPR,
    INDEX_EXPR,
    METHOD_CALL_EXPR,
    FIELD_EXPR,
    AWAIT_EXPR,
    TRY_EXPR,
    CAST_EXPR,
    REF_EXPR,
    PREFIX_EXPR,
    RANGE_EXPR,
    BIN_EXPR,
    EXTERN_BLOCK,
    EXTERN_ITEM_LIST,
    VARIANT,
    RECORD_FIELD_LIST,
    RECORD_FIELD,
    TUPLE_FIELD_LIST,
    TUPLE_FIELD,
    VARIANT_LIST,
    ITEM_LIST,
    ASSOC_ITEM_LIST,
    ATTR,
    META,
    USE_TREE,
    USE_TREE_LIST,
    PATH,
    PATH_SEGMENT,
    LITERAL,
    RENAME,
    VISIBILITY,
    WHERE_CLAUSE,
    WHERE_PRED,
    ABI,
    NAME,
    NAME_REF,
    LET_STMT,
    LET_ELSE,
    EXPR_STMT,
    GENERIC_PARAM_LIST,
    GENERIC_PARAM,
    LIFETIME_PARAM,
    TYPE_PARAM,
    RETURN_TYPE_ARG,
    CONST_PARAM,
    GENERIC_ARG_LIST,
    LIFETIME,
    LIFETIME_ARG,
    TYPE_ARG,
    ASSOC_TYPE_ARG,
    CONST_ARG,
    PARAM_LIST,
    PARAM,
    SELF_PARAM,
    ARG_LIST,
    TYPE_BOUND,
    TYPE_BOUND_LIST,
    MACRO_ITEMS,
    MACRO_STMTS,
    MACRO_EAGER_INPUT,
    REQUIRES_CLAUSE,
    ENSURES_CLAUSE,
    DECREASES_CLAUSE,
    RECOMMENDS_CLAUSE,
    LOOP_CLAUSE,
    INVARIANT_EXCEPT_BREAK_CLAUSE,
    INVARIANT_CLAUSE,
    ASSERT_EXPR,
    ASSERT_FORALL_EXPR,
    ASSUME_EXPR,
    VIEW_EXPR,
    PUBLISH,
    FN_MODE,
    DATA_MODE,
    MODE_SPEC_CHECKED,
    PROVER,
    SIGNATURE_DECREASES,
    TRIGGER_ATTRIBUTE,
    VERUS_GLOBAL,
    BROADCAST_USE_LIST,
    BROADCAST_USE,
    BROADCAST_GROUP_IDENTIFIER,
    BROADCAST_GROUP_MEMBER,
    BROADCAST_GROUP_LIST,
    BROADCAST_GROUP,
    IS_EXPR,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            ABSTRACT_KW
                | AS_KW
                | ASYNC_KW
                | AWAIT_KW
                | BECOME_KW
                | BOX_KW
                | BREAK_KW
                | CONST_KW
                | CONTINUE_KW
                | CRATE_KW
                | DO_KW
                | DYN_KW
                | ELSE_KW
                | ENUM_KW
                | EXTERN_KW
                | FALSE_KW
                | FINAL_KW
                | FN_KW
                | FOR_KW
                | IF_KW
                | IMPL_KW
                | IN_KW
                | LET_KW
                | LOOP_KW
                | MACRO_KW
                | MATCH_KW
                | MOD_KW
                | MOVE_KW
                | MUT_KW
                | OVERRIDE_KW
                | PRIV_KW
                | PUB_KW
                | REF_KW
                | RETURN_KW
                | SELF_KW
                | SELF_TYPE_KW
                | STATIC_KW
                | STRUCT_KW
                | SUPER_KW
                | TRAIT_KW
                | TRUE_KW
                | TRY_KW
                | TYPE_KW
                | TYPEOF_KW
                | UNSAFE_KW
                | UNSIZED_KW
                | USE_KW
                | VIRTUAL_KW
                | WHERE_KW
                | WHILE_KW
                | YIELD_KW
                | VERUS_KW
                | SPEC_KW
                | PROOF_KW
                | REQUIRES_KW
                | ENSURES_KW
                | CHECKED_KW
                | RECOMMENDS_KW
                | DECREASES_KW
                | EXEC_KW
                | OPEN_KW
                | CLOSED_KW
                | TRACKED_KW
                | GHOST_KW
                | INVARIANT_EXCEPT_BREAK_KW
                | INVARIANT_KW
                | ASSERT_KW
                | ASSUME_KW
                | IMPLIES_KW
                | BY_KW
                | FORALL_KW
                | EXISTS_KW
                | CHOOSE_KW
                | VIA_KW
                | WHEN_KW
                | TRIGGER_KW
                | GLOBAL_KW
                | BROADCAST_KW
                | GROUP_KW
                | IS_KW
                | AUTO_KW
                | BUILTIN_KW
                | DEFAULT_KW
                | EXISTENTIAL_KW
                | UNION_KW
                | RAW_KW
                | MACRO_RULES_KW
                | YEET_KW
                | OFFSET_OF_KW
                | ASM_KW
                | FORMAT_ARGS_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            SEMICOLON
                | COMMA
                | L_PAREN
                | R_PAREN
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
                | L_ANGLE
                | R_ANGLE
                | AT
                | POUND
                | TILDE
                | QUESTION
                | DOLLAR
                | AMP
                | PIPE
                | PLUS
                | STAR
                | SLASH
                | CARET
                | PERCENT
                | UNDERSCORE
                | DOT
                | DOT2
                | DOT3
                | DOT2EQ
                | COLON
                | COLON2
                | EQ
                | EQ2
                | FAT_ARROW
                | BANG
                | NEQ
                | MINUS
                | THIN_ARROW
                | LTEQ
                | GTEQ
                | PLUSEQ
                | MINUSEQ
                | PIPEEQ
                | AMPEQ
                | CARETEQ
                | SLASHEQ
                | STAREQ
                | PERCENTEQ
                | AMP2
                | PIPE2
                | SHL
                | SHR
                | SHLEQ
                | SHREQ
                | BIGAND
                | BIGOR
                | EQUIV
                | IMPLY
                | EXPLY
                | EQEQEQ
                | NEEQ
                | ExtEq
                | ExtNe
                | ExtDeepEq
                | ExtDeepNe
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(self, INT_NUMBER | FLOAT_NUMBER | CHAR | BYTE | STRING | BYTE_STRING | C_STRING)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "abstract" => ABSTRACT_KW,
            "as" => AS_KW,
            "async" => ASYNC_KW,
            "await" => AWAIT_KW,
            "become" => BECOME_KW,
            "box" => BOX_KW,
            "break" => BREAK_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "crate" => CRATE_KW,
            "do" => DO_KW,
            "dyn" => DYN_KW,
            "else" => ELSE_KW,
            "enum" => ENUM_KW,
            "extern" => EXTERN_KW,
            "false" => FALSE_KW,
            "final" => FINAL_KW,
            "fn" => FN_KW,
            "for" => FOR_KW,
            "if" => IF_KW,
            "impl" => IMPL_KW,
            "in" => IN_KW,
            "let" => LET_KW,
            "loop" => LOOP_KW,
            "macro" => MACRO_KW,
            "match" => MATCH_KW,
            "mod" => MOD_KW,
            "move" => MOVE_KW,
            "mut" => MUT_KW,
            "override" => OVERRIDE_KW,
            "priv" => PRIV_KW,
            "pub" => PUB_KW,
            "ref" => REF_KW,
            "return" => RETURN_KW,
            "self" => SELF_KW,
            "Self" => SELF_TYPE_KW,
            "static" => STATIC_KW,
            "struct" => STRUCT_KW,
            "super" => SUPER_KW,
            "trait" => TRAIT_KW,
            "true" => TRUE_KW,
            "try" => TRY_KW,
            "type" => TYPE_KW,
            "typeof" => TYPEOF_KW,
            "unsafe" => UNSAFE_KW,
            "unsized" => UNSIZED_KW,
            "use" => USE_KW,
            "virtual" => VIRTUAL_KW,
            "where" => WHERE_KW,
            "while" => WHILE_KW,
            "yield" => YIELD_KW,
            "verus" => VERUS_KW,
            "spec" => SPEC_KW,
            "proof" => PROOF_KW,
            "requires" => REQUIRES_KW,
            "ensures" => ENSURES_KW,
            "checked" => CHECKED_KW,
            "recommends" => RECOMMENDS_KW,
            "decreases" => DECREASES_KW,
            "exec" => EXEC_KW,
            "open" => OPEN_KW,
            "closed" => CLOSED_KW,
            "tracked" => TRACKED_KW,
            "ghost" => GHOST_KW,
            "invariant_except_break" => INVARIANT_EXCEPT_BREAK_KW,
            "invariant" => INVARIANT_KW,
            "assert" => ASSERT_KW,
            "assume" => ASSUME_KW,
            "implies" => IMPLIES_KW,
            "by" => BY_KW,
            "forall" => FORALL_KW,
            "exists" => EXISTS_KW,
            "choose" => CHOOSE_KW,
            "via" => VIA_KW,
            "when" => WHEN_KW,
            "trigger" => TRIGGER_KW,
            "global" => GLOBAL_KW,
            "broadcast" => BROADCAST_KW,
            "group" => GROUP_KW,
            "is" => IS_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "auto" => AUTO_KW,
            "builtin" => BUILTIN_KW,
            "default" => DEFAULT_KW,
            "existential" => EXISTENTIAL_KW,
            "union" => UNION_KW,
            "raw" => RAW_KW,
            "macro_rules" => MACRO_RULES_KW,
            "yeet" => YEET_KW,
            "offset_of" => OFFSET_OF_KW,
            "asm" => ASM_KW,
            "format_args" => FORMAT_ARGS_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            ';' => SEMICOLON,
            ',' => COMMA,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '@' => AT,
            '#' => POUND,
            '~' => TILDE,
            '?' => QUESTION,
            '$' => DOLLAR,
            '&' => AMP,
            '|' => PIPE,
            '+' => PLUS,
            '*' => STAR,
            '/' => SLASH,
            '^' => CARET,
            '%' => PERCENT,
            '_' => UNDERSCORE,
            '.' => DOT,
            ':' => COLON,
            '=' => EQ,
            '!' => BANG,
            '-' => MINUS,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: SyntaxKind :: R_CURLY } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [@] => { $ crate :: SyntaxKind :: AT } ; [#] => { $ crate :: SyntaxKind :: POUND } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [?] => { $ crate :: SyntaxKind :: QUESTION } ; [$] => { $ crate :: SyntaxKind :: DOLLAR } ; [&] => { $ crate :: SyntaxKind :: AMP } ; [|] => { $ crate :: SyntaxKind :: PIPE } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [^] => { $ crate :: SyntaxKind :: CARET } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [_] => { $ crate :: SyntaxKind :: UNDERSCORE } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [..] => { $ crate :: SyntaxKind :: DOT2 } ; [...] => { $ crate :: SyntaxKind :: DOT3 } ; [..=] => { $ crate :: SyntaxKind :: DOT2EQ } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [::] => { $ crate :: SyntaxKind :: COLON2 } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [==] => { $ crate :: SyntaxKind :: EQ2 } ; [=>] => { $ crate :: SyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: SyntaxKind :: BANG } ; [!=] => { $ crate :: SyntaxKind :: NEQ } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [->] => { $ crate :: SyntaxKind :: THIN_ARROW } ; [<=] => { $ crate :: SyntaxKind :: LTEQ } ; [>=] => { $ crate :: SyntaxKind :: GTEQ } ; [+=] => { $ crate :: SyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: SyntaxKind :: MINUSEQ } ; [|=] => { $ crate :: SyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: SyntaxKind :: AMPEQ } ; [^=] => { $ crate :: SyntaxKind :: CARETEQ } ; [/=] => { $ crate :: SyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: SyntaxKind :: STAREQ } ; [%=] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [&&] => { $ crate :: SyntaxKind :: AMP2 } ; [||] => { $ crate :: SyntaxKind :: PIPE2 } ; [<<] => { $ crate :: SyntaxKind :: SHL } ; [>>] => { $ crate :: SyntaxKind :: SHR } ; [<<=] => { $ crate :: SyntaxKind :: SHLEQ } ; [>>=] => { $ crate :: SyntaxKind :: SHREQ } ; [&&&] => { $ crate :: SyntaxKind :: BIGAND } ; [|||] => { $ crate :: SyntaxKind :: BIGOR } ; [<==>] => { $ crate :: SyntaxKind :: EQUIV } ; [==>] => { $ crate :: SyntaxKind :: IMPLY } ; [<==] => { $ crate :: SyntaxKind :: EXPLY } ; [===] => { $ crate :: SyntaxKind :: EQEQEQ } ; [!==] => { $ crate :: SyntaxKind :: NEEQ } ; [=~=] => { $ crate :: SyntaxKind :: ExtEq } ; [!~=] => { $ crate :: SyntaxKind :: ExtNe } ; [=~~=] => { $ crate :: SyntaxKind :: ExtDeepEq } ; [!~~=] => { $ crate :: SyntaxKind :: ExtDeepNe } ; [abstract] => { $ crate :: SyntaxKind :: ABSTRACT_KW } ; [as] => { $ crate :: SyntaxKind :: AS_KW } ; [async] => { $ crate :: SyntaxKind :: ASYNC_KW } ; [await] => { $ crate :: SyntaxKind :: AWAIT_KW } ; [become] => { $ crate :: SyntaxKind :: BECOME_KW } ; [box] => { $ crate :: SyntaxKind :: BOX_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [const] => { $ crate :: SyntaxKind :: CONST_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [crate] => { $ crate :: SyntaxKind :: CRATE_KW } ; [do] => { $ crate :: SyntaxKind :: DO_KW } ; [dyn] => { $ crate :: SyntaxKind :: DYN_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [enum] => { $ crate :: SyntaxKind :: ENUM_KW } ; [extern] => { $ crate :: SyntaxKind :: EXTERN_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [final] => { $ crate :: SyntaxKind :: FINAL_KW } ; [fn] => { $ crate :: SyntaxKind :: FN_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [impl] => { $ crate :: SyntaxKind :: IMPL_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [loop] => { $ crate :: SyntaxKind :: LOOP_KW } ; [macro] => { $ crate :: SyntaxKind :: MACRO_KW } ; [match] => { $ crate :: SyntaxKind :: MATCH_KW } ; [mod] => { $ crate :: SyntaxKind :: MOD_KW } ; [move] => { $ crate :: SyntaxKind :: MOVE_KW } ; [mut] => { $ crate :: SyntaxKind :: MUT_KW } ; [override] => { $ crate :: SyntaxKind :: OVERRIDE_KW } ; [priv] => { $ crate :: SyntaxKind :: PRIV_KW } ; [pub] => { $ crate :: SyntaxKind :: PUB_KW } ; [ref] => { $ crate :: SyntaxKind :: REF_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [self] => { $ crate :: SyntaxKind :: SELF_KW } ; [Self] => { $ crate :: SyntaxKind :: SELF_TYPE_KW } ; [static] => { $ crate :: SyntaxKind :: STATIC_KW } ; [struct] => { $ crate :: SyntaxKind :: STRUCT_KW } ; [super] => { $ crate :: SyntaxKind :: SUPER_KW } ; [trait] => { $ crate :: SyntaxKind :: TRAIT_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [try] => { $ crate :: SyntaxKind :: TRY_KW } ; [type] => { $ crate :: SyntaxKind :: TYPE_KW } ; [typeof] => { $ crate :: SyntaxKind :: TYPEOF_KW } ; [unsafe] => { $ crate :: SyntaxKind :: UNSAFE_KW } ; [unsized] => { $ crate :: SyntaxKind :: UNSIZED_KW } ; [use] => { $ crate :: SyntaxKind :: USE_KW } ; [virtual] => { $ crate :: SyntaxKind :: VIRTUAL_KW } ; [where] => { $ crate :: SyntaxKind :: WHERE_KW } ; [while] => { $ crate :: SyntaxKind :: WHILE_KW } ; [yield] => { $ crate :: SyntaxKind :: YIELD_KW } ; [verus] => { $ crate :: SyntaxKind :: VERUS_KW } ; [spec] => { $ crate :: SyntaxKind :: SPEC_KW } ; [proof] => { $ crate :: SyntaxKind :: PROOF_KW } ; [requires] => { $ crate :: SyntaxKind :: REQUIRES_KW } ; [ensures] => { $ crate :: SyntaxKind :: ENSURES_KW } ; [checked] => { $ crate :: SyntaxKind :: CHECKED_KW } ; [recommends] => { $ crate :: SyntaxKind :: RECOMMENDS_KW } ; [decreases] => { $ crate :: SyntaxKind :: DECREASES_KW } ; [exec] => { $ crate :: SyntaxKind :: EXEC_KW } ; [open] => { $ crate :: SyntaxKind :: OPEN_KW } ; [closed] => { $ crate :: SyntaxKind :: CLOSED_KW } ; [tracked] => { $ crate :: SyntaxKind :: TRACKED_KW } ; [ghost] => { $ crate :: SyntaxKind :: GHOST_KW } ; [invariant_except_break] => { $ crate :: SyntaxKind :: INVARIANT_EXCEPT_BREAK_KW } ; [invariant] => { $ crate :: SyntaxKind :: INVARIANT_KW } ; [assert] => { $ crate :: SyntaxKind :: ASSERT_KW } ; [assume] => { $ crate :: SyntaxKind :: ASSUME_KW } ; [implies] => { $ crate :: SyntaxKind :: IMPLIES_KW } ; [by] => { $ crate :: SyntaxKind :: BY_KW } ; [forall] => { $ crate :: SyntaxKind :: FORALL_KW } ; [exists] => { $ crate :: SyntaxKind :: EXISTS_KW } ; [choose] => { $ crate :: SyntaxKind :: CHOOSE_KW } ; [via] => { $ crate :: SyntaxKind :: VIA_KW } ; [when] => { $ crate :: SyntaxKind :: WHEN_KW } ; [trigger] => { $ crate :: SyntaxKind :: TRIGGER_KW } ; [global] => { $ crate :: SyntaxKind :: GLOBAL_KW } ; [broadcast] => { $ crate :: SyntaxKind :: BROADCAST_KW } ; [group] => { $ crate :: SyntaxKind :: GROUP_KW } ; [is] => { $ crate :: SyntaxKind :: IS_KW } ; [auto] => { $ crate :: SyntaxKind :: AUTO_KW } ; [builtin] => { $ crate :: SyntaxKind :: BUILTIN_KW } ; [default] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; [existential] => { $ crate :: SyntaxKind :: EXISTENTIAL_KW } ; [union] => { $ crate :: SyntaxKind :: UNION_KW } ; [raw] => { $ crate :: SyntaxKind :: RAW_KW } ; [macro_rules] => { $ crate :: SyntaxKind :: MACRO_RULES_KW } ; [yeet] => { $ crate :: SyntaxKind :: YEET_KW } ; [offset_of] => { $ crate :: SyntaxKind :: OFFSET_OF_KW } ; [asm] => { $ crate :: SyntaxKind :: ASM_KW } ; [format_args] => { $ crate :: SyntaxKind :: FORMAT_ARGS_KW } ; [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }
