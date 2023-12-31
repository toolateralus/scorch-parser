use super::lexer::TokenKind;

pub const ARRAY_TNAME: &str = "array";
pub const INT_TNAME: &str = "int";
pub const DOUBLE_TNAME: &str = "double";
pub const BOOL_TNAME: &str = "bool";
pub const STRING_TNAME: &str = "string";
pub const NONE_TNAME: &str = "none";
pub const DYNAMIC_TNAME: &str = "dynamic";
pub const FN_TNAME: &str = "fn";

pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_eof(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
    fn visit_function_decl(&mut self, node: &Node) -> T;
    fn visit_function_call(&mut self, node: &Node) -> T;
    fn visit_program(&mut self, node: &Node) -> T;
    fn visit_repeat_stmnt(&mut self, node: &Node) -> T;
    fn visit_break_stmnt(&mut self, node: &Node) -> T;
    fn visit_relational_expression(&mut self, node: &Node) -> T;
    fn visit_logical_expression(&mut self, node: &Node) -> T;
    // unary operations
    fn visit_not_op(&mut self, node: &Node) -> T;
    fn visit_neg_op(&mut self, node: &Node) -> T;
    fn visit_assignment(&mut self, node: &Node) -> T;
    fn visit_declaration(&mut self, node: &Node) -> T;
    fn visit_block(&mut self, node: &Node) -> T;
    fn visit_expression(&mut self, node: &Node) -> T;
    fn visit_string(&mut self, node: &Node) -> T;
    fn visit_identifier(&mut self, node: &Node) -> T;
    fn visit_bool(&mut self, node: &Node) -> T;
    fn visit_array(&mut self, node: &Node) -> T;
    fn visit_array_access(&mut self, node: &Node) -> T;
    fn visit_type_assoc_block(&mut self, node: &Node) -> T;
    fn visit_if_stmnt(&mut self, node: &Node) -> T;
    fn visit_else_stmnt(&mut self, node: &Node) -> T;
    fn visit_struct_def(&mut self, node: &Node) -> T;
    fn visit_struct_init(&mut self, node: &Node) -> T;
}
#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Box<Node>>),
    Block(Vec<Box<Node>>),

    // literal & values
    Int(i32),
    Bool(bool),
    Undefined(),
    Double(f64),
    String(String),
    Identifier(String),

    // Expressions
    LogicalExpression {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    RelationalExpression {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    BinaryOperation {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },

    // todo: do the same with Unary operations :
    // we can have a special noed for these instead of
    // weaving it in with factors.
    NegOp(Box<Node>), // for unary -
    NotOp(Box<Node>), // for unary !
    ReturnStmnt(Option<Box<Node>>),

    Expression(Box<Node>),
    // Statements
    AssignStmnt {
        id: Box<Node>,
        expression: Box<Node>,
    },

    FunctionCall {
        id: String,
        arguments: Option<Vec<Node>>,
    },

    DeclStmt {
        target_type: String,
        id: String,
        expression: Box<Node>,
        mutable: bool,
    },
    RepeatStmnt {
        iterator_id: Option<String>,
        condition: Option<Box<Node>>,
        block: Box<Node>,
    },
    // not implemented
    IfStmnt {
        condition: Box<Node>,
        block: Box<Node>,
        else_stmnt: Option<Box<Node>>,
    },
    ElseStmnt {
        condition: Option<Box<Node>>,
        block: Box<Node>,
        else_stmnt: Option<Box<Node>>,
    },
    FnDeclStmnt {
        id: String,
        body: Box<Node>,
        params: Vec<Node>,
        return_type: String,
        mutable: bool,
    },
    ParamDeclNode {
        varname: Box<Node>,
        typename: Box<Node>,
    },

    Array {
        typename: String,
        elements: Vec<Box<Node>>,
        init_capacity: usize,
        mutable: bool,
        elements_mutable: bool,
    },
    ArrayAccessExpr {
        id: String,
        index_expr: Box<Node>,
        expression: Option<Box<Node>>,
        assignment: bool,
    },

    StructDecl {
        id: String,
        block: Box<Node>,
    },
    StructInit {
        id: String,
        args: Vec<Node>,
    },
    TypeAssocBlock {
        typename: String,
        block: Box<Node>,
    },
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::TypeAssocBlock { .. } => visitor.visit_type_assoc_block(self),
            Node::Array { .. } => visitor.visit_array(self),
            Node::FunctionCall { .. } => visitor.visit_function_call(self),
            Node::StructInit { .. } => visitor.visit_struct_init(self),
            Node::ArrayAccessExpr { .. } => visitor.visit_array_access(self),

            Node::Undefined() => visitor.visit_eof(self),
            Node::Identifier(..) => visitor.visit_identifier(self),
            Node::String(..) => visitor.visit_string(self),
            Node::Bool(..) => visitor.visit_bool(self),
            Node::Int(..) => visitor.visit_number(self),
            Node::Double(..) => visitor.visit_number(self),
            Node::ParamDeclNode { varname: _, typename: _ } => {
                panic!("this is not implemented")
            }
            Node::Program(..) => visitor.visit_program(self),
            Node::Block(..) => visitor.visit_block(self),
            Node::Expression(..) => visitor.visit_expression(self),

            Node::DeclStmt { .. } => visitor.visit_declaration(self),
            Node::StructDecl { .. } => visitor.visit_struct_def(self),
            Node::FnDeclStmnt { .. } => visitor.visit_function_decl(self),

            Node::IfStmnt { .. } => visitor.visit_if_stmnt(self),
            Node::ElseStmnt { .. } => visitor.visit_else_stmnt(self),
            Node::ReturnStmnt(..) => visitor.visit_break_stmnt(self),
            Node::AssignStmnt { .. } => visitor.visit_assignment(self),
            Node::RepeatStmnt { .. } => visitor.visit_repeat_stmnt(self),
            
            Node::BinaryOperation { .. } => visitor.visit_binary_op(self),
            Node::RelationalExpression { .. } => visitor.visit_relational_expression(self),
            Node::LogicalExpression { .. } => visitor.visit_logical_expression(self),

            Node::NegOp(..) => visitor.visit_neg_op(self),
            Node::NotOp(..) => visitor.visit_not_op(self),
        }
    }
}
