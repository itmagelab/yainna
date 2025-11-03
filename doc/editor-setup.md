# Настройка редактора для работы с Trunk

> Решение проблемы множественных перезагрузок при изменении файлов

## Проблема

При сохранении файла (особенно `content.yaml`) trunk serve множество раз перезагружает страницу. Это происходит из-за того, что редакторы создают временные файлы и выполняют несколько операций записи при сохранении.

## Решения

### Для Neovim/Vim

Добавьте в ваш `~/.config/nvim/init.lua` (или `init.vim`):

```lua
-- Для init.lua
vim.opt.backup = false          -- Отключить резервные копии
vim.opt.writebackup = false     -- Отключить временные копии при сохранении
vim.opt.swapfile = false        -- Отключить swap файлы

-- Использовать атомарное сохранение (рекомендуется)
vim.opt.backupcopy = "yes"      -- Копировать файл вместо rename
```

Или для `init.vim`:

```vim
set nobackup
set nowritebackup
set noswapfile
set backupcopy=yes
```

**Важно:** `backupcopy=yes` особенно важно - это заставляет vim копировать и перезаписывать файл вместо использования стратегии rename, что вызывает только одно событие изменения файла.

### Для VS Code

Добавьте в `.vscode/settings.json` в корне проекта:

```json
{
  "files.watcherExclude": {
    "**/target/**": true,
    "**/dist/**": true,
    "**/.git/**": true
  },
  "files.exclude": {
    "**/.DS_Store": true,
    "**/Thumbs.db": true
  },
  "files.hotExit": "off",
  "files.autoSave": "off"
}
```

### Для других редакторов

#### Sublime Text

Добавьте в настройки (`Preferences -> Settings`):

```json
{
  "atomic_save": true,
  "create_window_at_startup": false
}
```

#### Emacs

Добавьте в `~/.emacs` или `~/.emacs.d/init.el`:

```elisp
(setq make-backup-files nil)
(setq auto-save-default nil)
(setq create-lockfiles nil)
```

## Настройка проекта

### .trunkignore

Файл `.trunkignore` уже настроен в проекте для игнорирования:
- Временных файлов редакторов (`*~`, `*.swp`, `*.tmp`)
- Системных файлов (`.DS_Store`)
- Резервных копий (`*.bak`, `*.orig`)

### Trunk.toml

В `Trunk.toml` настроен список игнорирования в секции `[watch]`:

```toml
[watch]
ignore = [
    "dist",
    "target",
    ".git",
    ".DS_Store",
    "*~",
    "*.swp",
    "*.swo",
    ".*.sw?",
    "*.tmp",
    "*.bak"
]
```

## Проверка

После настройки редактора:

1. Запустите `trunk serve`
2. Откройте `static/content.yaml`
3. Измените одну строку и сохраните
4. Проверьте терминал - должна быть только одна перезагрузка

## Дополнительные советы

### Если проблема сохраняется

1. **Проверьте, какие файлы создаются:**
   ```bash
   # В другом терминале во время сохранения
   watch -n 0.1 'ls -la static/'
   ```

2. **Посмотрите логи trunk с подробностями:**
   ```bash
   RUST_LOG=debug trunk serve
   ```

3. **Используйте fswatch для мониторинга:**
   ```bash
   # macOS
   brew install fswatch
   fswatch -r static/ | while read file; do echo "Changed: $file"; done
   ```

### Для macOS пользователей

Если используете Time Machine или другие системы резервного копирования:

```bash
# Исключите директории из автоматического резервного копирования
tmutil addexclusion yainna/target
tmutil addexclusion yainna/dist
```

## Альтернативное решение

Если проблема критична и настройка редактора не помогает, можно использовать более агрессивный debounce на уровне файловой системы:

```bash
# Установить watchman (более продвинутая система мониторинга файлов)
brew install watchman  # macOS
# или
apt install watchman   # Linux
```

Затем trunk автоматически использует watchman вместо встроенного механизма отслеживания.

## Рекомендуемая конфигурация (Neovim)

Полная рекомендуемая конфигурация для работы с проектом:

```lua
-- ~/.config/nvim/lua/yainna-project.lua
local M = {}

function M.setup()
  -- Отключить временные файлы
  vim.opt.backup = false
  vim.opt.writebackup = false
  vim.opt.swapfile = false
  
  -- Атомарное сохранение
  vim.opt.backupcopy = "yes"
  
  -- Автокоманда для YAML файлов
  vim.api.nvim_create_autocmd("BufWritePost", {
    pattern = "*.yaml",
    callback = function()
      print("YAML сохранен: " .. vim.fn.expand("%"))
    end,
  })
end

return M
```

Подключите в `init.lua`:

```lua
require('yainna-project').setup()
```

## Итог

После правильной настройки редактора вы должны увидеть:
- ✅ Только одна перезагрузка при сохранении
- ✅ Быстрый отклик trunk serve
- ✅ Нет временных файлов в директории проекта