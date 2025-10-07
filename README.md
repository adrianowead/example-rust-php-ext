# 🚀 Criando Extensões PHP em Rust com ext-php-rs

[![Rust](https://img.shields.io/badge/Rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-8.1%2B-blue.svg)](https://www.php.net/)
[![ext-php-rs](https://img.shields.io/badge/ext--php--rs-0.14-green.svg)](https://docs.rs/ext-php-rs)

Um guia completo e prático para criar extensões PHP nativas usando Rust e a biblioteca `ext-php-rs`.

---

## 📚 Índice

- [Introdução](#-introdução)
- [Por que Rust para PHP?](#-por-que-rust-para-php)
- [Requisitos](#-requisitos)
- [Instalação e Configuração](#-instalação-e-configuração)
- [Estrutura do Projeto](#-estrutura-do-projeto)
- [Conceitos Fundamentais](#-conceitos-fundamentais)
- [Código Completo](#-código-completo)
- [Compilação](#-compilação)
- [Instalação da Extensão](#-instalação-da-extensão)
- [Testando](#-testando)
- [Troubleshooting](#-troubleshooting)
- [Recursos Avançados](#-recursos-avançados)
- [Referências](#-referências)

---

## 🎯 Introdução

Este guia demonstra como criar uma extensão PHP completa em Rust, incluindo:

- ✅ Funções isoladas
- ✅ Classes customizadas
- ✅ Métodos de instância e estáticos
- ✅ Propriedades e constantes
- ✅ Getters e setters
- ✅ Construtores
- ✅ Integração com phpinfo()
- ✅ Interação com a API Zend

---

## 💡 Por que Rust para PHP?

### Vantagens

| Aspecto | Rust | C/C++ |
|---------|------|-------|
| **Segurança de memória** | ✅ Garantida em tempo de compilação | ❌ Manual, propenso a erros |
| **Performance** | ✅ Equivalente a C | ✅ Excelente |
| **Curva de aprendizado** | 🟡 Moderada | 🔴 Íngreme |
| **Ecossistema moderno** | ✅ Cargo, crates.io | 🟡 Fragmentado |
| **Concorrência** | ✅ Sem data races | ❌ Complexo e perigoso |
| **Produtividade** | ✅ Alta | 🟡 Média |

### Casos de Uso Ideais

- 🔥 **Performance crítica**: Processamento pesado, algoritmos complexos
- 🔒 **Segurança**: Manipulação de dados sensíveis, criptografia
- 🎮 **Computação intensiva**: Machine learning, processamento de imagem
- 📦 **Bibliotecas existentes**: Integrar crates Rust no PHP
- 🌐 **Protocolos nativos**: Implementações de baixo nível

---

## 📋 Requisitos

### Software Necessário

#### Windows
- ✅ **Rust nightly**: `rustup default nightly`
- ✅ **PHP 8.1+**: Thread-Safe ou Non-Thread-Safe
- ✅ **PHP Development Pack**: SDK para compilar extensões
- ✅ **Visual Studio Build Tools**: MSVC ou Clang
- ✅ **Git** (opcional)

#### Linux
- ✅ **Rust stable ou nightly**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- ✅ **PHP 8.1+**: Com headers de desenvolvimento (`php-dev`)
- ✅ **GCC ou Clang**
- ✅ **Make**

#### macOS
- ✅ **Rust stable ou nightly**
- ✅ **PHP 8.1+**: Homebrew ou MacPorts
- ✅ **Xcode Command Line Tools**

### Verificar Instalação

```bash
# Rust
rustc --version
cargo --version

# PHP
php --version
php-config --version  # Linux/macOS

# Windows: verificar Thread Safety
php -i | findstr "Thread"

# Linux/macOS: verificar Thread Safety
php -i | grep "Thread"
```

---

## 🛠️ Instalação e Configuração

### 1. Instalar Rust

#### Windows
```powershell
# Baixar e instalar: https://rustup.rs/
# Depois, configurar nightly
rustup install nightly
rustup default nightly
```

#### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Baixar PHP Development Pack (Windows)

Acesse: https://windows.php.net/download/

Escolha baseado em sua configuração:

```powershell
# Descobrir sua configuração
php -v
php -i | findstr "Thread"
php -i | findstr "Architecture"
```

| Configuração | Download |
|-------------|----------|
| x64 + Thread-Safe | `php-devel-pack-8.3.x-Win32-vs16-x64.zip` |
| x64 + Non-Thread-Safe | `php-devel-pack-8.3.x-nts-Win32-vs16-x64.zip` |
| x86 + Thread-Safe | `php-devel-pack-8.3.x-Win32-vs16-x86.zip` |
| x86 + Non-Thread-Safe | `php-devel-pack-8.3.x-nts-Win32-vs16-x86.zip` |

**Extraia** para uma pasta (ex: `C:\php-8.3-sdk`)

### 3. Configurar Variáveis de Ambiente (Windows)

```powershell
# Temporário (sessão atual)
$env:PHP_LIB = "C:\php-8.3-sdk\lib"

# Permanente
[Environment]::SetEnvironmentVariable("PHP_LIB", "C:\php-8.3-sdk\lib", "User")
```

### 4. Configurar Linker (Windows)

Crie `.cargo\config.toml` no diretório do usuário:

```toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[target.i686-pc-windows-msvc]
linker = "rust-lld"
```

---

## 📁 Estrutura do Projeto

```
minha_extensao/
├── .cargo/
│   └── config.toml          # Configuração do linker (Windows)
├── src/
│   └── lib.rs               # Código principal
├── Cargo.toml               # Configuração do projeto
├── rust-toolchain.toml      # Versão do Rust (nightly para Windows)
├── README.md                # Documentação
└── stubs.php                # Stubs para IDEs
```

### Criar Projeto

```bash
cargo new --lib minha_extensao
cd minha_extensao
```

---

## ⚙️ Configuração do Projeto

### Cargo.toml

```toml
[package]
name = "minha_extensao"
version = "1.0.0"
edition = "2021"
authors = ["Seu Nome <seu@email.com>"]
description = "Extensão PHP em Rust"
license = "MIT"

[lib]
crate-type = ["cdylib"]

[dependencies]
ext-php-rs = "0.14"

[profile.release]
strip = true        # Remove símbolos de debug
lto = true          # Link-Time Optimization
codegen-units = 1   # Otimização máxima
opt-level = 3       # Nível de otimização
```

### rust-toolchain.toml (Windows)

```toml
[toolchain]
channel = "nightly"
```

### .cargo/config.toml (Windows)

```toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[target.i686-pc-windows-msvc]
linker = "rust-lld"
```

---

## 🧩 Conceitos Fundamentais

### Macros Principais

| Macro | Uso | Descrição |
|-------|-----|-----------|
| `#[php_function]` | Função | Exporta função para PHP |
| `#[php_class]` | Struct | Define classe PHP |
| `#[php_impl]` | impl | Implementa métodos da classe |
| `#[php_module]` | Função | Ponto de entrada do módulo |
| `#[php(prop)]` | Campo | Expõe propriedade pública |
| `#[php(getter)]` | Método | Define getter |
| `#[php(setter)]` | Método | Define setter |

### Tipos de Dados

| Rust | PHP | Exemplo |
|------|-----|---------|
| `String` | `string` | `"texto"` |
| `i32`, `i64` | `int` | `42` |
| `f32`, `f64` | `float` | `3.14` |
| `bool` | `bool` | `true` |
| `Vec<T>` | `array` | `[1, 2, 3]` |
| `HashMap<K,V>` | `array` | `['a' => 1]` |
| `Option<T>` | `?T` ou `null` | `Some(5)` / `None` |
| `Result<T,E>` | `T` ou exception | `Ok(5)` / `Err(e)` |

### Convenções de Nomenclatura

Rust usa `snake_case`, mas o PHP usa `camelCase`:

```rust
// Rust
pub fn fazer_aniversario(&mut self) -> i32 { ... }

// PHP (convertido automaticamente)
$pessoa->fazerAniversario();
```

Para manter o nome original:
```rust
#[php(name = "fazer_aniversario")]
pub fn fazer_aniversario(&mut self) -> i32 { ... }
```

---

## 💻 Código Completo

### src/lib.rs

```rust
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
    nome: String,
    idade: i32,
    
    #[php(prop)]
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
```

---

## 🔨 Compilação

### Comandos Básicos

```bash
# Compilação release (otimizada)
cargo build --release

# Compilação debug (desenvolvimento)
cargo build

# Limpar builds anteriores
cargo clean

# Executar testes
cargo test
```

### Compilação por Plataforma

#### Windows (x64, Non-Thread-Safe)
```powershell
rustup default nightly
cargo build --release --target x86_64-pc-windows-msvc
```

#### Windows (x64, Thread-Safe)
```powershell
rustup default nightly
cargo build --release --target x86_64-pc-windows-msvc
```

#### Windows (x86, 32-bit)
```powershell
rustup default nightly
cargo build --release --target i686-pc-windows-msvc
```

#### Linux (x64)
```bash
cargo build --release
```

#### macOS (Universal Binary)
```bash
cargo build --release
```

### Localização dos Arquivos

```
target/
├── release/
│   ├── minha_extensao.dll      # Windows
│   ├── libminha_extensao.so    # Linux
│   └── libminha_extensao.dylib # macOS
└── debug/
    └── (mesma estrutura)
```

---

## 📦 Instalação da Extensão

### Windows

```powershell
# 1. Localizar diretório de extensões
$phpExt = php -r "echo ini_get('extension_dir');"
Write-Host "Diretório de extensões: $phpExt"

# 2. Copiar DLL
Copy-Item target\release\minha_extensao.dll "$phpExt\minha_extensao.dll" -Force

# 3. Localizar php.ini
$phpIni = php --ini | Select-String "Loaded Configuration File" | ForEach-Object { $_.ToString().Split(":")[1].Trim() }
Write-Host "Arquivo php.ini: $phpIni"

# 4. Adicionar extensão ao php.ini
Add-Content $phpIni "`nextension=minha_extensao"

# 5. Verificar
php -m | Select-String "minha_extensao"
```

### Linux (Ubuntu/Debian)

```bash
# 1. Copiar extensão
sudo cp target/release/libminha_extensao.so $(php-config --extension-dir)/minha_extensao.so

# 2. Criar arquivo de configuração
echo "extension=minha_extensao.so" | sudo tee /etc/php/8.3/mods-available/minha_extensao.ini

# 3. Ativar extensão
sudo phpenmod minha_extensao

# 4. Reiniciar PHP-FPM (se aplicável)
sudo systemctl restart php8.3-fpm

# 5. Verificar
php -m | grep minha_extensao
```

### macOS

```bash
# 1. Localizar diretório de extensões
phpExt=$(php-config --extension-dir)
echo "Diretório: $phpExt"

# 2. Copiar extensão
sudo cp target/release/libminha_extensao.dylib "$phpExt/minha_extensao.so"

# 3. Editar php.ini
phpIni=$(php --ini | grep "Loaded Configuration File" | cut -d: -f2 | xargs)
echo "extension=minha_extensao.so" | sudo tee -a "$phpIni"

# 4. Verificar
php -m | grep minha_extensao
```

---

## ✅ Testando

### Verificar Instalação

```bash
# Listar extensões carregadas
php -m

# Verificar funções exportadas
php -r "print_r(get_extension_funcs('minha_extensao'));"

# Ver informações no phpinfo
php -r "phpinfo();" | grep -A 10 "Minha Extensão"
```

### Script de Teste (teste.php)

```php
<?php

echo "=== Testando Extensão ===\n\n";

// Verificar se está carregada
if (!extension_loaded('minha_extensao')) {
    die("❌ Extensão não carregada!\n");
}
echo "✅ Extensão carregada!\n\n";

// Testar funções isoladas
echo "=== FUNÇÕES ===\n";
echo hello_world("Mundo") . "\n";
echo "2 + 3 = " . somar(2, 3) . "\n";
echo exemplo_zend_api() . "\n\n";

// Testar classe Pessoa
echo "=== CLASSE PESSOA ===\n";
$pessoa = new Pessoa("Maria", 25);
$pessoa->cidade = "Rio de Janeiro";

echo $pessoa->apresentar() . "\n";
echo "É maior de idade? " . ($pessoa->ehMaiorDeIdade() ? "Sim" : "Não") . "\n";

// Getter/Setter
echo "Nome: " . $pessoa->nome . "\n";
$pessoa->nome = "Maria Silva";
echo "Novo nome: " . $pessoa->nome . "\n";

// Fazer aniversário
echo "Idade atual: " . $pessoa->idade . "\n";
$pessoa->fazerAniversario();
echo "Nova idade: " . $pessoa->idade . "\n";

// Métodos estáticos
echo "\n=== MÉTODOS ESTÁTICOS ===\n";
echo "Idade máxima: " . Pessoa::getIdadeMaxima() . "\n";
echo "Constante: " . Pessoa::IDADE_MAXIMA . "\n";

$pessoaPadrao = Pessoa::criarPadrao();
echo $pessoaPadrao->apresentar() . "\n";

echo "Nome 'Jo' é válido? " . (Pessoa::validarNome("Jo") ? "Sim" : "Não") . "\n";

// Testar classe Utilitarios
echo "\n=== UTILITÁRIOS ===\n";
echo "Área do círculo (raio 5): " . Utilitarios::areaCirculo(5) . "\n";
echo "25°C em Fahrenheit: " . Utilitarios::celsiusParaFahrenheit(25) . "°F\n";
echo "Constante PI: " . Utilitarios::PI . "\n";

echo "\n✅ Todos os testes concluídos!\n";
```

Executar:
```bash
php teste.php
```

Saída esperada:
```
=== Testando Extensão ===

✅ Extensão carregada!

=== FUNÇÕES ===
Hello, Mundo!
2 + 3 = 5
Informações da compilação:
 - Modo: Non-Thread-Safe
 - Build: Release
 - Versão: 1.0.0

=== CLASSE PESSOA ===
Olá! Meu nome é Maria e tenho 25 anos. Moro em Rio de Janeiro.
É maior de idade? Sim
Nome: Maria
Novo nome: Maria Silva
Idade atual: 25
Nova idade: 26

=== MÉTODOS ESTÁTICOS ===
Idade máxima: 150
Constante: 150
Olá! Meu nome é João Silva e tenho 30 anos. Moro em São Paulo.
Nome 'Jo' é válido? Sim

=== UTILITÁRIOS ===
Área do círculo (raio 5): 78.539816339745
25°C em Fahrenheit: 77°F
Constante PI: 3.14159265359

✅ Todos os testes concluídos!
```

---

## 🐛 Troubleshooting

### Erro: "Cannot load extension"

**Causa**: Incompatibilidade de Thread Safety ou arquitetura

**Solução**:
```bash
# Verificar PHP
php -i | grep "Thread Safety"
php -i | grep "Architecture"

# Recompilar para a configuração correta
cargo clean
cargo build --release
```

### Erro: "Call to undefined function"

**Causa**: Funções não foram registradas corretamente

**Solução**:
1. Verificar se `wrap_function!` está sendo usado
2. Recompilar e reinstalar a extensão
3. Verificar funções exportadas:
```bash
php -r "print_r(get_extension_funcs('minha_extensao'));"
```

### Erro: "feature(abi_vectorcall)" (Windows)

**Causa**: Usando Rust stable ao invés de nightly

**Solução**:
```powershell
rustup install nightly
rustup default nightly
```

### Erro: "PHP_LIB not found" (Windows)

**Causa**: Variável de ambiente não configurada

**Solução**:
```powershell
$env:PHP_LIB = "C:\caminho\para\php-devel-pack\lib"
[Environment]::SetEnvironmentVariable("PHP_LIB", "C:\caminho\para\php-devel-pack\lib", "User")
```

### Erro: "Thread Safety mismatch"

**Causa**: Extensão compilada para TS mas PHP é NTS (ou vice-versa)

**Solução**: O tipo é detectado automaticamente. Certifique-se de estar usando o PHP Development Pack correto.

### Erro de Segmentation Fault

**Causa**: Geralmente problemas com lifetime ou referências

**Solução**:
1. Revisar código Rust para unsafe operations
2. Verificar se está usando `&self` corretamente
3. Testar em modo debug: `cargo build`
4. Usar ferramentas de debug: `valgrind` (Linux) ou `drmemory` (Windows)

### Cache do PHP

Se mudanças não aparecerem:
```bash
# Limpar opcache
php -r "opcache_reset();"

# Reiniciar PHP-FPM (Linux)
sudo systemctl restart php8.3-fpm

# Reiniciar Apache (se aplicável)
sudo systemctl restart apache2
```

---

## 🚀 Recursos Avançados

### 1. Tratamento de Erros

```rust
#[php_function]
pub fn dividir(a: i64, b: i64) -> Result<f64, String> {
    if b == 0 {
        return Err("Divisão por zero!".to_string());
    }
    Ok(a as f64 / b as f64)
}
```

No PHP, `Err` se torna uma Exception:
```php
try {
    echo dividir(10, 0);
} catch (Exception $e) {
    echo "Erro: " . $e->getMessage();
}
```

### 2. Parâmetros Opcionais

```rust
#[php_function]
pub fn saudar(nome: String, saudacao: Option<String>) -> String {
    let saudacao = saudacao.unwrap_or_else(|| "Olá".to_string());
    format!("{}, {}!", saudacao, nome)
}
```

```php
echo saudar("Maria");              // Olá, Maria!
echo saudar("Maria", "Bom dia");   // Bom dia, Maria!
```

### 3. Arrays e Vetores

```rust
#[php_function]
pub fn somar_array(numeros: Vec<i64>) -> i64 {
    numeros.iter().sum()
}
```

```php
echo somar_array([1, 2, 3, 4, 5]); // 15
```

### 4. HashMaps (Arrays Associativos)

```rust
use std::collections::HashMap;

#[php_function]
pub fn contar_palavras(texto: String) -> HashMap<String, i32> {
    let mut contador = HashMap::new();
    
    for palavra in texto.split_whitespace() {
        *contador.entry(palavra.to_string()).or_insert(0) += 1;
    }
    
    contador
}
```

```php
$resultado = contar_palavras("hello world hello");
print_r($resultado);
// Array ( [hello] => 2 [world] => 1 )
```

### 5. Callbacks (Closures)

```rust
use ext_php_rs::types::Callable;

#[php_function]
pub fn executar_callback(callback: Callable, valor: i64) -> i64 {
    callback.call([valor]).ok().and_then(|r| r.long()).unwrap_or(0)
}
```

```php
$resultado = executar_callback(fn($x) => $x * 2, 5);
echo $resultado; // 10
```

### 6. Herança (Traits)

```rust
#[php_class]
pub struct Animal {
    #[php(prop)]
    pub nome: String,
}

#[php_impl]
impl Animal {
    pub fn fazer_som(&self) -> String {
        "Som genérico".to_string()
    }
}

#[php_class(extends = "Animal")]
pub struct Cachorro;

#[php_impl]
impl Cachorro {
    pub fn fazer_som(&self) -> String {
        "Au au!".to_string()
    }
}
```

### 7. Constantes Globais

```rust
#[php_const]
pub const VERSAO_API: &str = "1.0.0";

#[php_const]
pub const MAX_CONEXOES: i32 = 100;
```

```php
echo VERSAO_API;      // 1.0.0
echo MAX_CONEXOES;    // 100
```

### 8. Acesso a Variáveis Globais PHP

```rust
use ext_php_rs::types::Zval;

#[php_function]
pub fn ler_global(nome: String) -> Option<String> {
    use ext_php_rs::zend::globals;
    
    unsafe {
        globals()
            .get(nome.as_str())
            .and_then(|zval| zval.string())
            .map(|s| s.to_string())
    }
}
```

```php
$GLOBALS['minha_var'] = "teste";
echo ler_global('minha_var'); // teste
```

### 9. Trabalhando com Referências

```rust
#[php_function]
pub fn incrementar_referencia(valor: &mut i64) {
    *valor += 1;
}
```

```php
$num = 5;
incrementar_referencia($num);
echo $num; // 6
```

### 10. Namespace PHP

```rust
#[php_class(name = "MinhaEmpresa\\MinhaClasse")]
pub struct MinhaClasse;
```

```php
use MinhaEmpresa\MinhaClasse;
$obj = new MinhaClasse();
```

---

## 📊 Performance e Otimização

### Benchmarking

Crie `benches/benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minha_extensao::*;

fn bench_somar(c: &mut Criterion) {
    c.bench_function("somar", |b| {
        b.iter(|| somar(black_box(2), black_box(3)))
    });
}

criterion_group!(benches, bench_somar);
criterion_main!(benches);
```

Adicione ao `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "benchmark"
harness = false
```

Execute:
```bash
cargo bench
```

### Otimizações de Compilação

#### Profile Personalizado (Cargo.toml)

```toml
[profile.release-with-debug]
inherits = "release"
debug = true
strip = false

[profile.release-lto]
inherits = "release"
lto = "fat"
codegen-units = 1
panic = "abort"
```

Compilar:
```bash
cargo build --profile release-lto
```

### Dicas de Performance

1. **Use `&str` ao invés de `String`** quando possível
2. **Evite clones desnecessários** com `.clone()`
3. **Use `Vec::with_capacity()`** se souber o tamanho
4. **Prefira iteradores** ao invés de loops manuais
5. **Profile com `cargo flamegraph`** para identificar gargalos

---

## 🔒 Segurança

### Validação de Entrada

```rust
#[php_function]
pub fn processar_email(email: String) -> Result<String, String> {
    // Validação básica
    if !email.contains('@') || email.len() > 255 {
        return Err("Email inválido".to_string());
    }
    
    // Sanitização
    let email_limpo = email.trim().to_lowercase();
    
    Ok(format!("Email processado: {}", email_limpo))
}
```

### Prevenção de Injection

```rust
#[php_function]
pub fn escapar_sql(valor: String) -> String {
    valor
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('"', "\\\"")
        .replace('\0', "\\0")
}
```

### Limites de Recursos

```rust
const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB

#[php_function]
pub fn processar_dados(dados: Vec<u8>) -> Result<String, String> {
    if dados.len() > MAX_BUFFER_SIZE {
        return Err("Dados excedem o limite de 1MB".to_string());
    }
    
    // Processar...
    Ok("Dados processados".to_string())
}
```

---

## 🧪 Testes Unitários

### Estrutura de Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funcoes_basicas() {
        assert_eq!(somar(2, 3), 5);
        assert_eq!(somar(-1, 1), 0);
    }
    
    #[test]
    fn test_pessoa_construtor() {
        let pessoa = Pessoa::__construct("João".to_string(), 25);
        assert_eq!(pessoa.get_nome(), "João");
        assert_eq!(pessoa.get_idade(), 25);
    }
    
    #[test]
    fn test_pessoa_aniversario() {
        let mut pessoa = Pessoa::__construct("Maria".to_string(), 17);
        assert!(!pessoa.eh_maior_de_idade());
        
        pessoa.fazer_aniversario();
        assert!(pessoa.eh_maior_de_idade());
    }
    
    #[test]
    fn test_validacao_idade() {
        let mut pessoa = Pessoa::__construct("Ana".to_string(), 30);
        
        // Idade válida
        assert!(pessoa.set_idade(40).is_ok());
        
        // Idade inválida
        assert!(pessoa.set_idade(-5).is_err());
        assert!(pessoa.set_idade(200).is_err());
    }
    
    #[test]
    fn test_utilitarios() {
        let area = Utilitarios::area_circulo(1.0);
        assert!((area - std::f64::consts::PI).abs() < 0.0001);
        
        let f = Utilitarios::celsius_para_fahrenheit(0.0);
        assert_eq!(f, 32.0);
    }
}
```

### Executar Testes

```bash
# Todos os testes
cargo test

# Com output detalhado
cargo test -- --nocapture

# Teste específico
cargo test test_pessoa_aniversario

# Com threads paralelas
cargo test -- --test-threads=1
```

### Cobertura de Código

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Gerar relatório
cargo tarpaulin --out Html
```

---

## 📝 Documentação

### Gerar Documentação

```bash
# Documentação do projeto
cargo doc --open

# Incluir dependências
cargo doc --open --document-private-items
```

### Documentação no Código

```rust
/// Calcula o fatorial de um número
///
/// # Exemplos
///
/// ```
/// use minha_extensao::fatorial;
/// assert_eq!(fatorial(5), 120);
/// ```
///
/// # Panics
///
/// Entra em pânico se `n > 20` devido a overflow
///
/// # Erros
///
/// Retorna erro se `n` for negativo
#[php_function]
pub fn fatorial(n: i64) -> Result<i64, String> {
    if n < 0 {
        return Err("Número não pode ser negativo".to_string());
    }
    
    if n > 20 {
        panic!("Overflow! Use números menores que 20");
    }
    
    Ok((1..=n).product())
}
```

---

## 🐳 Docker

### Dockerfile para Desenvolvimento

```dockerfile
FROM rust:latest

# Instalar PHP e dependências
RUN apt-get update && apt-get install -y \
    php8.3 \
    php8.3-dev \
    php8.3-cli \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Configurar diretório de trabalho
WORKDIR /app

# Copiar código
COPY . .

# Compilar extensão
RUN cargo build --release

# Instalar extensão
RUN cp target/release/libminha_extensao.so \
    $(php-config --extension-dir)/minha_extensao.so \
    && echo "extension=minha_extensao.so" > /etc/php/8.3/cli/conf.d/99-minha_extensao.ini

# Comando padrão
CMD ["php", "-m"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  dev:
    build: .
    volumes:
      - .:/app
    working_dir: /app
    command: bash
    
  test:
    build: .
    volumes:
      - .:/app
    working_dir: /app
    command: php teste.php
```

Uso:
```bash
docker-compose build
docker-compose run test
```

---

## 🔄 CI/CD

### GitHub Actions

`.github/workflows/build.yml`:

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        php: ['8.1', '8.2', '8.3']
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup PHP
      uses: shivammathur/setup-php@v2
      with:
        php-version: ${{ matrix.php }}
        extensions: none
        tools: php-config
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
        override: true
    
    - name: Build
      run: cargo build --release
    
    - name: Run tests
      run: cargo test
    
    - name: Install extension (Linux)
      if: runner.os == 'Linux'
      run: |
        sudo cp target/release/libminha_extensao.so \
          $(php-config --extension-dir)/minha_extensao.so
        echo "extension=minha_extensao.so" | \
          sudo tee /etc/php/${{ matrix.php }}/cli/conf.d/99-minha.ini
    
    - name: Verify installation
      run: php -m | grep minha_extensao
    
    - name: Run PHP tests
      run: php teste.php
```

---

## 📦 Distribuição

### Criando Releases

#### 1. Automatizar Builds Multi-Plataforma

`.github/workflows/release.yml`:

```yaml
name: Release

on:
  release:
    types: [created]

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            ext: so
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            ext: dll
          - os: macos-latest
            target: x86_64-apple-darwin
            ext: dylib
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
        target: ${{ matrix.target }}
        override: true
    
    - name: Build
      run: cargo build --release --target ${{ matrix.target }}
    
    - name: Upload artifact
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ github.event.release.upload_url }}
        asset_path: ./target/${{ matrix.target }}/release/libminha_extensao.${{ matrix.ext }}
        asset_name: minha_extensao-${{ matrix.target }}.${{ matrix.ext }}
        asset_content_type: application/octet-stream
```

#### 2. Script de Instalação

`install.sh` (Linux/macOS):

```bash
#!/bin/bash

set -e

VERSION="1.0.0"
ARCH=$(uname -m)
OS=$(uname -s)

echo "Instalando minha_extensao v${VERSION} para ${OS} ${ARCH}"

# Download
URL="https://github.com/usuario/minha_extensao/releases/download/v${VERSION}/minha_extensao-${OS}-${ARCH}.so"
curl -L "$URL" -o minha_extensao.so

# Instalar
EXT_DIR=$(php-config --extension-dir)
sudo cp minha_extensao.so "${EXT_DIR}/"
echo "extension=minha_extensao.so" | sudo tee /etc/php/8.3/mods-available/minha_extensao.ini

# Ativar
sudo phpenmod minha_extensao

# Verificar
php -m | grep minha_extensao && echo "✅ Instalação concluída!"
```

`install.ps1` (Windows):

```powershell
$VERSION = "1.0.0"
$ARCH = "x86_64"

Write-Host "Instalando minha_extensao v$VERSION"

# Download
$URL = "https://github.com/usuario/minha_extensao/releases/download/v$VERSION/minha_extensao-$ARCH.dll"
Invoke-WebRequest $URL -OutFile minha_extensao.dll

# Instalar
$phpExt = php -r "echo ini_get('extension_dir');"
Copy-Item minha_extensao.dll "$phpExt\minha_extensao.dll"

# Configurar
$phpIni = php --ini | Select-String "Loaded Configuration File" | ForEach-Object { $_.ToString().Split(":")[1].Trim() }
Add-Content $phpIni "`nextension=minha_extensao"

# Verificar
php -m | Select-String "minha_extensao"
Write-Host "✅ Instalação concluída!"
```

---

## 📚 Recursos e Referências

### Documentação Oficial

- **ext-php-rs**: https://docs.rs/ext-php-rs
- **Guia ext-php-rs**: https://davidcole1340.github.io/ext-php-rs
- **PHP Internals Book**: https://www.phpinternalsbook.com/
- **Rust Book**: https://doc.rust-lang.org/book/

### Exemplos de Projetos

- **anonaddy-sequoia**: https://gitlab.com/willbrowning/anonaddy-sequoia
- **opus-php**: https://github.com/davidcole1340/opus-php
- **php-scrypt**: https://github.com/appwrite/php-scrypt
- **php-rocksdb**: https://github.com/s00d/php-rocksdb-rc

### Comunidade

- **GitHub**: https://github.com/davidcole1340/ext-php-rs
- **Discord Rust**: https://discord.gg/rust-lang
- **Reddit r/rust**: https://reddit.com/r/rust
- **PHP Internals**: https://externals.io/

### Ferramentas Úteis

- **cargo-php**: https://crates.io/crates/cargo-php
- **cargo-watch**: Recompila automaticamente
  ```bash
  cargo install cargo-watch
  cargo watch -x build
  ```
- **cargo-expand**: Expande macros
  ```bash
  cargo install cargo-expand
  cargo expand
  ```
- **rust-analyzer**: LSP para IDEs

---

## 🎓 Próximos Passos

### Aprendizado Avançado

1. **Async/Await**: Operações assíncronas em extensões
2. **FFI**: Integrar bibliotecas C existentes
3. **Macros Procedurais**: Criar suas próprias macros
4. **Unsafe Rust**: Quando e como usar corretamente
5. **Profiling**: Otimizar performance com `perf`, `flamegraph`

### Projetos Práticos

1. **Parser JSON rápido**: Alternativa ao json_decode()
2. **Compressor de imagens**: Com `image` crate
3. **Cliente HTTP**: Com `reqwest` crate
4. **Encriptação**: Com `ring` ou `sodiumoxide`
5. **Parser Markdown**: Com `pulldown-cmark`

### Contribuindo

Se encontrar bugs ou quiser melhorar a biblioteca:

1. Fork o repositório
2. Crie uma branch: `git checkout -b minha-feature`
3. Commit: `git commit -am 'Adiciona feature X'`
4. Push: `git push origin minha-feature`
5. Abra um Pull Request

---

## ❓ FAQ

### P: Preciso realmente usar nightly no Windows?

**R**: Sim, devido ao `abi_vectorcall`. No Linux/macOS, stable funciona.

### P: A extensão funciona em produção?

**R**: Sim! Muitos projetos usam ext-php-rs em produção. Teste bem antes de deployar.

### P: Como debugar problemas?

**R**: Use `cargo build` (sem `--release`) e ferramentas como `gdb` (Linux) ou `lldb` (macOS).

### P: Posso misturar PHP e Rust no mesmo arquivo?

**R**: Não. Rust compila para binário nativo, PHP interpreta código.

### P: Como atualizar a extensão?

**R**: Recompile e reinstale. Em produção, reinicie PHP-FPM/Apache.

### P: Qual versão do PHP é suportada?

**R**: PHP 8.1+ oficialmente. Versões antigas não têm suporte.

### P: A extensão funciona com Composer?

**R**: Sim, mas você precisa garantir que esteja instalada no sistema.

### P: Como distribuir a extensão?

**R**: Via PECL (complexo) ou releases no GitHub com binários pré-compilados.

---

## 📄 Licença

Este guia é fornecido sob a licença MIT. Você é livre para usar, modificar e distribuir.

```
MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🙏 Agradecimentos

- **David Cole** - Criador do ext-php-rs
- **Comunidade Rust** - Ferramentas e suporte
- **PHP Core Team** - Zend Engine

---

## 📞 Suporte

- **Issues**: https://github.com/davidcole1340/ext-php-rs/issues
- **Discussões**: https://github.com/davidcole1340/ext-php-rs/discussions
- **Email**: seu@email.com

---

**Feito com ❤️ e Rust 🦀**
