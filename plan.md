# Plano: Adicionar Testes Unitários e Baseados em Propriedades para LinkedList

**Contexto** — Temos uma implementação inicial de `LinkedList` (`push_front`, `pop_front`, `push_back`, `pop_back`). Não há cobertura de testes, dificultando a validação das estruturas em evolução.

**Objetivo** — Garantir a corretude da estrutura de dados via testes automatizados, focando no contrato das funções e usando testes de propriedade (Property-Based Testing).

**Abordagem** — Usar o crate `proptest` (o equivalente em Rust mais popular, robusto e idiomático ao `hypothesis` do Python). Combinar testes unitários convencionais (para edge cases específicos e comportamentos determinísticos) com propriedades geradas (para garantir que invariantes se mantenham independentemente da sequência de operações).

### Passos

1. **Adicionar dependências de teste**
   - **Arquivos:** `dsa/Cargo.toml`
   - **Mudanças:** Adicionar `proptest` na seção `[dev-dependencies]`. Isso possibilita geração aleatória e automática de dados (fuzzing/shrinking) para validação de propriedades.

2. **Implementar testes unitários convencionais**
   - **Arquivos:** `dsa/src/linked_list.rs`
   - **Mudanças:** Criar módulo interno `mod tests` com `#[cfg(test)]`. Adicionar funções testando: 
     - Lidar corretamente com retornos `None` ao chamar `pop` numa lista vazia.
     - Sequências previsíveis: `push_front` + `pop_front`, `push_back` + `pop_back`.

3. **Implementar testes baseados em propriedades**
   - **Arquivos:** `dsa/src/linked_list.rs`
   - **Mudanças:** Adicionar macro `proptest!` para gerar sequências de `push_front` seguidos de `pop_front` (mesmo para `back`). O teste garante que inserir `[A, B, C]` no front resulta em extrair `[C, B, A]`, validando consistência e estado da estrutura para qualquer input genérico `T` (ex: inteiros).

### Arquivos afetados
- `dsa/Cargo.toml` — Adição de ferramentas de teste.
- `dsa/src/linked_list.rs` — Implementação dos casos de teste validando a lógica das operações.

### Impacto de performance
Nenhum em produção. Dependências `[dev-dependencies]` e código sob `#[cfg(test)]` não são compilados no build final.

### Riscos e questões abertas
- **Riscos:** Operações encadeadas de `push_back` são O(n) e podem tornar os testes lentos se `proptest` gerar listas massivas.
- **Mitigação:** Controlar os limites de tamanho de vetores gerados pelo `proptest` (ex: coleções com max 100 elementos).

### Estratégia de rollback
Remover os testes deletando o módulo `mod tests` e reverter as mudanças no `Cargo.toml`.

### Estratégia de testes
Para verificar se tudo funcionou:
Executar `$ cargo test` dentro da pasta `dsa`. O `proptest` automaticamente reportará os dados minimizados (shrinking) caso encontre um teste falho.

### Fora de escopo
- Implementar Traits faltantes de bibliotecas padrão (como `Iterator` ou `Drop`), que seriam boa prática, mas fogem do escopo inicial de validar o que já existe.
