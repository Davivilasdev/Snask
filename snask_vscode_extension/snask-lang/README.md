# snask-lang README

Esta é a extensão para a linguagem Snask, desenvolvida para proporcionar uma experiência de desenvolvimento rica e eficiente no Visual Studio Code.

## Funcionalidades

Esta extensão oferece um conjunto completo de ferramentas para trabalhar com arquivos Snask (`.snask`):

*   **Execução de Arquivos:** Rode seus arquivos Snask diretamente do editor ou via comando.
*   **Realce de Sintaxe:** Colorização completa da sintaxe para melhorar a legibilidade do código.
*   **Diagnósticos em Tempo Real:** Receba feedback instantâneo sobre erros de sintaxe e outros problemas enquanto você digita.
*   **Snippets de Código:** Atalhos para inserir rapidamente estruturas comuns da linguagem (condicionais, loops, funções, etc.).
*   **Informações ao Passar o Mouse (Hover Info):** Obtenha descrições e exemplos de palavras-chave e funções nativas da linguagem.
*   **Autocompletar (IntelliSense):** Sugestões inteligentes de código enquanto você digita para acelerar o desenvolvimento.
*   **Formatação de Código:** Formate seu documento Snask com um comando para manter o código limpo e padronizado.
*   **Snask AI Explica (Recurso Único!):** Selecione um trecho de código Snask e use o comando de contexto (`clique direito`) para obter uma análise e explicação gerada por IA sobre o que o código faz.

## Requisitos

*   Visual Studio Code
*   Um executável da linguagem Snask (`snask.exe`) disponível no seu sistema ou configurado para a extensão.

## Configurações da Extensão

Esta extensão contribui com as seguintes configurações:

*   `snask.executablePath`: Caminho completo para o executável do Snask (ex: `/caminho/para/executor/snask.exe`). Se não for definido, a extensão tentará encontrá-lo na pasta `executor` do seu workspace.

## Ativar o Ícone de Arquivo Personalizado

Para ver o ícone da logo do Snask nos seus arquivos `.snask`:

1.  Recompile a extensão e recarregue o VS Code.
2.  Vá em `File > Preferences > File Icon Theme` (ou `Code > Preferences > File Icon Theme` no macOS).
3.  Selecione **"Snask File Icons"** na lista.

## Problemas Conhecidos

Atualmente, o formatador de código e a análise de diagnósticos são baseados em regras simples e podem não cobrir todos os casos complexos da linguagem ou formatos de erro específicos do compilador. Se encontrar problemas, por favor, verifique o formato das mensagens de erro do seu compilador ou ajuste suas configurações.

---

**Aproveite o desenvolvimento com Snask!**