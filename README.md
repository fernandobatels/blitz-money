# Blitz Money - Aplicação para controle financeiro pessoal

Inspirada no KMyMoney esta aplicação usa um arquivo de texto para a persistência dos dados e baseia-se na importação de arquivos OFX.

### Funcionalidades

- [ ] Controle de contas bancárias
- [ ] Movimentação financeira
- [ ] Importação de arquivos OFX
- [ ] Integração com o Google Agenda(para notificação sobre os lançamentos agendados)
- [ ] Indentificação padronizada de contatos, contas, categorias... ao importar arquivos OFX

Build this code with:

```bash
rust_qt_binding_generator src/bindings.json
mkdir build
cd build
cmake ..
make
```
