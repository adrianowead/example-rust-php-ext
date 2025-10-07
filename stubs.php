<?php
// Stubs para minha_extensao v1.0.0
// Gerado para ext-php-rs 0.14

// ============================================================================
// FUNÇÕES
// ============================================================================

/**
 * Função clássica Hello World
 * @param string $name Nome da pessoa
 * @return string Mensagem de saudação
 */
function hello_world(string $name): string {}

/**
 * Soma dois números
 * @param int $a Primeiro número
 * @param int $b Segundo número
 * @return int Resultado da soma
 */
function somar(int $a, int $b): int {}

/**
 * Exemplo de função que interage com informações da compilação
 * @return string Informações da compilação
 */
function exemplo_zend_api(): string {}

// ============================================================================
// CLASSE PESSOA
// ============================================================================

class Pessoa {
    /** @var string Cidade onde a pessoa mora */
    public string $cidade;
    public int $idade;
    public string $nome;
    
    // Constantes
    public const int IDADE_MAXIMA = 150;
    public const string VERSAO = "1.0.0";
    
    /**
     * Construtor da classe Pessoa
     * @param string $nome Nome da pessoa
     * @param int $idade Idade da pessoa
     */
    public function __construct(string $nome, int $idade) {}
    
    /**
     * Apresenta a pessoa
     * @return string Apresentação formatada
     */
    public function apresentar(): string {}
    
    /**
     * Faz aniversário (incrementa idade)
     * @return int Nova idade
     */
    public function fazerAniversario(): int {}
    
    /**
     * Verifica se é maior de idade
     * @return bool True se maior de idade
     */
    public function ehMaiorDeIdade(): bool {}
    
    /**
     * Retorna a idade máxima permitida
     * @return int Idade máxima
     */
    public static function getIdadeMaxima(): int {}
    
    /**
     * Cria uma pessoa com dados padrão
     * @return Pessoa Nova instância
     */
    public static function criarPadrao(): Pessoa {}
    
    /**
     * Valida se um nome é válido
     * @param string $nome Nome a validar
     * @return bool True se válido
     */
    public static function validarNome(string $nome): bool {}
    
    /**
     * Retorna informações do objeto interno
     * @return string Informações do objeto
     */
    public function getInfoObjeto(): string {}
}

// ============================================================================
// CLASSE UTILITARIOS
// ============================================================================

class Utilitarios {
    // Constantes
    public const float PI = 3.14159265359;
    
    /**
     * Calcula a área do círculo
     * @param float $raio Raio do círculo
     * @return float Área calculada
     */
    public static function areaCirculo(float $raio): float {}
    
    /**
     * Converte Celsius para Fahrenheit
     * @param float $celsius Temperatura em Celsius
     * @return float Temperatura em Fahrenheit
     */
    public static function celsiusParaFahrenheit(float $celsius): float {}
}