# Resumo Autoexplicativo — 20 Questões do Capítulo 2 (Sebesta)

**Disciplina:** Paradigmas de Linguagens de Programação
**Professor:** Munif Gebara Junior
**Aluno:** João Miguel Silva Salvalagio
**Base:** SEBESTA, Robert W. *Conceitos de Linguagens de Programação*, 11ª ed. — Capítulo 2, *Evolução das principais linguagens de programação* (páginas 49–118 do PDF)

---

## Como usar este documento

Este é um **material de revisão**: cada uma das 20 questões da lista aparece condensada em quatro blocos fixos, para leitura rápida antes da prova.

| Bloco | O que traz |
|---|---|
| **O que se pergunta** | O enunciado reduzido ao que realmente está sendo cobrado |
| **Conceito central** | A ideia única que a questão testa |
| **Resposta em síntese** | O conteúdo mínimo que uma resposta correta precisa ter |
| **Por que isso importa** | A ligação com paradigmas — o motivo de a questão existir |

As respostas desenvolvidas de 10 questões estão no arquivo **`exercicios-cap02.md`**, na mesma pasta. Este resumo cobre **todas as 20**.

### Índice

**Seção 1 — Origens (1 a 4):** genealogia · Plankalkül · pseudocódigos · Fortran
**Seção 2 — Os primeiros paradigmas (5 a 10):** Lisp · ALGOL 60 · COBOL · Basic/PL-I · APL/SNOBOL/SIMULA · ALGOL 68
**Seção 3 — Consolidação (11 a 15):** Pascal/C/Prolog · programação lógica · Ada · Smalltalk/C++/Java · Java e a Web
**Seção 4 — Era moderna (16 a 20):** scripting · C# · XSLT/JSP · linha do tempo · estudo de caso

---

# Seção 1 — Origens

## Questão 1 — A genealogia não é uma escada de progresso

**O que se pergunta:** por que linguagens novas não tornam as antigas obsoletas, e quais fatores fazem uma influenciar a outra sem substituí-la.

**Conceito central:** **influência ≠ substituição.** A história das linguagens é uma árvore com ramos paralelos, não uma linha ascendente.

**Resposta em síntese:**
Três evidências contra a "escada": (1) **coexistência** — Fortran (1957), Lisp (1959) e COBOL (1960) nasceram quase juntos, para domínios diferentes, e os três seguem vivos; (2) **influência sem adoção** — ALGOL 60 nunca dominou o mercado e mesmo assim moldou quase toda linguagem imperativa; (3) **retorno de ideias antigas** — recursos do Lisp de 1959 (lambda, coleta de lixo, funções de primeira classe) voltaram como novidade em Java 8 e no JavaScript moderno.
Dois fatores de influência sem substituição: **nicho de domínio consolidado** (COBOL em bancos — migrar custa mais que manter) e **custo de base instalada** (Fortran mantém BLAS/LAPACK, que linguagens novas preferem *chamar* a reescrever).

**Por que isso importa:** é a moldura de todo o capítulo. Ensina a nunca dizer "X é melhor que Y" sem completar "para qual domínio". Uma linguagem morre por perda de domínio e comunidade, não por existir uma melhor.

---

## Questão 2 — Plankalkül

**O que se pergunta:** por que uma linguagem nunca implementada é historicamente relevante, e quais recursos ela antecipou.

**Conceito central:** **mérito técnico ≠ influência histórica.** Uma ideia certa na época errada não influencia ninguém.

**Resposta em síntese:**
Konrad Zuse projetou Plankalkül ("cálculo de programas") entre 1943 e 1945 na Alemanha isolada pela guerra; o manuscrito só foi publicado em **1972**. Nunca foi implementada e não influenciou ninguém em sua época.
**Três recursos antecipados:** (1) tipos de dados estruturados construídos a partir do bit, incluindo **vetores e registros** (o que hoje chamamos de `struct`); (2) instrução de seleção **sem cláusula `else`**; (3) uma forma de **asserção matemática** sobre o programa.
**Valor do principal deles:** os registros e vetores antecipam em mais de uma década a ideia de que o programador deve modelar dados no vocabulário **do problema**, não no da memória da máquina — o núcleo de toda abstração de dados posterior.

**Por que isso importa:** mostra que difusão de ideias depende de **circunstância** (guerra, isolamento, ausência de hardware) tanto quanto de qualidade. É o contraponto perfeito da Questão 1: aqui há mérito sem influência; em ALGOL 60, influência sem mercado.

---

## Questão 3 — Short Code, Speedcoding e os sistemas A-0/A-1/A-2

**O que se pergunta:** que problema cada um enfrentava, que estratégia adotou, e por que chamá-los de "compiladores modernos" é impreciso.

**Conceito central:** os três atacam o **mesmo problema** — programar em código de máquina é lento e propenso a erro — com **três estratégias diferentes**.

**Resposta em síntese:**

| Sistema | Autor / Ano | Estratégia |
|---|---|---|
| **Short Code** | John Mauchly, 1949 (BINAC, depois UNIVAC I) | **Interpretado**: expressões matemáticas em versões codificadas, decodificadas em tempo de execução — muito lento |
| **Speedcoding** | John Backus, 1954 (IBM 701) | **Interpretador que simulava uma máquina virtual** de ponto flutuante de três endereços em hardware que só tinha inteiros; incluía incremento automático de registradores de endereço, recurso que só apareceu em hardware em 1962 |
| **A-0 / A-1 / A-2** | Grace Hopper, 1951–53 (UNIVAC) | **Expansão de subrotinas**: pseudocódigos eram substituídos por rotinas de biblioteca pré-escritas, com ajuste de argumentos |

**Por que "compilador moderno" é impreciso:** um compilador moderno faz análise **léxica, sintática e semântica** de uma linguagem com **gramática formal** e **gera código otimizado**. Nenhum dos três fazia isso — dois **interpretavam em execução** (não traduziam previamente) e os sistemas A-* faziam substituição e ligação de rotinas, mais próximos de um macroexpansor/linker. O verbo "compilar" ali tinha o sentido literal de **juntar** trechos de biblioteca.

**Por que isso importa:** ensina que termos técnicos mudam de significado com o tempo, e que a fronteira interpretar/traduzir/ligar estava sendo inventada naquele momento.

---

## Questão 4 — Fortran e a resistência ao código traduzido

**O que se pergunta:** por que o projeto Fortran precisou provar que código traduzido competia com montagem manual, relacionando desempenho, custo de programação e adoção.

**Conceito central:** **abstração só é adotada quando é barata o suficiente.**

**Resposta em síntese:**
Em 1954, tempo de máquina e memória eram muito mais caros que o programador, e havia a convicção difundida de que nenhum tradutor automático geraria código tão eficiente quanto um humano escrevendo montagem. O time de Backus entendeu que **desempenho era a condição de adoção**, não um detalhe: código gerado sensivelmente mais lento seria rejeitado.
Por isso, **grande parte do esforço de 18 trabalhadores/ano** do primeiro compilador foi gasta em **otimização** — o primeiro otimizador da história, e "extraordinariamente eficaz" nas palavras do livro. Quando o Fortran I entregou código competitivo com o manual, a equação econômica virou: o **custo de programação** (dias em vez de semanas, menos erros, código legível e manutenível) passou a dominar, e a **adoção foi rápida**.

**Por que isso importa:** é uma regra que se repete em toda a história. Java só decolou com o **JIT**; C++ adotou o lema "não pague pelo que não usa"; Rust vende "abstrações de custo zero". Toda geração renegocia o mesmo trade-off.

---

# Seção 2 — Os primeiros paradigmas

## Questão 5 — Lisp e Fortran: dois mundos

**O que se pergunta:** comparar domínio, representação de dados e estilo de computação das duas.

**Conceito central:** a **primeira bifurcação de paradigmas da história** — imperativo orientado a máquina × funcional orientado a símbolos.

**Resposta em síntese:**

| Eixo | **Fortran** (1957, Backus/IBM) | **Lisp** (1959, McCarthy/MIT) |
|---|---|---|
| Domínio | Computação **numérica** científica | **Processamento simbólico** e IA |
| Motivação | **Econômica**: baratear a programação | **Teórica**: base em funções recursivas |
| Dados | Estáticos e homogêneos: escalares, vetores, matrizes de tamanho fixo, colados na memória | Uma estrutura dinâmica: a **lista encadeada** (célula *cons*), com **coleta de lixo** |
| Estilo | **Imperativo**: comandos, atribuição destrutiva, laços `DO` — modifica **estado** | **Funcional**: aplicação de funções, **recursão**, expressões que **produzem valores** |
| Código × dados | Fronteira rígida | **Homoiconicidade**: o programa é uma lista, então programas podem gerar programas (`eval`, macros) |

**A causa da diferença:** o domínio numérico tem tamanho **conhecido antecipadamente** e exige desempenho máximo → estruturas estáticas. O domínio simbólico tem tamanho **imprevisível** (uma árvore de dedução cresce conforme a busca) → alocação dinâmica e coleta de lixo, pagando em desempenho pelo ganho em expressividade.

**Por que isso importa:** de Fortran vem a tradição de desempenho e compilação otimizada; de Lisp vêm **coleta de lixo, tipagem dinâmica, funções de primeira classe, `if` como expressão e o REPL** — hoje presentes em Python, JavaScript, Ruby, C#, Java e Rust. Ideias de 1959 que levaram cinquenta anos para virar mainstream.

---

## Questão 6 — ALGOL 60: influência sem mercado

**O que se pergunta:** três contribuições que ultrapassaram sua adoção, e por que influência não depende de dominar o mercado.

**Conceito central:** **adoção e influência têm causas diferentes.**

**Resposta em síntese:**
**Três contribuições:**
1. **BNF (Backus-Naur Form)** — primeira sintaxe definida por **gramática formal** em vez de prosa. Consequência: fim da ambiguidade nas especificações, geração automática de analisadores (yacc, ANTLR, bison) e nascimento da área de compiladores como ciência.
2. **Estrutura de blocos e escopo léxico** — `begin...end` com declarações locais e aninhamento criou variável local, tempo de vida automático em pilha e visibilidade estática. É o alicerce da **programação estruturada**; toda linguagem com `{ }` herda esse modelo.
3. **Recursão e passagem de parâmetros formalizadas** — procedimentos recursivos (que Fortran não tinha) exigiram registro de ativação em pilha, o modelo de execução usado até hoje.

**Por que influência ≠ mercado:** **adoção** depende de fatores **extrínsecos** — apoio de fabricante, compiladores, base instalada, bibliotecas, E/S padronizada, marketing. ALGOL falhou em todos (sem IBM, sem E/S padrão, sem base instalada). **Influência** depende de fatores **intrínsecos** — clareza conceitual, boas abstrações, especificação precisa. ALGOL foi excelente nisso e virou a linguagem padrão para **publicar algoritmos** em periódicos, formando uma geração inteira de projetistas.

**Por que isso importa:** o livro afirma que **"todas as linguagens imperativas devem algo de seu projeto a ALGOL 60 e/ou ALGOL 68"**. Casos análogos: SIMULA 67 (pouco usada, criou a OO), Smalltalk (nicho, definiu a OO pura), ML (acadêmica, gerou a inferência de tipos de Rust, Swift e TypeScript).

---

## Questão 7 — COBOL: domínio e público como decisão de projeto

**O que se pergunta:** como o domínio comercial e o público-alvo moldaram a legibilidade, os registros e a relação com FLOW-MATIC.

**Conceito central:** **público-alvo é decisão de projeto** — verbosidade não é defeito quando o requisito é ser lida por não programadores.

**Resposta em síntese:**
COBOL (1960, comitê **CODASYL**) foi projetada para **processamento comercial de dados**, com público de gestores e programadores de negócio, não cientistas.
1. **Legibilidade acima de concisão** — sintaxe em inglês quase natural (`MULTIPLY HORAS BY VALOR GIVING SALARIO`), com a intenção explícita de que gerentes pudessem ler o código; a estrutura em quatro divisões (IDENTIFICATION, ENVIRONMENT, DATA, PROCEDURE) separa descrição de dados de lógica de processamento.
2. **Registros hierárquicos** — a inovação técnica principal. Dados comerciais são naturalmente hierárquicos (funcionário → endereço → cidade), e COBOL introduziu registros com **níveis numerados** e a cláusula **`PICTURE`** para formatação exata de valores monetários. Fortran, com apenas arrays numéricos, não conseguia expressar isso.
3. **Relação com FLOW-MATIC** — o livro a chama de "principal precursora de COBOL": linguagem compilada para negócios implementada em **1957** por Grace Hopper, mas proprietária da UNIVAC. Dela vêm a sintaxe em inglês, a separação entre descrição de dados e processamento, e o foco em arquivos.

**Por que isso importa:** COBOL é o caso mais claro de que o **domínio determina o projeto**. E explica a Questão 1 na prática: ela segue insubstituída em bancos e folhas de pagamento porque ninguém reescreve esse legado.

---

## Questão 8 — Basic e PL/I: dois modos de ampliar o alcance

**O que se pergunta:** comparar as duas como respostas ao desejo de ampliar acesso ou alcance, identificando o compromisso de cada uma.

**Conceito central:** **ampliar o público** e **ampliar o domínio** são objetivos opostos, e cada um cobra um preço diferente.

**Resposta em síntese:**

| | **Basic** (1964, Kemeny e Kurtz, Dartmouth) | **PL/I** (1964, IBM) |
|---|---|---|
| **Objetivo** | Ampliar o **público**: alunos de humanas programando em tempo compartilhado | Ampliar o **domínio**: uma linguagem só para substituir Fortran (científico) e COBOL (comercial), junto com o System/360 |
| **Compromisso** | Trocou **poder por acessibilidade** | Trocou **coerência por abrangência** |
| **O que ganhou** | Sintaxe mínima, erros amigáveis, resposta imediata, acesso democratizado | Recursos de Fortran + COBOL + ALGOL 60, mais concorrência e exceções |
| **O que pagou** | Inadequada a programas grandes: sem estruturação, `GOTO` onipresente | Linguagem enorme, compilador caro, semântica cheia de conversões automáticas surpreendentes |

**Nota do livro:** PL/I teve aceitação bem maior que ALGOL 68 — mas por **esforço promocional da IBM**, não por mérito de projeto. Fortran não podia ser a linguagem universal justamente por ser propriedade exclusiva da IBM.

**Por que isso importa:** **Basic errou por menos, PL/I errou por mais.** A resposta seguinte da história foram Pascal e C, ambas de **escopo deliberadamente contido** — o que confirma a lição.

---

## Questão 9 — APL, SNOBOL e SIMULA 67: três direções

**O que se pergunta:** associar cada linguagem ao seu foco e apontar uma contribuição duradoura.

**Conceito central:** linguagens de **propósito especial** provam que o domínio molda o projeto — e duas delas fundaram paradigmas.

**Resposta em síntese:**

| Linguagem | Ano / Autor | Foco | Contribuição |
|---|---|---|---|
| **APL** | 1962 · Kenneth Iverson (IBM), descrita no livro *A Programming Language* | Matrizes e vetores; notação matemática executável | **Programação orientada a arrays**: operar sobre coleções inteiras sem laço (`+/V` soma um vetor). Ressurge em NumPy, MATLAB, R, pandas e GPUs |
| **SNOBOL** | 1964 · Bell Labs (Farber, Griswold, Polonsky) | Processamento de **texto e cadeias** | **Casamento de padrões como operação primitiva** — linhagem que leva às expressões regulares e ao pattern matching de Haskell, Rust e Python 3.10 |
| **SIMULA 67** | 1967 · Dahl e Nygaard (Noruega); extensão de ALGOL 60 | **Simulação de eventos discretos** | **A classe e o objeto** — nascimento da **Orientação a Objetos**, além de corrotinas |

**Ressalva importante do próprio Sebesta:** o livro afirma que **"nem APL nem SNOBOL tiveram muita influência sobre as principais linguagens posteriores"** — a descendência direta é estreita (**J** vem de APL; **ICON** e parcialmente **AWK** vêm de SNOBOL). E SIMULA 67 "nunca atingiu amplo uso e teve pouco impacto na computação de sua época". Distinga então:
- **SIMULA 67** = influência **conceitual comprovada**: `class` foi retomada por Smalltalk e C++, e o conceito de **abstração de dados** só foi reconhecido como tal por **Hoare, em 1972**, olhando para ela.
- **APL e SNOBOL** = influência **de ideia, não de linhagem**: nenhuma linguagem principal descende delas, mas o estilo vetorial e o pattern matching foram **redescobertos** depois.

**Por que isso importa:** repare no padrão — **ninguém inventou a OO para "organizar software"**. Ela nasceu como técnica de modelagem de simulação; o uso em engenharia de software veio depois, com Smalltalk e C++.

---

## Questão 10 — Ortogonalidade e o caso ALGOL 68

**O que se pergunta:** definir ortogonalidade, usar ALGOL 68 para separar regularidade de simplicidade, e responder se ortogonalidade garante facilidade de uso.

**Conceito central:** **regularidade é sobre as regras; simplicidade é sobre a cabeça de quem programa.** Não são a mesma coisa.

**Resposta em síntese:**
**Definição:** ortogonalidade é a propriedade de um conjunto **pequeno** de construções primitivas poder ser **combinado livremente**, sem exceções, com significado previsível a partir das partes. Exige duas coisas: poucos conceitos primitivos **e** combinação irrestrita.
*Falta de ortogonalidade (C):* uma função pode retornar `struct` mas não array; um array decai para ponteiro em certos contextos e não em outros — exceções a decorar.

**ALGOL 68** teve a ortogonalidade como **critério de projeto declarado**: poucos conceitos (modos, valores, coerções) combinados quase sem restrição, com **tipos definidos pelo usuário** como resultado direto disso. Sebesta registra que a linguagem "conseguiu boa facilidade de escrita por meio do princípio da ortogonalidade" e que seu uso dela, "que alguns podem considerar excessivo, foi revolucionário".
**E ainda assim foi pouco adotada**, por dois motivos que o livro aponta: os documentos (escritos com a **gramática de dois níveis** de van Wijngaarden) foram "desafiadores para a comunidade de computação", e a combinação livre gerou **complexidade emergente** — regras uniformes aplicadas a tudo produzem uma explosão de combinações válidas mas obscuras.

| | **Regularidade** | **Simplicidade** |
|---|---|---|
| O que é | Ausência de exceções | Baixo esforço cognitivo para usar |
| Beneficia | Projetista e implementador do compilador | Usuário da linguagem |

**Resposta direta:** **não.** Ortogonalidade reduz a complexidade da **especificação**, mas não a do **uso** — pode até aumentá-la. É virtude necessária, não suficiente: precisa vir com sintaxe legível, documentação acessível e restrições **deliberadas**. Prova histórica: **Pascal** nasceu como reação explícita à complexidade de ALGOL 68 — menos ortogonal, mais restritiva, muito mais adotada.

**Por que isso importa:** é a única questão da lista sobre **critérios de projeto** (Capítulo 1) aplicados a um caso real. Usabilidade = ortogonalidade + legibilidade + escala controlada.

---

# Seção 3 — Consolidação

## Questão 11 — A cadeia ALGOL → Pascal → C, contra Prolog

**O que se pergunta:** montar a cadeia de influência imperativa e contrastá-la com a proposta declarativa.

**Conceito central:** a linhagem imperativa **abstraiu a máquina sem sair do modelo da máquina**; Prolog **trocou o modelo**.

**Resposta em síntese:**
**A cadeia** (com o tipo de cada influência — o que a questão realmente cobra):

| Ligação | Tipo de influência |
|---|---|
| ALGOL 60 → **ALGOL-W** → Pascal (1971, Wirth) | **Herança com simplificação deliberada**: mantém blocos e escopo, reduz recursos, reforça tipos |
| ALGOL 68 → complexidade → Pascal | **Influência negativa**: reagir a um projeto também é influência |
| **ALGOL 68** → CPL → BCPL → B → C (1972, Ritchie) | **Herança com inversão de prioridade**: mantém a estrutura, troca rigor por eficiência e acesso à máquina (C nasceu para escrever o Unix) |
| Pascal + C + SIMULA → Java / C# | **Síntese**: sintaxe de C + disciplina de tipos de Pascal + objetos de SIMULA/Smalltalk |

> **Precisão histórica:** o livro lista os ancestrais de C como **CPL, BCPL, B e ALGOL 68** — a influência chega por **ALGOL 68**, não diretamente de ALGOL 60. Pascal vem de ALGOL 60 por ALGOL-W. Mesmo tronco, ramos diferentes.

**O núcleo imperativo compartilhado:** variável = célula de memória nomeada; **atribuição destrutiva** como operação central; execução sequencial com controle explícito; subprogramas em pilha. O programa é uma **receita** — descreve o *como*.

**O contraste com Prolog** (1972, Colmerauer e Roussel; base teórica de Kowalski), que **não descende de ALGOL** mas da **lógica de predicados de primeira ordem**:

| | Linhagem ALGOL→Pascal→C | **Prolog** |
|---|---|---|
| Raiz | Máquina de von Neumann | Lógica de primeira ordem / resolução SLD |
| O programador escreve | A sequência de passos (*como*) | As relações verdadeiras (*o quê*) |
| Variável | Endereço **remutável** | Incógnita **unificada uma vez** |
| Controle | Explícito (laços, `if`) | **Implícito**: busca e **backtracking** do motor |
| Resultado | Único e determinístico | **Várias soluções**; pode falhar e retroceder |
| Direção | Entradas e saídas fixas | **Reversível**: a mesma relação responde em vários sentidos |

```c
/* C — descrevo o algoritmo */
Pessoa* avo(Pessoa* p) {
    if (p == NULL || p->pai == NULL) return NULL;
    return p->pai->pai;
}
```
```prolog
% Prolog — descrevo a relação; a busca é do motor
avo(X, Y) :- progenitor(X, Z), progenitor(Z, Y).
```
A cláusula Prolog responde `avo(joao, Q)` **e** `avo(Q, pedro)`. Em C, cada pergunta exigiria uma função.

**Por que isso importa:** o preço da troca de modelo é perder **previsibilidade de desempenho** e precisar de recursos extralógicos (o corte `!`) quando a busca automática não basta.

---

## Questão 12 — Uma base Prolog e o que a torna "programação lógica"

**O que se pergunta:** modelar dois fatos, uma regra e uma consulta, e justificar por que isso é programação lógica e não armazenamento de dados.

**Conceito central:** banco de dados **recupera** o que foi gravado; programação lógica **deriva** o que nunca foi gravado.

**Resposta em síntese:**
Modelo em linguagem natural (pré-requisitos de disciplinas):
- *Fato 1:* Algoritmos é pré-requisito de Estruturas de Dados.
- *Fato 2:* Estruturas de Dados é pré-requisito de Paradigmas.
- *Regra:* A é **dependência** de C se A é pré-requisito direto de C, **ou** se A é pré-requisito de algum B que tem C como dependente (fecho transitivo).
- *Consulta:* o que preciso ter cursado antes de Paradigmas?

```prolog
prerequisito(algoritmos, estruturas_dados).
prerequisito(estruturas_dados, paradigmas).

dependencia(A, C) :- prerequisito(A, C).
dependencia(A, C) :- prerequisito(A, B), dependencia(B, C).

?- dependencia(X, paradigmas).
X = estruturas_dados ;
X = algoritmos ;
false.
```

**Como o motor chega lá:** unifica o objetivo com a 1ª cláusula → `estruturas_dados`; ao pedir outra solução, faz **backtracking** para a 2ª cláusula, unifica com o Fato 1 e prova recursivamente → `algoritmos`; esgotadas as alternativas → `false`.

**Por que é programação lógica:**

| Critério | Banco de dados | **Programação lógica** |
|---|---|---|
| Conhecimento | Só o que foi gravado | `algoritmos` **não está na base** — foi **deduzido** |
| Recursão | Exige extensão especial | A regra é **recursiva por natureza** |
| Motor | Busca em índices | **Inferência**: unificação + resolução SLD + backtracking |
| Direção | Fixa | A mesma regra responde em vários sentidos |
| Poder | Não é linguagem completa | Prolog é **Turing-completo** |

**O ponto decisivo:** fatos + regras são **dados e programa ao mesmo tempo**. As cláusulas são fórmulas lógicas (**cláusulas de Horn**); executar o programa é **provar um teorema**, e `X = algoritmos` é a *testemunha* da prova.

**Por que isso importa:** note o que **não** foi escrito — nenhum laço, pilha de visitados, condição de parada ou ordem de percurso. Em linguagem imperativa, esse fecho transitivo seria uma busca em grafo escrita à mão. Esse é o ganho do paradigma lógico.

---

## Questão 13 — Ada: requisitos antes do projeto

**O que se pergunta:** como confiabilidade, tipos, pacotes e concorrência se relacionam ao domínio de sistemas críticos.

**Conceito central:** o **domínio crítico** é a variável que explica cada decisão da linguagem — quando falhar custa vidas, o projeto prefere **rigidez** a conveniência.

**Resposta em síntese:**
Ada (1980/1983) resultou do **maior esforço de projeto da história** (título da própria seção do livro): o Departamento de Defesa dos EUA usava centenas de linguagens em sistemas embarcados e conduziu uma competição pública de propostas, vencida pela equipe de **Jean Ichbiah**.

| Recurso | Como responde ao domínio |
|---|---|
| **Confiabilidade** | É o requisito-raiz. A linguagem prefere **detectar erro em compilação** a ser conveniente |
| **Tipos** fortes, com **subfaixas** e tipos derivados | Permitem declarar que uma variável de altitude não recebe uma de velocidade, ainda que ambas sejam numéricas — o compilador vira **verificador de requisitos de domínio** |
| **Pacotes** | Encapsulamento com separação entre especificação e corpo → compilação separada verificada e desenvolvimento em equipes grandes. Resposta direta à **escala** |
| **Concorrência** (*tasks* e *rendezvous*) | Está **na linguagem**, não em biblioteca, porque sistemas embarcados são inerentemente concorrentes; delegar ao SO destruiria portabilidade e verificabilidade |

Somam-se **exceções** e **genéricos**.

**Por que isso importa:** o custo dessa escolha foi uma linguagem grande e cara de implementar, que perdeu mercado para C++ — mas segue **insubstituída onde há certificação obrigatória** (SPARK/Ada em aviônica). Confirma a Questão 1: sobrevive quem domina um nicho.

---

## Questão 14 — Objetos em Smalltalk, C++ e Java

**O que se pergunta:** comparar o papel dos objetos nas três, incluindo o compromisso de C++ com C e a portabilidade de Java.

**Conceito central:** três **graus de compromisso** com a OO — pura, híbrida e obrigatória-mas-pragmática.

**Resposta em síntese:**

| Aspecto | **Smalltalk** (1980, Kay/Xerox PARC) | **C++** (1983, Stroustrup) | **Java** (1995, Gosling/Sun) |
|---|---|---|---|
| Grau de OO | **Pura**: tudo é objeto | **Híbrida**: OO opcional; C procedural é C++ válido | **Quase pura**: tudo em classes, mas com **tipos primitivos** |
| Computação | **Troca de mensagens** | Chamada de método | Chamada com despacho dinâmico |
| Vinculação | Totalmente **dinâmica** | **Estática por padrão**, dinâmica só com `virtual` | **Dinâmica por padrão** |
| Herança | Simples | **Múltipla** (problema do diamante) | Simples de classes + múltipla de **interfaces** |
| Memória | Coleta de lixo | **Manual** (`new`/`delete`, RAII) | **Coleta de lixo** |
| Execução | VM + imagem viva | **Nativa** | **Bytecode + JVM** |
| Prioridade | Expressividade | **Desempenho e compatibilidade** | **Portabilidade e segurança** |

**O compromisso de C++ com C:** ser (quase) um **superconjunto de C**, para que código, programadores e ferramentas migrassem sem reescrita. Daí o lema *"não pague pelo que não usa"* — métodos **não** são virtuais por padrão (evita o custo da vtable) e não há coletor de lixo obrigatório.
*Ganhos:* adoção imediata, desempenho de C, viabilidade em SO, jogos e embarcados. *Custos:* herdou toda a insegurança de C (ponteiro solto, estouro de buffer, comportamento indefinido), ficou enorme, e a OO virou **possibilidade, não disciplina**.

**A portabilidade de Java** (*write once, run anywhere*), em quatro mecanismos:
1. **Bytecode + JVM** — porta-se a JVM, não cada programa; o binário é o mesmo em todo lugar.
2. **Semântica fixa** — `int` é 32 bits **sempre**, contra o tamanho definido pela implementação em C/C++. Elimina uma classe inteira de bugs.
3. **Remoção do que depende da máquina** — sem ponteiros explícitos, aritmética de ponteiros, `goto`, herança múltipla de classes, sobrecarga de operadores ou memória manual.
4. **Biblioteca padrão uniforme** — rede, threads, coleções e GUI com o mesmo comportamento em todo sistema.
*Custo:* desempenho abaixo do nativo (mitigado pelo **JIT**), mais memória, pausas de GC (inviável em tempo real duro).

**Por que isso importa:** a trajetória Smalltalk → C++ → Java mostra o paradigma OO sendo **negociado** contra as restrições da indústria: pureza cedendo a desempenho, e depois a segurança e portabilidade.

---

## Questão 15 — Java: a Web reposicionou a linguagem

**O que se pergunta:** como mudanças de contexto podem reposicionar uma linguagem cuja primeira aplicação não era a Web.

**Conceito central:** adoção é **sincronia entre projeto técnico e demanda histórica** — não mérito isolado.

**Resposta em síntese:**
Segundo Sebesta, **em 1990 a Sun Microsystems** identificou a necessidade de uma linguagem para **dispositivos eletrônicos embarcados de consumo** — torradeiras, fornos de micro-ondas, sistemas interativos de TV. C e C++ foram avaliadas e consideradas **insatisfatórias** para isso. Nasceu daí o projeto que viria a ser Java (inicialmente chamado *Oak*). **Esse mercado não decolou.**
Entre 1993 e 1995 a Web explodiu, e ficou claro que os requisitos do domínio embarcado — **independência de arquitetura** (muitos processadores diferentes), **código compacto transportável**, **confiabilidade** e **segurança** (rodar código de terceiros sem quebrar o aparelho) — eram exatamente os requisitos de **baixar e executar código vindo de um servidor desconhecido**. Java foi relançada com os *applets* e o navegador HotJava.

**O mecanismo:** o **produto não mudou; o problema do mundo mudou** e passou a coincidir com ele. Decisões tomadas por um motivo (portar para muitos chips) revelaram-se valiosas por outro (executar código não confiável na rede). Depois, mortos os applets, Java se reposicionou **uma terceira vez** — servidor corporativo (J2EE) e **Android** —, sobrevivendo a duas mortes do próprio domínio.

**Por que isso importa:** é a Questão 1 vista pelo outro lado. Lá, uma linguagem sobrevive por dominar um nicho; aqui, sobrevive por **encontrar nichos novos** para as mesmas decisões técnicas.

---

# Seção 4 — Era moderna

## Questão 16 — Seis linguagens de scripting que não são iguais

**O que se pergunta:** comparar Perl, JavaScript, PHP, Python, Ruby e Lua por domínio inicial, estruturas de dados e implementação — sem concluir que são equivalentes.

**Conceito central:** "scripting" descreve um **modo de uso** (interpretada, tipagem dinâmica, ciclo rápido), **não um projeto comum**.

**Resposta em síntese:**

| Linguagem | Ano/Autor | Domínio inicial | Estruturas de dados | Implementação |
|---|---|---|---|---|
| **Perl** | 1987 · Larry Wall | Administração de sistemas Unix; texto e logs. Era originalmente uma **combinação de sh e awk**; depois virou linguagem CGI da Web | Escalares, **vetores** e **dispersões** (hashes) com sigilos `$ @ %`; **regex na sintaxe**; muitas variáveis implícitas | Interpretador de processo Unix; ecossistema CPAN |
| **JavaScript** | 1995 · Brendan Eich (Netscape; virou projeto conjunto com a Sun no fim de 1995, quando LiveScript foi renomeada) | **Interatividade no navegador** | **Objeto = mapa dinâmico**; herança por **protótipos**; funções de primeira classe com closures | Interpretador no navegador; hoje **JIT** (V8); também servidor (Node.js) |
| **PHP** | 1995 · Rasmus Lerdorf | **Páginas Web no servidor**: gerar HTML e falar com banco | **Vetor associativo unificado**: uma estrutura como lista e dicionário | **Embutida em HTML**; interpretada **no servidor Web** quando o documento é requisitado; estado por requisição |
| **Python** | 1991 · Guido van Rossum | **Propósito geral e ensino** (sucessora do ABC) | Conjunto rico e **distinto**: `list`, `tuple`, `dict`, `set`; indentação significativa | **Bytecode** em VM (CPython); C-API — origem de NumPy, pandas, TensorFlow |
| **Ruby** | 1995 · Yukihiro Matsumoto | **Produtividade do programador**; explodiu com Rails (2005) | **Tudo é objeto** (inclusive inteiros); **blocos** (closures); classes **abertas** | Interpretador de árvore → **VM de bytecode (YARV)**; forte **metaprogramação** |
| **Lua** | 1993 · PUC-Rio | **Extensão embarcada** de aplicações; jogos e dispositivos restritos | **Uma única estrutura: a `table`** — vetor, dicionário, objeto e classe; **metatabelas** dão herança | Traduzida em **código intermediário e interpretada**; **coleta de lixo**; apenas **21 palavras reservadas** |

**Onde está a diferença real:**
1. **Domínio molda a forma** — Perl otimiza texto (regex é sintaxe); PHP otimiza gerar HTML (o código mora no documento); JS otimiza eventos na página; Lua otimiza caber dentro de outro programa. Python e Ruby são **de propósito geral desde o início** — por isso alcançaram domínios que as outras não.
2. **Modelo de dados = filosofia oposta** — **Lua e PHP unificam radicalmente** (uma estrutura para tudo: minimalismo, mas a estrutura não comunica intenção); **Python distingue deliberadamente** (`list` ≠ `tuple` ≠ `dict` ≠ `set`: o tipo expressa intenção e custo algorítmico); **JavaScript fica no meio** (objeto-mapa universal, daí anomalias como `typeof [] === "object"`).
3. **Onde o código roda** — embutida em aplicação (Lua), no navegador (JS), no servidor Web por requisição (PHP), VM de propósito geral com extensões nativas (Python, Ruby), processo Unix (Perl).

**Por que isso importa:** sob o rótulo comum: **JS é prototípica, Ruby é OO pura, Lua é minimalista e embarcável, Python é multiparadigma e explícita, Perl é orientada a texto, PHP é orientada a requisições**. Seis projetos distintos — por isso nenhum eliminou os outros.

---

## Questão 17 — C# no ambiente .NET

**O que se pergunta:** comparar duas decisões de C# com as correspondentes em Java ou C++, explicando o problema que resolvem.

**Conceito central:** **influência corretiva** — chegar depois, observar o atrito das antecessoras e corrigi-lo.

**Resposta em síntese:**
C# (2000, **Anders Hejlsberg**, Microsoft) é baseada em C++ e Java, com ideias de Delphi e Visual Basic, e tem como propósito o **desenvolvimento baseado em componentes** no framework .NET. Sebesta registra que seus projetistas **discordaram das remoções feitas por Java** em relação a C++: todos aqueles recursos, **exceto a herança múltipla**, foram reincluídos.

**Duas decisões comparadas:**
1. **Unificação de tipos + `struct` como tipo de valor.** *Java* separou primitivos (`int`) de objetos (`Integer`), quebrando a uniformidade — coleções não aceitavam primitivos e cada elemento virava objeto no monte. C# fez **todo tipo derivar de `object`** (com boxing/unboxing) e ofereceu `struct` para tipos de valor. O livro é enfático: o `struct` de C# "foi modificado significativamente, resultando em uma construção verdadeiramente útil, enquanto em C++ tal construção é praticamente inútil". *Problema resolvido:* uniformidade conceitual **e** desempenho de memória.
2. **Delegates e properties.** Em *Java*, um callback exigia interface + classe anônima, e cada campo exigia `getX()`/`setX()` manuais. C# introduziu **delegates** (referências a métodos com tipo verificado, mais seguras que ponteiros de função de C++) e **properties** (sintaxe de campo, semântica de método). *Problema resolvido:* verbosidade cerimonial e a impossibilidade de tratar métodos como valores — o que abriu caminho para eventos, LINQ e lambdas.

*Outras comparações válidas, também no livro:* `enum` mais seguro (nunca convertido implicitamente para inteiro), a sentença **`foreach`**, e a coleta de lixo introduzida na evolução da plataforma.

**Por que isso importa:** o custo dessa correção foi amarrar-se a uma plataforma proprietária — limitação depois desfeita pelo .NET Core. Mesmo padrão da Questão 6: boas ideias precisam de plataforma para se difundir.

---

## Questão 18 — XSLT e JSP: marcação que computa

**O que se pergunta:** diferenciar as duas por entrada, processamento e saída, e explicar por que ambas são híbridas de marcação e programação.

**Conceito central:** a fronteira entre **descrever um documento** e **computar** é permeável — a Web forçou o surgimento dessa categoria intermediária.

**Resposta em síntese:**

| | **XSLT** | **JSP** |
|---|---|---|
| **Entrada** | Documento **XML** + folha de estilo XSLT (que também é XML) | Página **HTML** com código Java embutido + requisição HTTP |
| **Processamento** | **Declarativo por casamento de padrões**: templates casam com nós da árvore (via XPath) e são disparados pelo processador — sem laços explícitos | **Imperativo**: a página é traduzida em um **servlet Java**, compilada e executada no servidor |
| **Saída** | Outro documento (XML, HTML ou texto) — é uma **transformação de documento** | **HTML gerado dinamicamente**, enviado ao navegador |

**Por que ambas são híbridas:** nas duas, o documento é o **esqueleto** e as construções de programação são **inseridas dentro dele**. XSLT expressa seleção, iteração e recursão em sintaxe XML (`<xsl:if>`, `<xsl:for-each>`, `<xsl:template>`) — é uma **linguagem de programação escrita em marcação**, declarativa e Turing-completa. JSP usa HTML como molde com **ilhas de código Java** (`<% ... %>`, JSTL).
**A diferença essencial:** em XSLT a marcação **é** a linguagem; em JSP a marcação **hospeda** a linguagem.

**Por que isso importa:** é o exemplo de que paradigma não é propriedade do rótulo "linguagem de programação" — XSLT é declarativa e completa, ainda que ninguém a chame de linguagem de programação no dia a dia.

---

## Questão 19 — Linha do tempo com tipos de influência

**O que se pergunta:** oito linguagens, ao menos quatro paradigmas, e **o tipo** de cada ligação — proibido usar só setas cronológicas.

**Conceito central:** influência tem **tipos qualitativamente diferentes** — inclusive negativa e retardada.

**Resposta em síntese:**

```
1957  Fortran      [imperativo/numérico]
        │ (a) prova de viabilidade: compilação eficiente é possível
        ▼
1960  ALGOL 60     [imperativo estruturado]
        │ (b) herança com simplificação (via ALGOL-W)
        ├──────────────► 1971 Pascal    [imperativo estruturado / ensino]
        │ (d) extensão de propósito: ALGOL + classes para simulação
        ├──────────────► 1967 SIMULA 67 [orientado a objetos]
        │                      │ (e) radicalização: OO pura, tudo é objeto
        │                      ▼
        │                1980 Smalltalk  [OO puro]
        │                      │ (f) síntese pragmática: objetos + sintaxe/desempenho de C
        │                      ▼
        │                1983 C++ ──(g) influência corretiva──► 1995 Java [OO + VM portátil]
        │
        └──► 1968 ALGOL 68 ──(c) inversão de prioridade (via CPL/BCPL/B)──► 1972 C [sistemas]

1959  Lisp    [funcional] ──(h) fundamentação teórica: cálculo lambda──► Scheme, ML, Haskell
                                e o retorno do estilo funcional a Python/Java/JS

1972  Prolog  [lógico]    ──(i) ruptura de modelo: lógica de predicados no lugar de von Neumann
```

**Os nove tipos de influência (é isto que a questão cobra):**
(a) **prova de viabilidade** — Fortran não deu sintaxe a ALGOL, deu **legitimidade** à compilação;
(b) **herança com simplificação deliberada** — ALGOL 60 → Pascal;
(c) **herança com inversão de prioridade** — ALGOL 68 → C: mantém a estrutura, troca rigor por acesso à máquina;
(d) **extensão de propósito** — ALGOL 60 → SIMULA 67: acrescenta `class` para simular entidades;
(e) **radicalização conceitual** — SIMULA → Smalltalk: de recurso a princípio único;
(f) **síntese pragmática** — Smalltalk + C → C++;
(g) **influência corretiva** — C++ → Java: remove o que causava erro;
(h) **fundamentação teórica** — Lisp: modelo matemático que reaparece décadas depois em outra família;
(i) **ruptura de modelo** — Prolog: não descende de ALGOL, descende da lógica formal.

**Quatro paradigmas cobertos:** imperativo (Fortran, ALGOL, Pascal, C), OO (SIMULA, Smalltalk, C++, Java), funcional (Lisp), lógico (Prolog).

**Por que isso importa:** duas categorias merecem destaque porque quebram a intuição cronológica — a **influência negativa** (a complexidade de ALGOL 68 motivou Wirth a criar Pascal por rejeição) e a **influência retardada** (Lisp voltando ao mainstream décadas depois).

---

## Questão 20 — Estudo de caso: escolher tecnologia com argumento histórico

**O que se pergunta:** propor famílias de linguagens para cálculo científico, regras declarativas, Web interativa e firmware restrito, justificar historicamente e explicitar dois trade-offs.

**Conceito central:** a genealogia **prevê** a adequação — cada família foi **moldada pelo domínio que a originou**.

**Resposta em síntese:**

| Subsistema | Família | Proposta | Justificativa histórica |
|---|---|---|---|
| **Cálculo científico** | Fortran / arrays | Python + NumPy/SciPy sobre núcleo Fortran/C; Julia para código novo | Fortran (1957) nasceu para isso, sob pressão de igualar a montagem manual. **BLAS/LAPACK**, escritas em Fortran, ninguém reescreve — todos **chamam**. O estilo vetorial vem de **APL** (1962) e mapeia para SIMD e GPU |
| **Regras declarativas** | Lógica | Prolog, Datalog ou motor de regras (Drools/CLIPS) | Prolog (1972) foi criado para expressar conhecimento como fatos e regras, delegando a busca ao motor. Regras ficam **declaradas e separadas do controle** — em `if/else`, cada mudança de regra vira alteração de algoritmo |
| **Web interativa** | Scripting Web | TypeScript/JavaScript no cliente; TS/Python/Ruby/C#/Java no servidor | JS (1995) é a **única** linguagem executada nativamente pelo navegador — monopólio de plataforma, não superioridade. TypeScript devolve a disciplina de tipos da linhagem ALGOL→Pascal→Java |
| **Firmware restrito** | Sistemas | C (MISRA) ou Rust `no_std`; **Ada/SPARK** se houver certificação crítica | C (1972) nasceu para escrever o Unix: abstração mínima, custo previsível, **sem GC** (logo sem pausas), compilador para quase todo microcontrolador. Ada (1980) foi projetada por requisitos do DoD para sistemas críticos |

**Trade-off 1 — Poliglotismo × custo de integração.** Quatro famílias maximizam a adequação técnica, mas custam quatro cadeias de build, quatro ecossistemas para contratar e treinar, e — o mais caro — **as fronteiras entre elas**: cada FFI, serialização ou RPC é ponto de falha, de perda de desempenho e de tipos que deixam de ser verificados. Unificar tudo em uma linguagem reduz esse custo, mas condena pelo menos dois subsistemas a ferramentas subótimas.
*Encaminhamento:* limitar a três ecossistemas, com Python como cola entre núcleo científico e motor de regras, e isolar o firmware por **protocolo de mensagens**, não por FFI.

**Trade-off 2 — Segurança/produtividade × controle e determinismo.** GC e tipagem dinâmica (Python, JS) dão velocidade de desenvolvimento e eliminam bugs de memória, ao custo de **latência imprevisível** — inaceitável em firmware. Sem runtime (C), há controle total e determinismo, ao custo de transferir ao programador erros caros (estouro de buffer, ponteiro solto). **Rust** tenta recusar o dilema — segurança de memória sem GC — pagando com curva de aprendizado íngreme e menos mão de obra disponível.
*Encaminhamento:* aceitar GC onde a latência não é crítica; exigir ausência de runtime no firmware, escolhendo Rust só se a equipe absorver o aprendizado, e C+MISRA caso contrário.

**Por que isso importa:** é a síntese da disciplina. Cada escolha é rastreável a uma decisão tomada entre 1957 e 1995 sob uma restrição concreta — **linguagens não são intercambiáveis porque carregam, em sua estrutura, o problema que as originou.**

---

# Mapa final: as ideias que atravessam as 20 questões

| Ideia | Onde aparece | Formulação |
|---|---|---|
| **Domínio molda o projeto** | 5, 7, 9, 13, 16, 20 | Toda decisão de linguagem responde a uma restrição concreta do problema que a originou |
| **Influência ≠ adoção** | 1, 2, 6, 9, 15, 17 | Mérito técnico e sucesso de mercado dependem de fatores diferentes |
| **Abstração precisa ser barata** | 4, 14, 20 | Fortran otimizou para ser aceita; C++ criou "não pague pelo que não usa"; Java precisou do JIT |
| **Simplicidade ≠ regularidade** | 8, 10, 13 | ALGOL 68 e PL/I falharam por excesso; Basic limitou-se por falta |
| **Declarativo × imperativo** | 11, 12, 18, 20 | Descrever *o quê* delega o controle ao motor; descrever *o como* mantém a previsibilidade |
| **Ideias retornam** | 1, 5, 9, 19 | Lisp (1959) e APL (1962) reaparecem em Java 8, NumPy e pandas |

**Os quatro paradigmas e suas origens no capítulo:**

| Paradigma | Origem | Ideia fundadora |
|---|---|---|
| **Imperativo/procedural** | Fortran (1957), ALGOL 60 (1960) | Sequência de comandos que modificam estado; blocos e escopo |
| **Funcional** | Lisp (1959) | Computação como aplicação de funções sobre listas, sem estado mutável |
| **Orientado a objetos** | SIMULA 67 (1967), Smalltalk (1980) | Entidades que encapsulam dados e comportamento e trocam mensagens |
| **Lógico** | Prolog (1972) | Declarar relações verdadeiras e deixar a inferência encontrar as soluções |

---

## Referência

SEBESTA, Robert W. **Conceitos de Linguagens de Programação**. 11. ed. Porto Alegre: Bookman. Capítulo 2 — *Evolução das principais linguagens de programação*, p. 49–118 (numeração do PDF).

**Arquivo complementar:** `exercicios-cap02.md` — respostas desenvolvidas das questões 1, 5, 6, 9, 10, 11, 12, 14, 16 e 20.
