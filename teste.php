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
echo "Área do círculo (raio 5): " . Utilitarios::areaCirculo(5.0) . "\n";
echo "25°C em Fahrenheit: " . Utilitarios::celsiusParaFahrenheit(25.0) . "°F\n";
echo "Constante PI: " . Utilitarios::PI . "\n";

echo "\n✅ Todos os testes concluídos!\n";