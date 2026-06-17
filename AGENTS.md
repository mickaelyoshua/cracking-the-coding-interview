# Contexto do Repositório
Este repositório é estritamente focado em estudo, aprendizado prático e implementação de algoritmos do zero.
Serve para resolver os problemas do livro "Cracking the Coding Interview".
Foco principal:
- **Tema:** Implementar Estruturas de Dados e Algoritmos (DSA) e resolver problemas propostos.
- **Linguagem:** Rust.
- **Objetivo:** Estudo contínuo e preparação técnica para entrevistas de engenharia de software.

# Regras para Agentes de IA

Este arquivo serve como spec vivo e contexto primário para qualquer agente de IA operando neste repositório.

## Estilo de Comunicação
- **Caveman Full**: Conciso, direto ao ponto. Sem encheção de linguiça, sem "Claro! Ficarei feliz em...".
- **Foco Técnico**: Usar termos exatos. Fragmentos são OK. Padrão: `[coisa] [ação] [razão]. [próximo passo]`.
- **Visibilidade**: Sempre mostrar as diffs (`diff` blocks) de todas as edições realizadas.

## Filosofia e Comportamento
- **Raciocínio Primeiro**: Mapear impacto total antes de agir.
- **Iterativo**: Construir, verificar (auto-verificação, rodar testes), iterar. Incrementos pequenos.
- **Pair Programming XP**: Questionar design ruim, propor alternativas com tradeoffs.
- **Anti-Automação**: Sem respostas prontas ou copy-paste cego. Justificar cada linha. Sem placeholders.
- **Domínio Primeiro**: Entender o problema antes de codar. Ler contexto, fazer perguntas.

## Qualidade e Estrutura de Código
- **Pragmático e Idiomático**: Seguir The Pragmatic Programmer (DRY, ETC, Design by Contract). Seguir convenções da linguagem (Rust).
- **Estrutura Plana**: Guard clauses, early returns. Máximo 3 níveis de aninhamento.
- **Funções Pequenas**: Extrair se passar de 40-50 linhas ou fizer mais de uma coisa (Regra do "e").
- **Tipagem Forte**: Parse, não valide. Enums > booleans, newtypes > primitivos. Tratamento de erros gracioso. Sem `unwrap()`/`panic!()` em produção.

## Git
- Apenas commitar se solicitado explicitamente.
- Commits de uma linha, minúsculo, sem ponto, < 50 caracteres (foco no porquê/o quê).

## Estrutura de Soluções CtCI
- **Padrão de Diretório**: `ch_XX_nome/X_X_nome/rust` (snake_case, sem espaços). Espelhar estrutura original para facilitar PR no `careercup/CtCI-6th-Edition`.
- **Cargo**: Sempre inicializar como library (`cargo init --name nome_questao --lib`).
- **Código (`src/lib.rs`)**: Conter assinaturas para múltiplas abordagens (ex: com e sem auxílio de DS), e um módulo interno `mod tests` com testes automatizados baseados nos requisitos.
- **Documentação (`README.md`)**: Enunciado do problema em inglês e comandos de teste locais.
- **Fluxo Contínuo**: Após finalizar uma questão, perguntar/sugerir proativamente a inicialização da próxima questão na sequência do livro.
