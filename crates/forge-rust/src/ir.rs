use serde::{Deserialize, Serialize};

/// Universal Intermediate Representation (UIR) for polyglot code refactoring.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirModule {
    pub name: String,
    pub doc: Option<String>,
    pub items: Vec<UirItem>,
    pub required_dependencies: Vec<String>,
}

impl UirModule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            items: Vec::new(),
            required_dependencies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UirItem {
    Struct(UirStruct),
    Enum(UirEnum),
    Function(UirFunction),
    Trait(UirTrait),
    Const(UirConst),
    RawBlock(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirStruct {
    pub name: String,
    pub doc: Option<String>,
    pub is_pub: bool,
    pub fields: Vec<UirField>,
    pub methods: Vec<UirFunction>,
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirField {
    pub name: String,
    pub ty: UirType,
    pub is_pub: bool,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirEnum {
    pub name: String,
    pub doc: Option<String>,
    pub is_pub: bool,
    pub variants: Vec<UirVariant>,
    pub derives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirVariant {
    pub name: String,
    pub fields: Option<Vec<UirField>>,
    pub discriminant: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirTrait {
    pub name: String,
    pub doc: Option<String>,
    pub is_pub: bool,
    pub methods: Vec<UirFunction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirConst {
    pub name: String,
    pub ty: UirType,
    pub value: String,
    pub is_pub: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirFunction {
    pub name: String,
    pub doc: Option<String>,
    pub is_pub: bool,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_method: bool,
    pub struct_target: Option<String>,
    pub self_kind: Option<UirSelfKind>,
    pub params: Vec<UirParam>,
    pub return_type: Option<UirType>,
    pub body: Vec<UirStmt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UirSelfKind {
    Ref,
    MutRef,
    Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirParam {
    pub name: String,
    pub ty: UirType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UirType {
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    StrRef,
    Custom(String),
    Vec(Box<UirType>),
    Slice(Box<UirType>),
    Array(Box<UirType>, usize),
    Option(Box<UirType>),
    Result { ok: Box<UirType>, err: Box<UirType> },
    Boxed(Box<UirType>),
    ArcMutex(Box<UirType>),
    Reference { mutable: bool, inner: Box<UirType> },
    HashMap { key: Box<UirType>, value: Box<UirType> },
    Tuple(Vec<UirType>),
    RawPointer { mutable: bool, inner: Box<UirType> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UirStmt {
    Let {
        name: String,
        mutable: bool,
        ty: Option<UirType>,
        value: Option<UirExpr>,
    },
    Assign {
        target: String,
        value: UirExpr,
    },
    Expr(UirExpr),
    Return(Option<UirExpr>),
    If {
        condition: UirExpr,
        then_branch: Vec<UirStmt>,
        else_branch: Option<Vec<UirStmt>>,
    },
    While {
        condition: UirExpr,
        body: Vec<UirStmt>,
    },
    For {
        var: String,
        iter: UirExpr,
        body: Vec<UirStmt>,
    },
    Match {
        expr: UirExpr,
        arms: Vec<UirMatchArm>,
    },
    SpawnTask {
        body: Vec<UirStmt>,
    },
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirMatchArm {
    pub pattern: String,
    pub body: Vec<UirStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UirExpr {
    Literal(String),
    Var(String),
    BinaryOp {
        op: String,
        left: Box<UirExpr>,
        right: Box<UirExpr>,
    },
    Call {
        callee: String,
        args: Vec<UirExpr>,
    },
    MethodCall {
        receiver: Box<UirExpr>,
        method: String,
        args: Vec<UirExpr>,
    },
    FieldAccess {
        receiver: Box<UirExpr>,
        field: String,
    },
    StructInit {
        name: String,
        fields: Vec<(String, UirExpr)>,
    },
    VecInit(Vec<UirExpr>),
    Try(Box<UirExpr>),
    Await(Box<UirExpr>),
    Raw(String),
}
