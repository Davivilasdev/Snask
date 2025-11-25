use crate::ast::{Program, Stmt, StmtKind, Expr, ExprKind, LiteralValue, BinaryOp, UnaryOp, VarDecl, MutDecl, ConstDecl, VarSet, ConditionalStmt, LoopStmt, FuncDecl};
use crate::symbol_table::{SymbolTable, Symbol};
use crate::types::Type;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Dict(HashMap<Value, Value>),
    Nil,
    Function(FuncDecl),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    (*n as i64).hash(state);
                } else {
                    n.to_bits().hash(state);
                }
            },
            Value::String(s) => s.hash(state),
            Value::Boolean(b) => b.hash(state),
            Value::List(_) => { "List".hash(state); },
            Value::Dict(_) => { "Dict".hash(state); },
            Value::Nil => "Nil".hash(state),
            Value::Function(f) => f.name.hash(state),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i < list.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Value::Dict(dict) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in dict {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                    first = false;
                }
                write!(f, "}}")
            },
            Value::Nil => write!(f, "nil"),
            Value::Function(func) => write!(f, "<fun {}>", func.name),
        }
    }
}

impl Eq for Value {}

pub enum InterpretResult {
    Ok,
    RuntimeError,
}

pub struct Interpreter {
    globals: SymbolTable,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: SymbolTable::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> InterpretResult {
        for statement in program {
            if let Err(_) = self.interpret_statement(statement) {
                // TODO: Print error with location
                return InterpretResult::RuntimeError;
            }
        }
        InterpretResult::Ok
    }

    fn interpret_statement(&mut self, statement: Stmt) -> Result<(), String> {
        match statement.kind {
            StmtKind::VarDeclaration(var_decl) => self.execute_var_declaration(var_decl),
            StmtKind::MutDeclaration(mut_decl) => self.execute_mut_declaration(mut_decl),
            StmtKind::ConstDeclaration(const_decl) => self.execute_const_declaration(const_decl),
            StmtKind::VarAssignment(var_set) => self.execute_var_assignment(var_set),
            StmtKind::Print(expressions) => self.execute_print_statement(expressions),
            StmtKind::Input { name, var_type } => self.execute_input_statement(name, var_type),
            StmtKind::Conditional(conditional) => self.execute_conditional_statement(conditional),
            StmtKind::Loop(loop_stmt) => self.execute_loop_statement(loop_stmt),
            StmtKind::FuncDeclaration(func_decl) => self.execute_func_declaration(func_decl),
            StmtKind::Return(expr) => self.execute_return_statement(expr),
            StmtKind::FuncCall(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            },
            _ => Err(format!("Statement not yet implemented: {:?}", statement.kind)),
        }
    }

    fn execute_input_statement(&mut self, name: String, var_type: Type) -> Result<(), String> {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Err("Não foi possível ler a entrada do console.".to_string());
        }
        let trimmed_input = input.trim();

        let value = match var_type {
            Type::String => Value::String(trimmed_input.to_string()),
            Type::Float => trimmed_input.parse::<f64>().map(Value::Number).map_err(|_| format!("Entrada inválida. Esperado um número float, mas recebeu '{}'.", trimmed_input))?,
            Type::Int => trimmed_input.parse::<i64>().map(|n| Value::Number(n as f64)).map_err(|_| format!("Entrada inválida. Esperado um número inteiro, mas recebeu '{}'.", trimmed_input))?,
            Type::Bool => trimmed_input.parse::<bool>().map(Value::Boolean).map_err(|_| format!("Entrada inválida. Esperado 'true' ou 'false', mas recebeu '{}'.", trimmed_input))?,
            _ => return Err(format!("Tipo de 'input' não suportado: {:?}", var_type)),
        };

        self.globals.define(name, value, true, true);
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expr) -> Result<Value, String> {
        match expression.kind {
            ExprKind::Literal(literal) => Ok(self.evaluate_literal(literal)),
            ExprKind::Variable(name) => self.evaluate_variable(name),
            ExprKind::Unary { op, expr } => self.evaluate_unary_expression(op, *expr),
            ExprKind::Binary { op, left, right } => self.evaluate_binary_expression(op, *left, *right),
            ExprKind::FunctionCall { callee, args } => self.evaluate_function_call(*callee, args),
            ExprKind::IndexAccess { target, index } => self.evaluate_index_access(*target, *index),
            ExprKind::PropertyAccess { .. } => Err("Property access must be part of a call".to_string()),
        }
    }

    fn evaluate_index_access(&mut self, target: Expr, index: Expr) -> Result<Value, String> {
        let target_val = self.evaluate_expression(target)?;
        let index_val = self.evaluate_expression(index)?;

        match target_val {
            Value::List(list) => {
                if let Value::Number(idx) = index_val {
                    let idx = idx as usize;
                    if idx < list.len() {
                        Ok(list[idx].clone())
                    } else {
                        Err(format!("Erro de tempo de execução: Índice fora dos limites da lista. Tamanho: {}, Índice: {}", list.len(), idx))
                    }
                } else {
                    Err(format!("Erro de tempo de execução: Índice de lista não numérico: {:?}", index_val))
                }
            },
            Value::Dict(dict) => {
                if dict.contains_key(&index_val) {
                    Ok(dict[&index_val].clone())
                } else {
                    Err(format!("Erro de tempo de execução: Chave de dicionário não encontrada: {:?}", index_val))
                }
            },
            _ => Err(format!("Erro de tempo de execução: Tentativa de indexar valor não indexável: {:?}", target_val)),
        }
    }

    fn evaluate_literal(&mut self, literal: LiteralValue) -> Value {
        match literal {
            LiteralValue::Number(n) => Value::Number(n),
            LiteralValue::String(s) => Value::String(s),
            LiteralValue::Boolean(b) => Value::Boolean(b),
            LiteralValue::List(expr_list) => {
                let mut list = Vec::new();
                for expr in expr_list {
                    if let Ok(val) = self.evaluate_expression(expr) {
                        list.push(val);
                    } else {
                        list.push(Value::Nil);
                    }
                }
                Value::List(list)
            },
            LiteralValue::Dict(expr_dict) => {
                let mut dict = HashMap::new();
                for (key_expr, val_expr) in expr_dict {
                    if let (Ok(key_val), Ok(actual_val)) = (self.evaluate_expression(key_expr), self.evaluate_expression(val_expr)) {
                        dict.insert(key_val, actual_val);
                    }
                }
                Value::Dict(dict)
            },
        }
    }

    fn evaluate_variable(&mut self, name: String) -> Result<Value, String> {
        match self.globals.get(&name) {
            Some(Symbol { value, .. }) => Ok(value.clone()),
            None => Err(format!("Variável '{}' não encontrada.", name)),
        }
    }

    fn execute_var_declaration(&mut self, var_decl: VarDecl) -> Result<(), String> {
        let value = self.evaluate_expression(var_decl.value)?;
        self.globals.define(var_decl.name, value, false, true);
        Ok(())
    }

    fn execute_mut_declaration(&mut self, mut_decl: MutDecl) -> Result<(), String> {
        let value = self.evaluate_expression(mut_decl.value)?;
        self.globals.define(mut_decl.name, value, true, true);
        Ok(())
    }

    fn execute_const_declaration(&mut self, const_decl: ConstDecl) -> Result<(), String> {
        let value = self.evaluate_expression(const_decl.value)?;
        self.globals.define(const_decl.name, value, false, false);
        Ok(())
    }

    fn execute_var_assignment(&mut self, var_set: VarSet) -> Result<(), String> {
        let value = self.evaluate_expression(var_set.value)?;
        match self.globals.get_mut(&var_set.name) {
            Some(symbol) => {
                if !symbol.is_reassignable {
                    return Err(format!("Variável '{}' não pode ser reatribuída (é constante).", var_set.name));
                }
                symbol.value = value;
                Ok(())
            },
            None => Err(format!("Variável '{}' não encontrada para atribuição.", var_set.name)),
        }
    }

    fn execute_print_statement(&mut self, expressions: Vec<Expr>) -> Result<(), String> {
        let mut output = String::new();
        for (i, expr) in expressions.iter().enumerate() {
            let value = self.evaluate_expression(expr.clone())?;
            output.push_str(&format!("{}", value));
            if i < expressions.len() - 1 {
                output.push_str(" ");
            }
        }
        println!("{}", output);
        Ok(())
    }

    fn evaluate_unary_expression(&mut self, op: UnaryOp, expr: Expr) -> Result<Value, String> {
        let right = self.evaluate_expression(expr)?;
        match op {
            UnaryOp::Negative => {
                if let Value::Number(n) = right {
                    Ok(Value::Number(-n))
                } else {
                    Err(format!("Operador unário '-' aplicado a tipo não numérico: {:?}", right))
                }
            }
        }
    }

    fn evaluate_binary_expression(&mut self, op: BinaryOp, left: Expr, right: Expr) -> Result<Value, String> {
        let left_val = self.evaluate_expression(left)?;
        let right_val = self.evaluate_expression(right)?;

        match op {
            BinaryOp::Add => self.add_values(left_val, right_val),
            BinaryOp::Subtract => self.subtract_values(left_val, right_val),
            BinaryOp::Multiply => self.multiply_values(left_val, right_val),
            BinaryOp::Divide => self.divide_values(left_val, right_val),
            BinaryOp::Equals => Ok(Value::Boolean(left_val == right_val)),
            BinaryOp::NotEquals => Ok(Value::Boolean(left_val != right_val)),
            BinaryOp::GreaterThan => self.compare_values(left_val, right_val, |a, b| a > b),
            BinaryOp::LessThan => self.compare_values(left_val, right_val, |a, b| a < b),
            BinaryOp::GreaterThanOrEquals => self.compare_values(left_val, right_val, |a, b| a >= b),
            BinaryOp::LessThanOrEquals => self.compare_values(left_val, right_val, |a, b| a <= b),
        }
    }

    fn add_values(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            (Value::String(a), b) => Ok(Value::String(a + &b.to_string())),
            (a, Value::String(b)) => Ok(Value::String(a.to_string() + &b)),
            (l, r) => Err(format!("Operador '+' não suportado para tipos {:?} e {:?}", l, r)),
        }
    }

    fn subtract_values(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (l, r) => Err(format!("Operador '-' não suportado para tipos {:?} e {:?}", l, r)),
        }
    }

    fn multiply_values(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (l, r) => Err(format!("Operador '*' não suportado para tipos {:?} e {:?}", l, r)),
        }
    }

    fn divide_values(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(String::from("Erro de tempo de execução: Divisão por zero."))
                } else {
                    Ok(Value::Number(a / b))
                }
            },
            (l, r) => Err(format!("Operador '/' não suportado para tipos {:?} e {:?}", l, r)),
        }
    }

    fn compare_values<F>(&self, left: Value, right: Value, comparator: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(comparator(a, b))),
            (l, r) => Err(format!("Operadores de comparação não suportados para tipos {:?} e {:?}", l, r)),
        }
    }

    fn execute_conditional_statement(&mut self, conditional: ConditionalStmt) -> Result<(), String> {
        if let Value::Boolean(true) = self.evaluate_expression(conditional.if_block.condition)? {
            self.execute_block(conditional.if_block.body)?;
            return Ok(());
        }

        for block in conditional.elif_blocks {
            if let Value::Boolean(true) = self.evaluate_expression(block.condition)? {
                self.execute_block(block.body)?;
                return Ok(());
            }
        }

        if let Some(else_body) = conditional.else_block {
            self.execute_block(else_body)?;
        }

        Ok(())
    }

    fn execute_loop_statement(&mut self, loop_stmt: LoopStmt) -> Result<(), String> {
        match loop_stmt {
            LoopStmt::While { condition, body } => {
                while let Value::Boolean(true) = self.evaluate_expression(condition.clone())? {
                    self.globals.enter_scope();
                    if let Err(e) = self.execute_block(body.clone()) {
                        self.globals.exit_scope();
                        return Err(e);
                    }
                    self.globals.exit_scope();
                }
                Ok(())
            },
            _ => Err(format!("Loop statement not yet implemented: {:?}", loop_stmt)),
        }
    }

    fn execute_block(&mut self, statements: Program) -> Result<(), String> {
        for statement in statements {
            self.interpret_statement(statement)?;
        }
        Ok(())
    }

    fn execute_func_declaration(&mut self, func_decl: FuncDecl) -> Result<(), String> {
        self.globals.define(func_decl.name.clone(), Value::Function(func_decl), false, false);
        Ok(())
    }

    fn evaluate_function_call(&mut self, callee: Expr, args: Vec<Expr>) -> Result<Value, String> {
        if let ExprKind::PropertyAccess { target, property } = callee.kind {
            let mut evaluated_args = Vec::new();
            for arg_expr in args {
                evaluated_args.push(self.evaluate_expression(arg_expr)?);
            }

            if let ExprKind::Variable(name) = target.kind {
                let symbol = self.globals.get_mut(&name).ok_or_else(|| format!("Variável '{}' não encontrada.", name))?;

                if let Value::List(list) = &mut symbol.value {
                    return match property.as_str() {
                        "push" => {
                            if evaluated_args.len() != 1 {
                                return Err(format!("Método 'push' espera 1 argumento, mas recebeu {}.", evaluated_args.len()));
                            }
                            list.push(evaluated_args[0].clone());
                            Ok(Value::Nil)
                        },
                        _ => Err(format!("Método '{}' não encontrado para o tipo Lista.", property)),
                    };
                } else if let Value::Dict(dict) = &mut symbol.value {
                    return match property.as_str() {
                        "set" => {
                            if evaluated_args.len() != 2 {
                                return Err(format!("Método 'set' espera 2 argumentos, mas recebeu {}.", evaluated_args.len()));
                            }
                            let key = evaluated_args[0].clone();
                            let value = evaluated_args[1].clone();
                            dict.insert(key, value);
                            Ok(Value::Nil)
                        },
                        _ => Err(format!("Método '{}' não encontrado para o tipo Dicionário.", property)),
                    };
                } else {
                    return Err(format!("Não é possível chamar o método '{}' em um valor que não é uma lista ou dicionário.", property));
                }
            } else {
                return Err("Alvos de método complexos (ex: `get_list().push()`) ainda não são suportados.".to_string());
            }
        }

        let func_val = self.evaluate_expression(callee)?;
        if let Value::Function(func_decl) = func_val {
            if args.len() != func_decl.params.len() {
                return Err(format!("Número incorreto de argumentos para a função '{}'. Esperado {}, encontrado {}.", func_decl.name, func_decl.params.len(), args.len()));
            }

            self.globals.enter_scope();
            for (i, (param_name, _param_type)) in func_decl.params.iter().enumerate() {
                let arg_value = self.evaluate_expression(args[i].clone())?;
                self.globals.define(param_name.clone(), arg_value, false, false);
            }
            let result = self.execute_block(func_decl.body.clone());
            self.globals.exit_scope();

            match result {
                Ok(_) => Ok(Value::Nil),
                Err(e) => {
                    if e.starts_with("RETURN_VALUE:") {
                        let return_val_str = e.strip_prefix("RETURN_VALUE:").unwrap();
                        if let Ok(num) = return_val_str.parse::<f64>() {
                            Ok(Value::Number(num))
                        } else if let Ok(bool_val) = return_val_str.parse::<bool>() {
                            Ok(Value::Boolean(bool_val))
                        } else if return_val_str == "nil" {
                            Ok(Value::Nil)
                        } else {
                            Ok(Value::String(return_val_str.to_string()))
                        }
                    } else {
                        Err(e)
                    }
                }
            }
        } else {
            Err(format!("Tentativa de chamar um valor não-invocável: {:?}", func_val))
        }
    }

    fn execute_return_statement(&mut self, expr: Expr) -> Result<(), String> {
        let value = self.evaluate_expression(expr)?;
        Err(format!("RETURN_VALUE:{}", value))
    }
}