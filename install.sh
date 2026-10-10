#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo "==> [rost-lsp] Building and installing binary..."

# 0. Sincronizar URL de repositorio dinamicamente en README.md si existe remote.origin.url
REPO_URL="$(git config --get remote.origin.url 2>/dev/null || echo "")"
if [ -n "$REPO_URL" ] && [ -f "$SCRIPT_DIR/README.md" ]; then
    DOC_URL="$(echo "$REPO_URL" | sed -E 's|git@github.com:|https://github.com/|')"
    sed -i -E "s|https://github.com/[^/]+/rost-lsp(\.git)?|$DOC_URL|g" "$SCRIPT_DIR/README.md" 2>/dev/null || true
fi

cargo install --path "$SCRIPT_DIR"
echo "  ✓ Binario instalado: $(which rost-lsp || echo "$HOME/.cargo/bin/rost-lsp")"

# Asegurar configuracion de Neovim si no existe
if [ -f "$HOME/.config/nvim/lua/plugins/rost.lua" ]; then
    echo "  ✓ Se encontro configuracion existente en ~/.config/nvim/lua/plugins/rost.lua"
else
    if [ -f "$SCRIPT_DIR/../tree-sitter-rost/install.sh" ]; then
        echo "  → Ejecutando instalador de tree-sitter-rost para registrar configuracion de Neovim..."
        "$SCRIPT_DIR/../tree-sitter-rost/install.sh"
    fi
fi

echo "==> [rost-lsp] Setup completado exitosamente!"
