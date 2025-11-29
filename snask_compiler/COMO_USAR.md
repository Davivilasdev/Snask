# 🚀 Como Usar o Snask v0.2.0

## ⚠️ IMPORTANTE: Versão Correta

Você tem **duas versões** do Snask instaladas:

- ❌ **Versão antiga (0.1.0)**: `C:\Users\Davi\Desktop\codes\Snask\executor\snask.exe`
- ✅ **Versão nova (0.2.0)**: `C:\Users\Davi\Desktop\codes\Snask\snask_compiler\target\release\snask.exe`

Quando você digita apenas `snask`, o Windows usa a versão **0.1.0** (antiga).

---

## ✅ Solução 1: Usar Caminho Completo (Recomendado)

### No diretório do projeto:

```powershell
cd C:\Users\Davi\Desktop\codes\Snask\snask_compiler

# Usar a versão 0.2.0 (nova)
.\target\release\snask.exe --version    # Mostra: snask 0.2.0
.\target\release\snask.exe --help       # Ver comandos
.\target\release\snask.exe interpret arquivo.snask
.\target\release\snask.exe repl         # REPL interativo
```

---

## ✅ Solução 2: Criar Alias (Mais Fácil)

### Adicione ao seu perfil do PowerShell:

```powershell
# Criar alias permanente
notepad $PROFILE

# Adicione esta linha no arquivo:
Set-Alias snask2 "C:\Users\Davi\Desktop\codes\Snask\snask_compiler\target\release\snask.exe"

# Salve e recarregue:
. $PROFILE

# Agora você pode usar:
snask2 --version    # snask 0.2.0
snask2 repl         # REPL funcionando!
```

---

## ✅ Solução 3: Substituir Versão Antiga

### Copiar nova versão para o executor:

```powershell
# Fazer backup da versão antiga
Copy-Item "C:\Users\Davi\Desktop\codes\Snask\executor\snask.exe" "C:\Users\Davi\Desktop\codes\Snask\executor\snask_old.exe"

# Copiar nova versão
Copy-Item "C:\Users\Davi\Desktop\codes\Snask\snask_compiler\target\release\snask.exe" "C:\Users\Davi\Desktop\codes\Snask\executor\snask.exe" -Force

# Agora 'snask' usará a v0.2.0
snask --version     # snask 0.2.0
snask repl          # Funciona!
```

---

## 🎯 Teste Rápido

### Verificar qual versão está usando:

```powershell
# Versão antiga (0.1.0)
snask --version

# Versão nova (0.2.0)
.\target\release\snask.exe --version
```

### Testar REPL (v0.2.0):

```powershell
cd C:\Users\Davi\Desktop\codes\Snask\snask_compiler
.\target\release\snask.exe repl
```

Dentro do REPL:
```
snask> print("Olá, Snask 2.0!")
Olá, Snask 2.0!
snask> sqrt(16)
4
snask> help
[mostra ajuda]
snask> exit
```

### Testar Stdlib (v0.2.0):

```powershell
.\target\release\snask.exe interpret basic_test.snask
```

---

## 📊 Comparação de Versões

| Comando | v0.1.0 (antiga) | v0.2.0 (nova) |
|---------|-----------------|---------------|
| `--version` | snask 0.1.0 | snask 0.2.0 |
| `repl` | ❌ Não existe | ✅ Funciona |
| Funções stdlib | 4 | 70+ |
| Módulos | 1 | 7 |

---

## 🚀 Comandos Disponíveis (v0.2.0)

```bash
# Ver ajuda
.\target\release\snask.exe --help

# Interpretar arquivo
.\target\release\snask.exe interpret arquivo.snask

# REPL interativo (NOVO!)
.\target\release\snask.exe repl
```

---

## 💡 Recomendação

**Use a Solução 3** (substituir versão antiga) para ter acesso fácil à v0.2.0 digitando apenas `snask` em qualquer lugar!

Ou crie um **alias** para não perder a versão antiga e ter ambas disponíveis.
