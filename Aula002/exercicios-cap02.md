# Aula 002 — Lista de Exercícios: Evolução das Principais Linguagens de Programação

**Disciplina:** Paradigmas de Linguagens de Programação
**Professor:** Munif Gebara Junior
**Aluno:** João Miguel Silva Salvalagio
**Referência:** SEBESTA, Robert W. *Conceitos de Linguagens de Programação* — Capítulo 2

---

## Sobre esta entrega

A lista possui 20 questões. Conforme solicitado, foram resolvidas **10 questões**: **5 entre as 10 primeiras** (1, 5, 6, 9, 10) e **5 entre as 10 últimas** (11, 12, 14, 16, 20).

O critério de escolha foi privilegiar as questões que tratam diretamente de **paradigmas** — nascimento do paradigma funcional (Lisp), do orientado a objetos (SIMULA 67/Smalltalk), do lógico (Prolog) e da linhagem imperativa/estruturada (ALGOL → Pascal → C) — além dos critérios de projeto de linguagens (ortogonalidade) e da aplicação prática desse conhecimento (estudo de caso).

Ao final há uma seção com **respostas resumidas das 10 questões não escolhidas**, para fins de estudo.

As afirmações históricas foram conferidas diretamente contra o texto do Capítulo 2 da 11ª edição (páginas 49–118 do PDF). Onde a interpretação corrente do mercado difere do que o livro afirma, isso está sinalizado no corpo da resposta.

| Bloco | Questões resolvidas | Tema central |
|---|---|---|
| 1–10 | 1, 5, 6, 9, 10 | Genealogia, funcional vs. imperativo, ALGOL 60, origens de paradigmas, ortogonalidade |
| 11–20 | 11, 12, 14, 16, 20 | Linhagem imperativa vs. lógica, Prolog, OO, scripting, decisão tecnológica |

---

# Parte I — Questões 1 a 10

## Questão 1

> *A genealogia das linguagens não é uma escada de progresso. Explique essa afirmação e apresente dois fatores históricos que fazem uma linguagem influenciar outra sem necessariamente substituí-la.*

**Resposta.**

A metáfora da "escada de progresso" sugere que cada linguagem nova seria estritamente melhor que a anterior e a tornaria obsoleta. A história do Capítulo 2 mostra o contrário: o desenvolvimento das linguagens é uma **árvore genealógica com ramos paralelos, cruzamentos e becos sem saída**, não uma linha ascendente.

Três evidências disso:

1. **Coexistência prolongada.** Fortran (1957), Lisp (1959) e COBOL (1960) nasceram quase juntos, para domínios diferentes, e nenhum eliminou o outro — os três seguem em uso décadas depois. "Melhor" só faz sentido em relação a um domínio.
2. **Influência sem adoção.** ALGOL 60 nunca dominou comercialmente, mas definiu a forma de quase toda linguagem imperativa posterior. A influência é independente do sucesso de mercado.
3. **Ideias antigas ressurgindo.** Recursos considerados "acadêmicos" do Lisp (funções de primeira classe, coleta de lixo, `map`/`filter`, expressões lambda) voltaram como novidade em Java 8, C# e JavaScript moderno. Se fosse uma escada, uma ideia superada não subiria de novo.

**Dois fatores históricos de influência sem substituição:**

| Fator | Como age | Exemplo |
|---|---|---|
| **Nicho de domínio consolidado** | A linguagem antiga resolve tão bem um domínio específico que o custo de migrar supera o ganho. A nova herda ideias, mas ocupa outro espaço. | COBOL segue em bancos e folhas de pagamento; nenhuma linguagem "sucessora" reescreveu esse legado. |
| **Custo de base instalada e treinamento** | Compiladores maduros, bibliotecas validadas, código legado e mão de obra especializada criam inércia econômica. A nova linguagem copia conceitos, não a base. | Fortran mantém bibliotecas numéricas (BLAS, LAPACK) que linguagens novas preferem *chamar* a reescrever. |

**Conclusão:** a genealogia é feita de **herança de ideias**, não de sucessão de tronos. Uma linguagem morre por perda de domínio e comunidade, não por existir uma "melhor".

---

## Questão 5

> *Lisp surgiu em um contexto diferente de Fortran. Compare os domínios, a representação de dados e o estilo de computação favorecido pelas duas linguagens.*

**Resposta.**

Esta é, na prática, a **primeira bifurcação de paradigmas da história**: de um lado o imperativo orientado a máquina, do outro o funcional orientado a símbolos.

### Comparação em três eixos

| Eixo | **Fortran** (1957, John Backus / IBM) | **Lisp** (1959, John McCarthy / MIT) |
|---|---|---|
| **Domínio** | Computação **numérica** científica e de engenharia: álgebra linear, simulações, física. Cliente: usuários de mainframes IBM 704. | **Processamento simbólico** e Inteligência Artificial: manipulação de fórmulas, dedução, listas de símbolos. |
| **Motivação** | Substituir a montagem manual com eficiência competitiva; problema **econômico** (custo de programação). | Expressar computação sobre símbolos com base **matemática** (funções recursivas / cálculo lambda); problema **teórico**. |
| **Representação de dados** | Estruturas **estáticas e homogêneas**: escalares, vetores e matrizes de tamanho fixo, mapeados diretamente sobre a memória da máquina. | Uma única estrutura **dinâmica e heterogênea**: a **lista encadeada** (célula *cons*), construída em tempo de execução, com **coleta de lixo**. |
| **Estilo de computação** | **Imperativo**: sequência de comandos, atribuição destrutiva a variáveis, laços contados (`DO`), desvios. O programa **modifica estado**. | **Funcional**: composição e aplicação de funções, **recursão** no lugar de laços, avaliação de expressões que **produzem valores**. |
| **Código × dados** | Fronteira rígida: código é código, dado é dado. | **Homoiconicidade**: o programa é escrito na mesma estrutura de lista dos dados (`(+ 1 2)` é uma lista). Isso permite que programas gerem e avaliem programas (`eval`, macros). |

### Exemplo do contraste

Somar os elementos de uma sequência:

```fortran
      SOMA = 0
      DO 10 I = 1, N
         SOMA = SOMA + V(I)
   10 CONTINUE
```

*Estado (`SOMA`) é criado e repetidamente sobrescrito. O foco é o **como**, passo a passo.*

```lisp
(defun soma (lista)
  (if (null lista)
      0
      (+ (car lista) (soma (cdr lista)))))
```

*Nenhuma variável é modificada. A definição é a própria relação matemática: a soma de uma lista é o primeiro elemento mais a soma do resto. O foco é o **o quê**.*

### Por que a diferença de domínio explica a diferença de projeto

O domínio numérico tem **tamanho conhecido antecipadamente** (uma matriz 100×100 é 100×100) e exige **desempenho máximo** — daí estruturas estáticas coladas ao hardware. O domínio simbólico tem **tamanho imprevisível** (uma árvore de dedução cresce conforme a busca) — daí alocação dinâmica, listas e coleta de lixo, aceitando custo de execução em troca de expressividade.

**Legado:** de Fortran vem a tradição de desempenho e compilação otimizada; de Lisp vêm coleta de lixo, tipagem dinâmica, funções de primeira classe, `if` como expressão e o REPL — recursos hoje presentes em Python, JavaScript, Ruby, C#, Java moderno e Rust.

---

## Questão 6

> *Avalie três contribuições de ALGOL 60 que ultrapassaram sua adoção comercial. Por que uma linguagem pode ser muito influente sem dominar o mercado?*

**Resposta.**

ALGOL 60 foi um fracasso comercial (nunca teve apoio da IBM, não tinha E/S padronizada, competia com Fortran já estabelecido), mas é provavelmente a linguagem mais influente já criada.

### Três contribuições duradouras

**1. A BNF (Backus-Naur Form) — descrição formal de sintaxe.**
Pela primeira vez a sintaxe de uma linguagem foi definida por uma **gramática formal**, não por prosa em inglês. Consequências: a ambiguidade da especificação desapareceu; tornou-se possível **gerar analisadores sintáticos automaticamente** (yacc, ANTLR, bison); e nasceu a área de compiladores como disciplina científica. Toda especificação de linguagem moderna (Java, C, Python) usa BNF ou variante.

**2. A estrutura de blocos e o escopo léxico.**
ALGOL 60 introduziu o bloco `begin ... end` com **declarações locais** e escopo aninhado, criando as noções de variável local, tempo de vida automático (pilha) e visibilidade estática. Isso é o alicerce da **programação estruturada** e da modularidade. Toda linguagem com `{ }` e variáveis locais — C, Pascal, Java, C#, JavaScript — herda esse modelo.

**3. Recursão e passagem de parâmetros bem definidas.**
ALGOL 60 permitiu **procedimentos recursivos** (Fortran original não permitia) e definiu formalmente mecanismos de passagem de parâmetros (valor e nome). A recursão exigiu registro de ativação em pilha — o modelo de execução usado até hoje em praticamente toda linguagem.

*Contribuições adicionais:* comandos compostos, `if-then-else` estruturado, arrays com limites dinâmicos, tipos declarados explicitamente.

### Por que influência ≠ domínio de mercado

Porque **adoção** e **influência** dependem de fatores diferentes:

- **Adoção** depende de fatores extrínsecos: apoio de um fabricante dominante, compiladores disponíveis, base instalada, bibliotecas, treinamento, E/S prática, marketing. ALGOL falhou em todos: sem padrão de E/S, sem IBM, sem base instalada.
- **Influência** depende de fatores intrínsecos: clareza conceitual, boas abstrações, especificação precisa. ALGOL era excelente nisso e por isso virou a **linguagem de referência para publicar algoritmos** em periódicos científicos — o que a colocou na formação de toda uma geração de projetistas de linguagens.

**Analogia:** ALGOL 60 foi para as linguagens o que um artigo científico seminal é para uma área — pouca gente usa o protótipo original, todo mundo usa as ideias. Casos semelhantes: SIMULA 67 (pouco usada, criou a OO), Smalltalk (nicho, definiu OO pura), ML (acadêmica, gerou inferência de tipos hoje em Rust, Swift, TypeScript).

---

## Questão 9

> *APL, SNOBOL e SIMULA 67 seguiram direções distintas. Associe cada linguagem ao seu foco e identifique uma contribuição duradoura de cada uma.*

**Resposta.**

As três são **linguagens de propósito especial** dos anos 1960 que provam que o domínio molda o projeto — e duas delas fundaram paradigmas inteiros.

### Quadro comparativo

| Linguagem | Ano / Autor | Foco (domínio) | Ideia central | Contribuição duradoura |
|---|---|---|---|---|
| **APL** | 1962 · Kenneth Iverson (IBM) | Manipulação de **matrizes e vetores**; notação matemática executável | Operadores de alto nível aplicados a **arrays inteiros**, sem laços explícitos; conjunto próprio de símbolos especiais | **Programação orientada a arrays** (*array programming*): operar sobre coleções inteiras de uma vez. Reaparece em NumPy, MATLAB, R, pandas, APIs vetorizadas e em GPUs. |
| **SNOBOL** | 1964 · Bell Labs (Farber, Griswold, Polonsky) | Processamento de **texto e cadeias de caracteres**; edição, análise textual | **Casamento de padrões** (*pattern matching*) como construção primitiva da linguagem, com backtracking | O reconhecimento de padrões como recurso de primeira classe — linhagem que leva às **expressões regulares** (grep, Perl, Python `re`, JavaScript) e ao pattern matching de Haskell, Rust, Scala e Python 3.10. |
| **SIMULA 67** | 1967 · Ole-Johan Dahl e Kristen Nygaard (Noruega) | **Simulação de eventos discretos** (filas, tráfego, sistemas físicos) | Modelar cada entidade do mundo real como uma unidade com dados + comportamento próprios | **A classe e o objeto** — nascimento da **Orientação a Objetos**: classes, instâncias, herança e corrotinas. Origem direta de Smalltalk, C++, Java, C#, Python. |

### Por que cada foco produziu esse projeto

- **APL** partiu de uma *notação para descrever máquinas*, não de uma linguagem de programação. Como matemática de matrizes não usa laços, APL também não precisa deles: `+/V` soma um vetor inteiro. Lição: **a notação escolhida determina o que é fácil de pensar**.
- **SNOBOL** enfrentava texto de tamanho e forma imprevisíveis, onde laços de caracteres são penosos. A solução foi elevar "casar um padrão" a operação primitiva — a mesma ideia que hoje faz uma regex substituir 30 linhas de código.
- **SIMULA 67** precisava simular muitas entidades concorrentes com estado próprio. Estruturas ALGOL não bastavam: era preciso agrupar dados e procedimentos e criar cópias independentes. Dahl e Nygaard estenderam ALGOL 60 com `class` — **a OO nasceu como técnica de modelagem de simulação, não como técnica de engenharia de software**; esse uso veio depois, com Smalltalk e C++.

### Uma ressalva importante do próprio Sebesta

O livro afirma explicitamente que **"nem APL nem SNOBOL tiveram muita influência sobre as principais linguagens posteriores"** — a influência direta e documentada é estreita: **J** é baseada em APL, e **ICON** e (parcialmente) **AWK** são baseadas em SNOBOL. Do mesmo modo, SIMULA 67 "nunca atingiu amplo uso e teve pouco impacto na computação de sua época".

Isso não anula o argumento acima, mas exige distinguir dois tipos de legado:

- **SIMULA 67** tem influência **conceitual direta e comprovada**: a construção `class` foi retomada por Smalltalk e C++, e o conceito de **abstração de dados** só foi reconhecido como tal por Hoare em 1972, olhando para ela.
- **APL e SNOBOL** têm influência **de ideia, não de linhagem**: nenhuma linguagem principal descende delas, mas o *estilo vetorial* e o *casamento de padrões como operação primitiva* reapareceram, redescobertos, em bibliotecas e linguagens posteriores.

**Síntese:** APL originou o estilo vetorial (hoje presente na computação científica), SNOBOL o pattern matching declarativo, e SIMULA 67 — a mais consequente das três — o paradigma orientado a objetos.

---

## Questão 10

> *Defina ortogonalidade no projeto de linguagens e use ALGOL 68 para discutir a diferença entre regularidade e simplicidade. Uma linguagem muito ortogonal é automaticamente fácil de usar?*

**Resposta.**

### Definição

**Ortogonalidade** é a propriedade de uma linguagem em que um conjunto pequeno de construções primitivas pode ser **combinado livremente**, sem exceções ou restrições arbitrárias, e o significado de cada combinação é previsível a partir do significado das partes.

Ortogonalidade tem, portanto, dois requisitos:

1. **Poucos conceitos primitivos.**
2. **Combinação irrestrita** desses conceitos (toda combinação é legal e tem o sentido esperado).

**Exemplo de falta de ortogonalidade (C):** uma função pode retornar `struct`, mas não um array. Um array decai para ponteiro em alguns contextos e não em outros. São exceções que o programador precisa memorizar.

**Exemplo de ortogonalidade:** se em uma linguagem "qualquer tipo pode ser elemento de array, campo de registro, parâmetro e retorno de função", sem exceções, então o conceito de tipo é ortogonal aos conceitos de array, registro e função.

### ALGOL 68: regularidade levada ao extremo

ALGOL 68 foi projetada com ortogonalidade como **objetivo explícito**. Ela partiu de poucos conceitos (modos/tipos, valores, coerções) e permitiu combiná-los de forma quase irrestrita: tipos definidos pelo usuário compostos livremente, atribuição como expressão que retorna valor, `if` e `case` como expressões, coerções automáticas aplicadas uniformemente.

O resultado foi uma linguagem **muito regular** — e mesmo assim de **uso difícil**, com pouca adoção. Duas razões:

- **A descrição ficou impenetrável.** O relatório usava uma gramática de dois níveis (*W-grammar*, de van Wijngaarden) e vocabulário próprio ("mode", "voiding", "deproceduring"), inacessível ao programador comum.
- **A combinação livre gerou complexidade emergente.** Regras uniformes aplicadas a tudo produzem uma explosão de combinações válidas mas obscuras; a rede de coerções automáticas tornava difícil prever qual conversão aconteceria em cada expressão.

### Regularidade ≠ simplicidade

| | **Regularidade** | **Simplicidade** |
|---|---|---|
| O que é | Ausência de exceções: as regras valem uniformemente | Baixo esforço cognitivo total para aprender e usar |
| Medida em | Consistência das regras | Quantidade de coisas a manter na cabeça |
| Quem se beneficia | Projetista da linguagem e implementador do compilador | Usuário da linguagem |

Regularidade é sobre **as regras**; simplicidade é sobre **o efeito das regras na cabeça de quem programa**. Poucas regras que se combinam livremente geram um **espaço de combinações enorme** — a especificação encolhe, o espaço de uso explode.

### Então, uma linguagem muito ortogonal é automaticamente fácil de usar?

**Não.** ALGOL 68 é a prova histórica. A ortogonalidade **reduz a complexidade da especificação**, mas não reduz — e pode até aumentar — a complexidade do **uso**. Ela é uma virtude *necessária, porém insuficiente*: precisa vir acompanhada de sintaxe legível, boa documentação, número controlado de primitivas e restrições **deliberadas** que protejam o programador.

Comparação prática:

- **Pascal** (Wirth, 1971) foi a resposta explícita a ALGOL 68: menos ortogonal, mais restritiva, **muito mais fácil** — e vastamente mais adotada, sobretudo no ensino.
- **Python** aceita algumas irregularidades em nome da legibilidade; **Go** proíbe combinações válidas em nome da uniformidade de estilo. Ambas são consideradas fáceis.
- **C++** tem alta capacidade de combinação e é notoriamente difícil.

**Conclusão:** ortogonalidade é um critério de projeto valioso, mas **usabilidade é ortogonalidade + legibilidade + escala controlada**. Regularidade sem contenção produz linguagens elegantes no papel e hostis na prática.

---

# Parte II — Questões 11 a 20

## Questão 11

> *Construa uma cadeia de influência que passe por ALGOL, Pascal e C. Depois contraste essa linhagem imperativa com a proposta declarativa de Prolog.*

**Resposta.**

### A cadeia imperativa

```
ALGOL 60 (1960)
   │ blocos, escopo léxico, recursão, if-then-else estruturado, BNF
   ├──────────────────────────────┐
   ▼                              ▼
ALGOL 68 (1968)              ALGOL-W (1966, Wirth/Hoare)
   │ ortogonalidade extrema       │ proposta simplificada e rejeitada
   │ (rejeitada por complexa)     ▼
   │                         Pascal (1971, Niklaus Wirth)
   │                              │ tipagem forte, registros, enumerações,
   │                              │ subfaixas, ponteiros tipados, ensino
   │                              ▼
   └──► CPL → BCPL (1967) → B (1969) → C (1972, Dennis Ritchie)
                                  │ acesso a baixo nível, aritmética de ponteiros,
                                  │ tipagem fraca, portabilidade (Unix)
                                  ▼
                    C++ (1983) · Java (1995) · C# (2000) · Go · Rust
```

> **Precisão histórica (Sebesta):** o livro lista os ancestrais de C como **CPL, BCPL, B e ALGOL 68** — a influência de ALGOL sobre C chega **por ALGOL 68** (em parte via BCPL, em parte diretamente), não diretamente de ALGOL 60. Pascal, por sua vez, vem de ALGOL 60 por ALGOL-W. As duas linhagens partem do mesmo tronco ALGOL, mas por ramos diferentes.

**Tipo de cada influência (não é apenas cronologia):**

| Ligação | Tipo de influência |
|---|---|
| ALGOL 60 → Pascal | **Herança direta com simplificação.** Wirth manteve blocos, escopo e estrutura, mas reduziu o conjunto de recursos e reforçou o sistema de tipos, com foco em ensino e programação estruturada. |
| ALGOL 60 → ALGOL 68 → (rejeição) | **Influência negativa.** A complexidade de ALGOL 68 foi o motivo declarado para Wirth criar Pascal — reagir a um projeto é uma forma de influência. |
| ALGOL 68 → CPL → BCPL → B → C | **Herança com inversão de prioridade.** A estrutura de blocos sobreviveu, mas o objetivo mudou: em vez de rigor e verificação, **eficiência e acesso à máquina** (C foi criada para escrever o Unix). |
| Pascal + C → Java/C# | **Síntese.** Sintaxe e pragmatismo de C somados à disciplina de tipos de Pascal e aos objetos de SIMULA/Smalltalk. |

**O que os três compartilham (o núcleo imperativo):** variáveis como células de memória nomeadas, **atribuição destrutiva** como operação central, execução sequencial com fluxo controlado explicitamente pelo programador, e subprogramas em pilha. O programa é uma **receita**: descreve *como* chegar ao resultado.

### O contraste com Prolog

Prolog (1972, Colmerauer e Roussel, Marselha; base teórica de Kowalski) rompe com essa linhagem inteira. Ele não descende de ALGOL: descende da **lógica de predicados de primeira ordem** e do princípio da resolução.

| Aspecto | Linhagem ALGOL → Pascal → C | **Prolog** |
|---|---|---|
| **Raiz teórica** | Máquina de von Neumann (memória + instruções) | Lógica de primeira ordem / resolução SLD |
| **O que o programador escreve** | A **sequência de passos** (*como*) | As **relações verdadeiras** (*o quê*) |
| **Unidade de programa** | Comando / procedimento | Fato, regra (cláusula de Horn) |
| **Variável** | Nome de um endereço de memória, **remutável** | Incógnita lógica, **unificada uma vez** (sem atribuição destrutiva) |
| **Controle de execução** | Explícito: laços, `if`, chamadas | **Implícito**: a máquina de inferência faz busca e **backtracking** |
| **Execução** | Determinística e única | Pode produzir **várias soluções**; pode falhar e retroceder |
| **Reversibilidade** | Função tem entradas e saídas fixas | Uma mesma relação serve para **consultar em várias direções** |

**O mesmo problema nos dois estilos — "quem é avô de quem":**

```c
/* C — imperativo: eu descrevo o algoritmo de busca */
Pessoa* avo(Pessoa* p) {
    if (p == NULL || p->pai == NULL) return NULL;
    return p->pai->pai;
}
```

```prolog
% Prolog — declarativo: eu descrevo a relação; a busca é do motor
avo(X, Y) :- progenitor(X, Z), progenitor(Z, Y).
```

Na versão Prolog não há laço, verificação de nulo nem ordem de busca escrita à mão — e a mesma cláusula responde `avo(joao, Q)` ("de quem João é avô?") **ou** `avo(Q, pedro)` ("quem é avô de Pedro?"). Em C, cada pergunta exigiria uma função diferente.

**Síntese:** a linhagem ALGOL→Pascal→C evoluiu **abstraindo a máquina progressivamente, sem sair do modelo da máquina**. Prolog trocou o modelo: o programador vira um **especificador de conhecimento** e delega o controle. O preço é perder previsibilidade de desempenho e precisar de recursos extralógicos (como o corte `!`) quando a busca automática não é eficiente o bastante.

---

## Questão 12

> *Modele em linguagem natural uma pequena base Prolog com dois fatos, uma regra e uma consulta. Explique por que isso representa programação lógica, não apenas armazenamento de dados.*

**Resposta.**

### O modelo em linguagem natural

Domínio escolhido: pré-requisitos de disciplinas de um curso.

- **Fato 1:** "Algoritmos é pré-requisito de Estruturas de Dados."
- **Fato 2:** "Estruturas de Dados é pré-requisito de Paradigmas de Linguagens de Programação."
- **Regra:** "Uma disciplina A é **dependência** de uma disciplina C se A é pré-requisito direto de C, **ou** se A é pré-requisito de alguma disciplina B que, por sua vez, tem C como dependente." (isto é: dependência é o fecho transitivo de pré-requisito)
- **Consulta:** "Quais disciplinas eu preciso ter cursado antes de Paradigmas?"

### A base em Prolog

```prolog
% --- Fatos: conhecimento explícito ---
prerequisito(algoritmos, estruturas_dados).
prerequisito(estruturas_dados, paradigmas).

% --- Regra: conhecimento derivável ---
dependencia(A, C) :- prerequisito(A, C).
dependencia(A, C) :- prerequisito(A, B), dependencia(B, C).

% --- Consulta ---
?- dependencia(X, paradigmas).
X = estruturas_dados ;
X = algoritmos ;
false.
```

### Como o motor chegou nessa resposta

1. Recebe o objetivo `dependencia(X, paradigmas)`.
2. Tenta a **primeira cláusula**: `prerequisito(X, paradigmas)` **unifica** com o Fato 2 → `X = estruturas_dados`. Primeira solução.
3. Ao ser pedida outra solução (`;`), faz **backtracking** e tenta a **segunda cláusula**: precisa de `prerequisito(X, B)` e `dependencia(B, paradigmas)`. Unifica com o Fato 1 → `X = algoritmos`, `B = estruturas_dados`; então precisa provar `dependencia(estruturas_dados, paradigmas)`, o que consegue pela primeira cláusula. Segunda solução.
4. Esgotadas as alternativas, responde `false` (não há mais soluções).

### Por que isso é programação lógica e não um banco de dados

Esta é a parte central da questão. Um banco de dados **armazena e recupera** o que foi gravado; a programação lógica **deriva conhecimento novo** que nunca foi armazenado.

| Critério | Armazenamento de dados | **Programação lógica** |
|---|---|---|
| **Conhecimento derivado** | Só devolve o que foi gravado | `algoritmos` é dependência de `paradigmas` — **esse fato não está na base**; foi **deduzido** pela regra |
| **Recursão** | Consultas hierárquicas exigem extensões especiais | A regra é **recursiva por natureza**: `dependencia` chama a si mesma, resolvendo cadeias de profundidade arbitrária |
| **Motor de execução** | Busca em índices | **Inferência**: unificação, resolução SLD, backtracking automático |
| **Direção da consulta** | Consulta e esquema são fixos | A **mesma regra** responde `dependencia(X, paradigmas)`, `dependencia(algoritmos, Y)` e `dependencia(algoritmos, paradigmas)` (sim/não) |
| **Papel do programador** | Descreve a estrutura dos dados e o algoritmo da consulta | Descreve **o que é verdade**; o *como* buscar é do sistema |
| **Completude computacional** | Não é linguagem de programação completa | Prolog é **Turing-completo** — regras recursivas com unificação bastam |

**O ponto decisivo:** o par fatos + regras é, ao mesmo tempo, **dados e programa**. As cláusulas são fórmulas lógicas (cláusulas de Horn); executar o programa é **provar um teorema** — a resposta `X = algoritmos` é a *testemunha* da prova. Nenhum banco de dados faz isso, porque nele não existe a noção de "provar"; existe apenas "encontrar o que está lá".

E note o que **não** foi escrito: nenhum laço, nenhuma pilha de visitados, nenhuma condição de parada explícita, nenhuma ordem de percurso. Em uma linguagem imperativa, esse fecho transitivo exigiria uma busca em grafo escrita à mão. Esse é o ganho — e a assinatura — do paradigma lógico.

---

## Questão 14

> *Compare o papel dos objetos em Smalltalk, C++ e Java. Inclua na resposta o compromisso de C++ com C e a estratégia de portabilidade de Java.*

**Resposta.**

As três representam **três graus diferentes de compromisso com a orientação a objetos**: OO pura, OO híbrida e OO obrigatória mas pragmática.

### Quadro comparativo

| Aspecto | **Smalltalk** (1980, Alan Kay / Xerox PARC) | **C++** (1983, Bjarne Stroustrup / Bell Labs) | **Java** (1995, James Gosling / Sun) |
|---|---|---|---|
| **Grau de OO** | **Pura**: *tudo* é objeto — inteiros, classes, blocos de código | **Híbrida**: OO é opcional; código C procedural é C++ válido | **Quase pura**: toda função vive em uma classe, mas há **tipos primitivos** (`int`, `double`) por desempenho |
| **Modelo de computação** | **Troca de mensagens** entre objetos; o objeto decide como responder | **Chamada de método**, resolvida em compilação salvo se `virtual` | Chamada de método com **despacho dinâmico por padrão** |
| **Vinculação (binding)** | Totalmente **dinâmica**, em tempo de execução | **Estática por padrão**, dinâmica só com `virtual` (paga-se pelo que se usa) | **Dinâmica por padrão**, estática com `final`/`static` |
| **Tipagem** | Dinâmica | Estática e forte (com brechas herdadas de C) | Estática e forte |
| **Herança** | Simples | **Múltipla** (com problema do diamante e herança `virtual`) | **Simples** de classes + múltipla de **interfaces** — evita o diamante |
| **Memória** | Coleta de lixo | **Manual** (`new`/`delete`, RAII, destrutores determinísticos) | **Coleta de lixo** automática |
| **Execução** | Máquina virtual + imagem viva | **Compilada para código nativo** | **Bytecode + JVM** |
| **Prioridade de projeto** | Expressividade e ambiente interativo | **Desempenho e compatibilidade** | **Portabilidade, segurança e simplicidade** |

### O compromisso de C++ com C

Stroustrup adotou uma decisão estratégica: **C++ deveria ser (quase) um superconjunto de C**, para que a base instalada de código, programadores e ferramentas C migrasse sem reescrita. Daí decorrem dois princípios:

- *"Não pague pelo que não usa"* — abstrações têm **custo zero** quando não utilizadas: por isso os métodos **não** são virtuais por padrão (evitando o custo da vtable), e não há coletor de lixo obrigatório.
- *Compatibilidade com C* — ponteiros, aritmética de ponteiros, `malloc`, cabeçalhos e chamadas de sistema continuam disponíveis.

**Os ganhos:** adoção massiva e imediata; desempenho equivalente ao de C; viabilidade em sistemas operacionais, jogos, embarcados e software de alto desempenho.

**Os custos:** a linguagem herdou toda a insegurança de C (ponteiros soltos, estouro de buffer, vazamento de memória, comportamento indefinido); ficou enorme e complexa, com múltiplos estilos coexistindo; e a OO passou a ser uma **possibilidade, não uma disciplina** — é perfeitamente possível escrever C++ sem uma única classe.

### A estratégia de portabilidade de Java

O lema era *"write once, run anywhere"*. Os mecanismos:

1. **Bytecode + JVM.** O compilador não gera código nativo, mas **bytecode** para uma máquina virtual abstrata. Portar Java para uma plataforma significa portar a JVM, não recompilar cada programa. O binário é o mesmo em todo lugar.
2. **Semântica fixa e independente de máquina.** Ao contrário de C/C++, onde o tamanho de `int` é definido pela implementação, Java **fixa** os tamanhos (`int` = 32 bits sempre) e o comportamento aritmético. Isso elimina uma classe inteira de bugs de portabilidade.
3. **Eliminação dos recursos dependentes de máquina.** Fora ponteiros explícitos, aritmética de ponteiros, `goto`, herança múltipla de classes, sobrecarga de operadores e gerenciamento manual de memória. Menos poder, mais previsibilidade e segurança.
4. **Biblioteca padrão ampla e uniforme.** Rede, threads, coleções e GUI vêm na plataforma com o mesmo comportamento em todo sistema — em C++ isso era território de bibliotecas específicas de cada SO.

**O custo:** desempenho inferior ao nativo (mitigado depois pela compilação **JIT**), consumo de memória maior, pausas de coleta de lixo (inadequadas para tempo real duro) e menos controle de baixo nível.

### Síntese: qual é o "papel do objeto" em cada uma

- **Smalltalk:** o objeto é a **única** unidade de computação; programar *é* fazer objetos trocarem mensagens. Consequência: enorme uniformidade conceitual, mas desempenho sacrificado.
- **C++:** o objeto é uma **ferramenta de abstração opcional e de custo controlado**, sobreposta ao modelo de memória de C. Consequência: potência máxima com responsabilidade máxima.
- **Java:** o objeto é a **unidade obrigatória de organização do programa**, com concessões pragmáticas (primitivos, interfaces) em nome de desempenho e simplicidade. Consequência: disciplina imposta, portabilidade real, teto de desempenho mais baixo.

A trajetória Smalltalk → C++ → Java mostra o paradigma OO sendo **negociado** contra as restrições reais da indústria: pureza conceitual cedendo espaço a desempenho, e depois a segurança e portabilidade.

---

## Questão 16

> *Compare Perl, JavaScript, PHP, Python, Ruby e Lua usando três eixos: domínio inicial, estruturas de dados e estratégia de implementação. Evite concluir que todas são iguais por serem chamadas de scripting.*

**Resposta.**

"Linguagem de script" descreve um **modo de uso** (interpretada, tipagem dinâmica, ciclo rápido de edição-execução), não um projeto comum. As seis divergem profundamente em origem e estrutura.

### Quadro comparativo

| Linguagem | Ano / Autor | **Domínio inicial** | **Estruturas de dados características** | **Estratégia de implementação** |
|---|---|---|---|---|
| **Perl** | 1987 · Larry Wall | Administração de sistemas Unix: relatórios, processamento de texto e logs — "canivete suíço" entre shell, sed e awk | Escalares, **arrays** e **hashes** distinguidos por sigilo (`$`, `@`, `%`); **regex embutida na sintaxe** da linguagem | Interpretador que compila para árvore/opcodes em memória a cada execução; CPAN como ecossistema |
| **JavaScript** | 1995 · Brendan Eich (Netscape) | **Interatividade no navegador**: validar formulários e manipular a página (criada em 10 dias) | **Objeto = mapa dinâmico** de chave→valor; herança por **protótipos** (não por classes); arrays são objetos; funções são objetos de primeira classe com closures | Interpretador embutido no navegador; hoje **JIT** avançado (V8, SpiderMonkey); execução também no servidor via Node.js |
| **PHP** | 1995 · Rasmus Lerdorf | **Páginas Web dinâmicas no servidor**: gerar HTML e conversar com banco de dados | **Array associativo unificado**: uma única estrutura serve como lista e dicionário (ordenada); forte acoplamento a `$_GET`, `$_POST`, `$_SESSION` | **Embutida no HTML** (`<?php ... ?>`); interpretador dentro do servidor Web, ciclo de vida por requisição (estado descartado a cada acesso) |
| **Python** | 1991 · Guido van Rossum | Linguagem de **propósito geral e ensino**, sucessora do ABC; scripts de sistema | Conjunto rico e **distinto**: `list`, `tuple`, `dict`, `set`; **indentação significativa**; protocolos (iterável, sequência) como contrato de tipos | Compila para **bytecode** (`.pyc`) executado por VM (CPython); implementações alternativas (PyPy com JIT); C-API para extensões — origem de NumPy, pandas, TensorFlow |
| **Ruby** | 1995 · Yukihiro Matsumoto | **Produtividade e prazer do programador**; propósito geral; explodiu com Rails (2005) na Web | **Tudo é objeto** (inclusive inteiros e `nil`); **blocos** (closures) como recurso central de iteração; classes **abertas** (reabríveis em tempo de execução) | Interpretador de árvore no início; a partir da 1.9, **VM de bytecode (YARV)**; forte uso de **metaprogramação** (`method_missing`, `define_method`) — base dos DSLs de Rails |
| **Lua** | 1993 · PUC-Rio (Ierusalimschy, Figueiredo, Celes) | **Linguagem embarcada de extensão**: configurar e estender aplicações hospedeiras (originalmente na Petrobras); depois jogos e dispositivos restritos | **Uma única estrutura: a `table`** — serve como array, dicionário, objeto, classe e namespace; **metatables** implementam herança Traduzida para **código intermediário e interpretada** (como as primeiras implementações de Java; hoje, VM de registradores), com **coleta de lixo** e apenas 21 palavras reservadas; projetada para **embutir em outra aplicação**; sem OO nativa — OO se constrói com tabelas e metatables |

### Onde está a diferença real

**1. Domínio inicial → forma da linguagem.**
Perl otimiza **texto** (regex é sintaxe, não biblioteca). PHP otimiza **gerar HTML** (o código vive dentro do documento). JavaScript otimiza **eventos na página**. Lua otimiza **caber dentro de outro programa**. Python e Ruby foram, desde o início, de **propósito geral** — e é por isso que se expandiram para domínios que as outras não alcançaram.

**2. Modelo de dados → filosofia oposta.**
Aqui está o contraste mais nítido do conjunto:

- **Lua e PHP escolheram a unificação radical:** uma estrutura para tudo (`table` / array associativo). Vantagem: minimalismo e implementação enxuta. Custo: menos garantias — a estrutura não comunica intenção.
- **Python escolheu a distinção deliberada:** `list` ≠ `tuple` ≠ `dict` ≠ `set`, cada uma com semântica, custo e mutabilidade próprios. Vantagem: o tipo expressa a intenção e a complexidade algorítmica. Custo: mais para aprender.
- **JavaScript é intermediária:** objeto-mapa universal, com arrays como caso especial — daí anomalias como `typeof [] === "object"`.

**3. Estratégia de implementação → onde o código roda.**
Isso separa as seis em grupos distintos: **embutida em aplicação C** (Lua), **embutida no navegador** (JavaScript), **embutida no servidor Web com estado por requisição** (PHP), **VM de bytecode de propósito geral com extensões nativas** (Python, Ruby), **interpretador de processo Unix** (Perl).

### Conclusão

Chamar as seis de "scripting" descreve apenas a superfície compartilhada. Sob ela: **JavaScript é prototípica, Ruby é OO pura, Lua é minimalista e embarcável, Python é multiparadigma e explícita, Perl é orientada a texto, PHP é orientada a requisições Web.** São *seis projetos de linguagem diferentes*, com decisões de modelo de dados e de execução que os tornam bons em coisas distintas — o que explica por que nenhum deles eliminou os outros.

---

## Questão 20

> *Estudo de caso: uma equipe precisa escolher tecnologias para cálculo científico, regras declarativas, aplicação Web interativa e firmware restrito. Proponha famílias de linguagens, justifique historicamente cada escolha e explicite dois trade-offs.*

**Resposta.**

Este exercício é a **síntese aplicada** do capítulo: o argumento é que a genealogia das linguagens não é curiosidade histórica — ela **prevê** qual família se encaixa em qual domínio, porque cada família foi *moldada* pelo domínio que a originou.

### Recomendações por subsistema

#### 1. Cálculo científico → família Fortran / orientada a arrays

**Proposta:** Python + NumPy/SciPy como camada de orquestração, com núcleo em Fortran/C, ou Julia para código novo intensivo.

**Justificativa histórica:** Fortran nasceu em 1957 exatamente para esse domínio, sob a pressão de igualar o desempenho da montagem manual. Décadas de otimização de compiladores para laços numéricos e as bibliotecas validadas **BLAS/LAPACK** (escritas em Fortran) formam um ativo que ninguém reescreve — NumPy, MATLAB, R e Julia **chamam** esse código. O estilo de operar sobre arrays inteiros vem de **APL** (1962) e é o que hoje mapeia naturalmente para SIMD e GPU. A escolha é herdeira direta de duas linhagens do capítulo.

#### 2. Regras declarativas → família lógica / baseada em regras

**Proposta:** Prolog (SWI-Prolog), Datalog ou um motor de regras (Drools/CLIPS) integrado ao sistema principal.

**Justificativa histórica:** Prolog (1972) foi criado para expressar conhecimento como fatos e regras, delegando a busca a um motor de inferência baseado em resolução. Quando o requisito é "as regras de negócio mudam com frequência e precisam ser auditáveis por especialistas do domínio", o paradigma lógico entrega o que a linhagem imperativa não entrega: as regras ficam **declaradas e separadas do controle**, e a mesma base responde a consultas em várias direções. Codificar isso em `if/else` em C ou Java transforma conhecimento em fluxo de controle, e cada mudança de regra vira alteração de algoritmo.

#### 3. Aplicação Web interativa → famílias de scripting Web

**Proposta:** JavaScript/TypeScript no cliente; no servidor, TypeScript (Node), Python, Ruby, C# ou Java, conforme a equipe.

**Justificativa histórica:** JavaScript (1995) foi criada especificamente para interatividade no navegador e é a **única** linguagem executada nativamente por ele — um monopólio de plataforma, não uma superioridade técnica. TypeScript resgata, sobre ela, a disciplina de tipos da linhagem ALGOL→Pascal→Java, que o JavaScript original descartou em nome da agilidade. No servidor, todas as opções citadas são maduras; a decisão aqui é de **ecossistema e equipe**, não de capacidade.

#### 4. Firmware restrito → família C / sistemas

**Proposta:** C (padrão MISRA em contexto crítico) ou Rust embarcado (`no_std`); Ada/SPARK se houver certificação de segurança crítica.

**Justificativa histórica:** C (1972) foi criada para escrever o Unix — ou seja, para **acessar o hardware com abstração mínima e custo previsível**. Não tem coletor de lixo (logo, sem pausas imprevisíveis), tem controle explícito de memória e mapeia diretamente sobre o hardware; existe compilador C para praticamente todo microcontrolador já fabricado. Rust oferece as mesmas garantias de custo zero **acrescidas** de segurança de memória verificada em compilação. Se o domínio for aviônica, médico ou ferroviário, **Ada** (1980) é a escolha histórica: foi projetada por requisitos do Departamento de Defesa dos EUA justamente para sistemas críticos, com tipagem forte, pacotes, tratamento de exceções e concorrência (*tasks*) na própria linguagem.

### Quadro-resumo

| Subsistema | Família | Linguagem-âncora | Restrição dominante |
|---|---|---|---|
| Cálculo científico | Fortran / arrays | Python+NumPy, Julia, Fortran | Throughput numérico e bibliotecas validadas |
| Regras declarativas | Lógica | Prolog, Datalog, Drools | Expressividade e manutenção do conhecimento |
| Web interativa | Scripting Web | TypeScript/JavaScript | Plataforma imposta (navegador) e velocidade de entrega |
| Firmware restrito | Sistemas | C, Rust, Ada | Memória, determinismo e ausência de runtime |

### Dois trade-offs explícitos

**Trade-off 1 — Poliglotismo vs. custo de integração e de equipe.**
Usar quatro famílias diferentes maximiza a adequação técnica de cada parte, mas cria custo real: quatro cadeias de build e teste, quatro conjuntos de dependências, contratação e treinamento em quatro ecossistemas, e — o mais caro — **as fronteiras entre eles**. Cada interface entre linguagens (FFI, serialização, RPC) é ponto de falha, de perda de desempenho e de tipos que deixam de ser verificados. A alternativa (unificar tudo em uma linguagem de propósito geral) reduz esse custo, mas aceita ferramentas subótimas em pelo menos dois subsistemas — provavelmente as regras declarativas viram um emaranhado de `if`, e o cálculo científico perde desempenho.
*Recomendação:* limitar a três ecossistemas, usando Python como "cola" entre o núcleo científico e o motor de regras, e isolar o firmware por interface bem definida (protocolo de mensagens), não por FFI.

**Trade-off 2 — Segurança/produtividade vs. controle e desempenho previsível.**
Linguagens com coletor de lixo e tipagem dinâmica (Python, JavaScript) entregam velocidade de desenvolvimento e eliminam classes inteiras de bugs de memória, ao custo de **latência imprevisível** (pausas de GC) e maior consumo de recursos — inaceitável no firmware. Linguagens sem runtime (C) dão controle total e determinismo, ao custo de transferir ao programador a responsabilidade por erros que custam caro em produção (estouro de buffer, ponteiro solto). Rust é a tentativa histórica recente de recusar esse dilema — segurança de memória **sem** coletor de lixo — pagando com uma **curva de aprendizado íngreme** e menor disponibilidade de mão de obra.
*Recomendação:* aceitar GC onde a latência não é crítica (Web e orquestração científica) e exigir ausência de runtime no firmware — escolhendo Rust apenas se a equipe puder absorver o custo de aprendizado, e C+MISRA caso contrário.

### Fechamento

Cada escolha acima é rastreável a uma decisão de projeto tomada entre 1957 e 1995 sob uma restrição concreta. É isso que o Capítulo 2 ensina: **linguagens não são intercambiáveis porque carregam, em sua estrutura, o problema que as originou.**

---

# Parte III — Questões não resolvidas: respostas breves comentadas

Resumos de estudo das 10 questões restantes, com o raciocínio esperado.

### Questão 2 — Plankalkül

**Resposta breve:** Konrad Zuse projetou Plankalkül entre 1943 e 1945, na Alemanha isolada pela guerra, e ela só foi publicada em 1972 — nunca foi implementada nem influenciou ninguém na época.
**Relevância:** demonstra que as ideias fundamentais das linguagens de alto nível eram **alcançáveis conceitualmente** muito antes de a tecnologia e o contexto permitirem sua difusão — a história depende tanto de circunstância quanto de mérito técnico.
**Três recursos antecipados:** (1) tipos de dados estruturados construídos a partir do bit, incluindo arrays e registros; (2) instrução de seleção sem `else`; (3) uma forma de asserção matemática sobre o programa.
**Valor de um deles:** os **registros/arrays** antecipam em mais de uma década a ideia de que o programador deve modelar dados no vocabulário do problema, e não no da memória da máquina — o núcleo de toda abstração de dados posterior.
**Por que a resposta é essa:** o ponto da questão é separar *influência histórica* de *qualidade técnica*; Plankalkül tem a segunda sem a primeira.

### Questão 3 — Short Code, Speedcoding, A-0/A-1/A-2

**Resposta breve:** os três atacam o mesmo problema — programar em código de máquina é lento e propenso a erro — com estratégias diferentes. **Short Code** (Mauchly, 1949) era **interpretado**: representava expressões matemáticas em código numérico e as decodificava em execução (dezenas de vezes mais lento que o código nativo). **Speedcoding** (Backus, 1954) era um **interpretador que simulava um computador virtual** com aritmética de ponto flutuante em uma máquina que não a possuía — vendia capacidade que o hardware não tinha. **A-0/A-1/A-2** (Grace Hopper, 1951-53) eram **montadores de subrotinas**: expandiam pseudocódigos em chamadas a rotinas de biblioteca pré-escritas, com ajuste de argumentos.
**Por que "compiladores modernos" é impreciso:** um compilador moderno faz **análise léxica, sintática e semântica** de uma linguagem com gramática formal e **gera código otimizado**. Nenhum dos três fazia isso: Short Code e Speedcoding interpretavam em tempo de execução (não traduziam previamente), e os sistemas A-* faziam substituição e ligação de rotinas — mais próximos de um macroexpansor/linker que de um compilador. O termo "compilador" na época significava literalmente *compilar* (juntar) trechos de biblioteca.

### Questão 4 — Fortran e a resistência ao código traduzido

**Resposta breve:** em 1954, memória e tempo de máquina eram muito mais caros que o programador, e existia a convicção difundida de que nenhum tradutor automático produziria código tão eficiente quanto um humano escrevendo montagem. O time de Backus sabia que **desempenho era a condição de adoção**, não um detalhe: se o código gerado fosse sensivelmente mais lento, ninguém migraria.
**Relação entre os três fatores:** por isso investiram um esforço desproporcional (cerca de 18 pessoas-ano) na **otimização** do compilador — o primeiro otimizador da história. Quando o Fortran I entregou código competitivo com o manual, a equação virou: o **custo de programação** (dias em vez de semanas, menos erros, código legível) passou a dominar, e a **adoção** foi rápida.
**Por que a resposta é essa:** o caso Fortran mostra que a adoção de uma abstração depende de ela ser **barata o suficiente**; abstração que custa desempenho demais é rejeitada — regra que se repete em Java (até o JIT), em C++ ("não pague pelo que não usa") e em Rust ("abstrações de custo zero").

### Questão 7 — COBOL, domínio e público

**Resposta breve:** COBOL (1960, comitê CODASYL, com forte influência do **FLOW-MATIC** de Grace Hopper) foi projetada para **processamento comercial de dados** e para um público de gestores e programadores de negócio, não de cientistas.
**Como domínio e público moldaram o projeto:**
(1) **Legibilidade acima de concisão** — sintaxe em inglês quase natural (`MULTIPLY HORAS BY VALOR GIVING SALARIO`), com a ideia explícita de que gerentes pudessem ler o código; a estrutura em divisões (IDENTIFICATION, ENVIRONMENT, DATA, PROCEDURE) separa descrição de dados de lógica.
(2) **Registros hierárquicos** — a inovação técnica principal: dados comerciais são naturalmente hierárquicos (funcionário → endereço → cidade), e COBOL introduziu registros com níveis numerados e `PICTURE` para formatação precisa de valores monetários — algo que Fortran, com apenas arrays numéricos, não conseguia expressar.
(3) **Relação com FLOW-MATIC** — foi a base direta: dele vêm a sintaxe em inglês, a separação entre descrição de dados e processamento e o foco em arquivos.
**Por que a resposta é essa:** COBOL é o exemplo mais claro do capítulo de que **público-alvo é decisão de projeto**: verbosidade não é defeito, é requisito atendido.

### Questão 8 — Basic vs. PL/I

**Resposta breve:** ambas respondem à ampliação do alcance da programação, mas em direções opostas. **Basic** (1964, Kemeny e Kurtz, Dartmouth) queria **ampliar o público**: alunos de humanas deveriam programar em uma sessão de tempo compartilhado. **PL/I** (1964, IBM) queria **ampliar o domínio**: uma única linguagem para substituir Fortran (científico) e COBOL (comercial) de uma vez, junto com o hardware unificado System/360.
**Compromissos de projeto:** Basic trocou **poder por acessibilidade** — sintaxe mínima, sem estruturas de dados sofisticadas, mensagens de erro amigáveis, resposta imediata; o custo foi uma linguagem inadequada a programas grandes (sem estruturação, com `GOTO` onipresente). PL/I trocou **coerência por abrangência** — juntou recursos de Fortran, COBOL e ALGOL 60 e acrescentou concorrência e exceções, resultando em uma linguagem enorme, de compilador caro e semântica cheia de conversões automáticas surpreendentes.
**Lição:** Basic errou por menos e PL/I por mais. A moral do capítulo é que **linguagem sem foco de domínio** tende a ficar grande demais para dominar, enquanto simplicidade excessiva limita o crescimento — e é por isso que a resposta seguinte da história foi Pascal e C, ambas de escopo deliberadamente contido.

### Questão 13 — Ada e sistemas críticos

**Resposta breve:** Ada (1980/1983) nasceu do maior processo de requisitos já feito para uma linguagem: o Departamento de Defesa dos EUA usava centenas de linguagens em sistemas embarcados e queria uma só, com competição pública de propostas (venceu a equipe de Jean Ichbiah).
**Como cada recurso responde ao domínio crítico:**
(1) **Confiabilidade** é o requisito-raiz — falha em aviônica ou armamento custa vidas, então a linguagem prefere **detectar erros em compilação** a ser conveniente.
(2) **Tipos** fortes com **subfaixas** e **tipos derivados** permitem declarar que uma variável de altitude não pode receber uma de velocidade, mesmo ambas sendo numéricas — o compilador vira verificador de requisitos de domínio.
(3) **Pacotes** oferecem encapsulamento com separação entre especificação e corpo, viabilizando desenvolvimento em equipes grandes e compilação separada verificada — resposta direta à escala do projeto.
(4) **Concorrência** com *tasks* e *rendezvous* está **na linguagem**, não em biblioteca, porque sistemas embarcados são inerentemente concorrentes e deixar isso a cargo do SO destruiria a portabilidade e a verificabilidade.
Somam-se a isso o tratamento de **exceções** e o suporte a genéricos.
**Por que a resposta é essa:** Ada é o exemplo do capítulo em que **requisitos precederam o projeto** — e o resultado foi uma linguagem grande e cara de implementar, que perdeu mercado para C++ mas segue insubstituída onde a certificação é obrigatória (SPARK/Ada em aviônica).

### Questão 15 — Java e a mudança de contexto

**Resposta breve:** segundo Sebesta, **em 1990 a Sun Microsystems** identificou a necessidade de uma linguagem para **dispositivos eletrônicos embarcados de consumo** — torradeiras, fornos de micro-ondas, sistemas interativos de TV. C e C++ foram avaliadas e consideradas insatisfatórias para esse fim, e daí nasceu o projeto que viria a ser Java (inicialmente chamado *Oak*). Esse mercado não decolou. Entre 1993 e 1995 a Web explodiu, e a Sun percebeu que os requisitos do domínio embarcado — **independência de arquitetura** (muitos processadores diferentes), **código compacto transportável**, **confiabilidade** e **segurança** (executar código de terceiros sem quebrar o aparelho) — eram exatamente os requisitos de **baixar e executar código vindo de um servidor desconhecido**. Java foi relançada com os *applets* e o navegador HotJava.
**Como contexto reposiciona uma linguagem:** o **produto não mudou; o problema do mundo mudou** e passou a coincidir com ele. Um conjunto de decisões técnicas tomadas por um motivo (portar para muitos chips) revelou-se valioso por outro (executar código não confiável na rede). Depois, quando os applets morreram, Java se reposicionou uma terceira vez — no **servidor corporativo** (J2EE) e no **Android** —, sobrevivendo a duas mortes do seu domínio.
**Por que a resposta é essa:** a questão testa se o aluno entende que adoção é **sincronia entre projeto técnico e demanda histórica**, e não mérito isolado — o mesmo argumento da Questão 1.

### Questão 17 — C# e o ambiente .NET

**Resposta breve:** C# (2000, Anders Hejlsberg, Microsoft) foi apresentada como evolução de C++ e Java dentro do .NET, buscando produtividade sem abandonar controle.
**Duas decisões comparadas:**

1. **Unificação de tipos e `struct` como tipo de valor** — *Java* separou tipos primitivos (`int`) de objetos (`Integer`), o que quebra a uniformidade: coleções não aceitavam primitivos e cada elemento virava um objeto alocado no heap. C# fez **todo tipo derivar de `object`**, incluindo os primitivos (com `boxing`/`unboxing`), e ofereceu `struct` para tipos de valor definidos pelo usuário. *Problema resolvido:* uniformidade conceitual e **desempenho de memória** (evita alocação e indireção para tipos pequenos, algo que Java só atenuou muito depois).
2. **Delegates e properties** — em *Java*, um callback exigia uma interface e uma classe anônima inteira, e o acesso a um campo exigia escrever `getX()`/`setX()` manualmente. C# introduziu **delegates** (referências a métodos com tipo verificado, mais seguras que os ponteiros de função de C++) e **properties** (sintaxe de campo com semântica de método). *Problema resolvido:* verbosidade cerimonial e a impossibilidade de tratar métodos como valores — o que abriu caminho para eventos, LINQ e expressões lambda.

*(Outras comparações válidas, todas citadas por Sebesta: o **`struct`** de C#, que foi modificado a ponto de virar "uma construção verdadeiramente útil", enquanto em C++ é "praticamente inútil"; os tipos **`enum`**, mais seguros por nunca serem convertidos implicitamente para inteiros; a sentença **`foreach`** de iteração; e, fora do livro, genéricos **reificados** em C# contra a **type erasure** de Java, além de `unsafe` como via de escape herdada de C++.)*
**Observação de contexto:** Sebesta registra que os projetistas de C# **discordaram das remoções feitas por Java** em relação a C++ — todos aqueles recursos, exceto a herança múltipla, foram reincluídos em C#.
**Por que a resposta é essa:** C# ilustra a **influência corretiva** — surge depois, observa os pontos de atrito das antecessoras e os corrige, ao custo de amarrar-se a uma plataforma proprietária (limitação depois desfeita pelo .NET Core).

### Questão 18 — XSLT e JSP

**Resposta breve:**

| | **XSLT** | **JSP** |
|---|---|---|
| **Entrada** | Um documento **XML** + uma folha de estilo XSLT (que também é XML) | Uma página **HTML** com código Java embutido + a requisição HTTP |
| **Processamento** | **Declarativo por casamento de padrões**: templates casam com nós da árvore XML (via XPath) e são disparados pelo processador, sem laços explícitos | **Imperativo**: a página é traduzida em um **servlet Java**, compilada e executada no servidor |
| **Saída** | Outro documento (XML, HTML ou texto) — é uma **transformação de documento** | **HTML gerado dinamicamente**, enviado ao navegador |

**Por que ambas são híbridas de marcação e programação:** em ambas, o documento é o **esqueleto** e as construções de programação são **inseridas dentro dele**. XSLT usa sintaxe XML para expressar seleção, iteração e recursão (`<xsl:if>`, `<xsl:for-each>`, `<xsl:template>`) — é uma **linguagem de programação escrita em marcação**, declarativa e Turing-completa. JSP usa marcação HTML como molde com **ilhas de código Java** (`<% ... %>`, JSTL). A diferença essencial: em XSLT a marcação **é** a linguagem; em JSP a marcação **hospeda** a linguagem.
**Por que a resposta é essa:** as duas mostram que o limite entre "descrever documento" e "computar" é permeável — e que a Web forçou o surgimento dessa categoria intermediária.

### Questão 19 — Linha do tempo com oito linguagens e quatro paradigmas

```
1957  Fortran      [imperativo/numérico]
        │ (a) influência conceitual: prova que compilação eficiente é viável
        ▼
1960  ALGOL 60     [imperativo estruturado]
        │ (b) herança sintática e de escopo, com simplificação (via ALGOL-W)
        ├──────────────► 1971 Pascal    [imperativo estruturado / ensino]
        │ (d) extensão de propósito: ALGOL + classes para simulação
        ├──────────────► 1967 SIMULA 67 [orientado a objetos]
        │                      │ (e) radicalização conceitual: OO pura, tudo é objeto
        │                      ▼
        │                1980 Smalltalk  [OO puro]
        │                      │ (f) síntese pragmática: objetos de Smalltalk
        │                      │      + sintaxe e desempenho de C
        │                      ▼
        │                1983 C++ ──(g) influência corretiva──► 1995 Java [OO + VM portátil]
        │                                (remove ponteiros, adiciona GC e bytecode)
        │
        └──► 1968 ALGOL 68 ──(c) herança com inversão de prioridade (via CPL/BCPL/B)──►
                             1972 C  [imperativo de sistemas]
                             (eficiência e acesso à máquina > rigor)
                                     ▲
                                     └── (f) também contribui para C++ (sintaxe e desempenho)

1959  Lisp         [funcional]  ──(h) fundamentação teórica: cálculo lambda aplicado──►
                                       influencia Scheme, ML, Haskell e o retorno
                                       do estilo funcional a Python/Java/JS

1972  Prolog       [lógico]     ──(i) ruptura de modelo: lógica de predicados
                                       no lugar da máquina de von Neumann
```

**Tipos de influência usados (o ponto da questão é este, não a cronologia):**
(a) **prova de viabilidade** — Fortran não deu sintaxe a ALGOL, deu legitimidade à compilação;
(b) **herança com simplificação deliberada** (ALGOL 60 → Pascal);
(c) **herança com inversão de prioridade** (ALGOL → C: mantém a estrutura, troca rigor por acesso à máquina);
(d) **extensão de propósito** (ALGOL 60 → SIMULA 67: acrescenta `class` para simular entidades);
(e) **radicalização conceitual** (SIMULA → Smalltalk: de recurso a princípio único);
(f) **síntese pragmática** (Smalltalk + C → C++);
(g) **influência corretiva** (C++ → Java: remove o que causava erro);
(h) **fundamentação teórica** (Lisp: modelo matemático que reaparece décadas depois em linguagens de outra família);
(i) **ruptura de modelo** (Prolog: não descende de ALGOL, descende da lógica formal).

**Quatro paradigmas cobertos:** imperativo/procedural (Fortran, ALGOL, Pascal, C), orientado a objetos (SIMULA, Smalltalk, C++, Java), funcional (Lisp) e lógico (Prolog).
**Por que a resposta é essa:** o enunciado proíbe "apenas setas cronológicas" justamente porque influência tem **tipos qualitativamente diferentes** — inclusive influência *negativa* (ALGOL 68 motivando Pascal por rejeição) e influência *retardada* (Lisp voltando décadas depois).

---

## Referência

SEBESTA, Robert W. **Conceitos de Linguagens de Programação**. Porto Alegre: Bookman. Capítulo 2 — *Evolução das Principais Linguagens de Programação*.
