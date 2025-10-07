// OBRIGATÓRIO para Windows
#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendClassObject;
use ext_php_rs::zend::ModuleEntry;

// ============================================================================
// FUNÇÕES ISOLADAS
// ============================================================================

/// Função clássica Hello World
#[php_function]
pub fn hello_world(name: String) -> String {
    format!("Hello, {}!", name)
}

/// Soma dois números
#[php_function]
pub fn somar(a: i64, b: i64) -> i64 {
    a + b
}

// ============================================================================
// CLASSE PESSOA
// ============================================================================

#[php_class]
#[derive(Debug, Default)]
pub struct Pessoa {
    #[php(prop)]
    nome: String,

    #[php(prop)]
    idade: i32,
    
    #[php(prop)] // sem essa marcação, são propriedades internas, sem possibilidade de getter e setter
    pub cidade: String,
}

#[php_impl]
impl Pessoa {
    const IDADE_MAXIMA: i32 = 150;
    const VERSAO: &'static str = "1.0.0";
    
    // Construtor
    pub fn __construct(nome: String, idade: i32) -> Self {
        Self {
            nome,
            idade,
            cidade: String::from("Desconhecida"),
        }
    }
    
    // Métodos de instância
    pub fn apresentar(&self) -> String {
        format!(
            "Olá! Meu nome é {} e tenho {} anos. Moro em {}.",
            self.nome, self.idade, self.cidade
        )
    }
    
    pub fn fazer_aniversario(&mut self) -> i32 {
        self.idade += 1;
        self.idade
    }
    
    pub fn eh_maior_de_idade(&self) -> bool {
        self.idade >= 18
    }
    
    // Getters e setters
    #[php(getter)]
    pub fn get_nome(&self) -> String {
        self.nome.clone()
    }
    
    #[php(setter)]
    pub fn set_nome(&mut self, nome: String) {
        self.nome = nome;
    }
    
    #[php(getter)]
    pub fn get_idade(&self) -> i32 {
        self.idade
    }
    
    #[php(setter)]
    pub fn set_idade(&mut self, idade: i32) -> Result<(), String> {
        if idade < 0 || idade > Self::IDADE_MAXIMA {
            return Err(format!("Idade deve estar entre 0 e {}", Self::IDADE_MAXIMA));
        }
        self.idade = idade;
        Ok(())
    }
    
    // Métodos estáticos
    pub fn get_idade_maxima() -> i32 {
        Self::IDADE_MAXIMA
    }
    
    pub fn criar_padrao() -> Self {
        Self {
            nome: String::from("João Silva"),
            idade: 30,
            cidade: String::from("São Paulo"),
        }
    }
    
    pub fn validar_nome(nome: String) -> bool {
        !nome.trim().is_empty() && nome.len() >= 2
    }
    
    // Acesso ao objeto Zend
    pub fn get_info_objeto(self_: &ZendClassObject<Pessoa>) -> String {
        format!("Objeto Pessoa - Classe: {}", std::any::type_name::<Pessoa>())
    }
}

// ============================================================================
// CLASSE UTILITARIOS
// ============================================================================

#[php_class]
pub struct Utilitarios;

#[php_impl]
impl Utilitarios {
    const PI: f64 = 3.14159265359;
    
    pub fn area_circulo(raio: f64) -> f64 {
        Self::PI * raio * raio
    }
    
    pub fn celsius_para_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }
}

// ============================================================================
// INTERAÇÃO COM API ZEND
// ============================================================================

#[php_function]
pub fn exemplo_zend_api() -> String {
    let ts_mode = if cfg!(php_zts) { "Thread-Safe" } else { "Non-Thread-Safe" };
    let debug_mode = if cfg!(php_debug) { "Debug" } else { "Release" };
    
    format!(
        "Informações da compilação:\n\
         - Modo: {}\n\
         - Build: {}\n\
         - Versão: {}",
        ts_mode,
        debug_mode,
        env!("CARGO_PKG_VERSION")
    )
}

// ============================================================================
// PHPINFO
// ============================================================================

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    use ext_php_rs::{info_table_start, info_table_row, info_table_end};
    
    info_table_start!();
    info_table_row!("Extensão", "Minha Extensão PHP");
    info_table_row!("Versão", env!("CARGO_PKG_VERSION"));
    info_table_row!("Autor", "Seu Nome");
    info_table_row!("Descrição", "Exemplo completo de extensão PHP em Rust");
    info_table_row!("Status", "Estável");
    info_table_row!("Suporte", "PHP 8.1+");
    info_table_end!();
}

// ============================================================================
// MÓDULO - REGISTRO
// ============================================================================

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(hello_world))
        .function(wrap_function!(somar))
        .function(wrap_function!(exemplo_zend_api))
        .class::<Pessoa>()
        .class::<Utilitarios>()
        .info_function(php_module_info)
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pessoa() {
        let mut pessoa = Pessoa::__construct("João".to_string(), 25);
        assert_eq!(pessoa.get_idade(), 25);
        pessoa.fazer_aniversario();
        assert_eq!(pessoa.get_idade(), 26);
        assert!(pessoa.eh_maior_de_idade());
    }
    
    #[test]
    fn test_funcoes() {
        assert_eq!(somar(2, 3), 5);
        assert_eq!(hello_world("Teste".to_string()), "Hello, Teste!");
    }
}