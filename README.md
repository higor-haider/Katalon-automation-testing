# Automation Testing

1. Escolher uma ferramenta para automatizar testes à seguinte Api REST, explica o porquê
    dessa escolha. [Go REST](https://gorest.co.in/)
    Katalon Studio é uma ferramenta de automação de testes completa que suporta testes de API, Web, Mobile e Desktop. Por que escolher Katalon Studio?
    1. Interface Amigável e Intuitiva: Katalon Studio possui uma interface gráfica intuitiva que facilita a criação, execução e manutenção de testes. Mesmo para usuários que não são desenvolvedores, a curva de aprendizado é baixa.
    2. Suporte Abrangente para Testes de API: Criação de Requisições: Facilidade na criação de requisições GET, POST, PUT, DELETE, etc. Validação de Respostas: Ferramentas para validar o conteúdo das respostas, status HTTP, tempo de resposta, entre outros. Scripting Avançado: Possibilidade de adicionar scripts personalizados em Groovy para validações complexas.
    3. Integração com CI/CD: Katalon Studio integra-se facilmente com ferramentas de CI/CD como Jenkins, Azure DevOps, GitLab CI, entre outras. Isso permite a execução automática de testes a cada mudança no código, garantindo qualidade contínua.

2. Explica os use case de teste;
    1. Validação de Schema: Assegurar que a resposta da API segue um esquema JSON específico.
    2. Validação de status completed: Garantir que todos os itens retornados possuem o status completed definido corretamente.
    3. Validação do campo due_on: Checar se o valor do campo due_on é válido e coerente com o formato esperado.

3. Em automação, com resposta desta API gorest.co.in/public/v2/todos
    1. Aplica uma validação de schema ao resultado;
    2. Valida se todos os resultados têm status completed;
    Verificaremos se todos os itens possuem o campo status com o valor completed, caso um item tenha valor diferente de completed o teste irá falhar.
    3. Interpreta e valida o valor “due_on”
    Checaremos se o campo due_on possui um valor de data válido e que segue o formato ISO 8601.

4. DevOps, CI/CD
    1. Explica e justifica uma implementação de testes de carga a esta API;
    Ferramenta: JMeter
    Justificativa: Meter é capaz de simular altas cargas de usuários.
    Configuração Flexível: Permite configuração detalhada de cenários de teste de carga.
    Integrável: Pode ser facilmente integrado com CI/CD usando plugins ou linha de comando.
    2. Como implementarias uma solução de Continuous Testing, justifica;
    Para implementar uma solução de Continuous Testing utilizando Katalon, podemos integrar Katalon com um pipeline de CI/CD, como Jenkins, para executar testes de forma contínua e automatizada.
